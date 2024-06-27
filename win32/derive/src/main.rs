//! Code generator for winapi functions.
//! Generates functions that pop arguments off the x86 stack.

use std::io::Write;

use proc_macro2::TokenStream;
use quote::quote;
mod gen;

/// Generate one module's shim functions.
fn generate_shims_module(
    module_name: &str,
    dllexports: Vec<(&syn::ItemFn, gen::DllExport)>,
) -> anyhow::Result<TokenStream> {
    let module = quote::format_ident!("{}", module_name);
    let dll_name = format!("{}.dll", module_name);

    let mut impls = Vec::new();
    let mut shims = Vec::new();
    let mut exports = Vec::new();
    for (func, dllexport) in dllexports {
        let (wrapper, shim) = gen::fn_wrapper(quote! { winapi::#module }, func, dllexport.callconv);
        impls.push(wrapper);
        shims.push(shim);

        let ordinal = match dllexport.ordinal {
            Some(ord) => quote!(Some(#ord)),
            None => quote!(None),
        };
        let name = &func.sig.ident;
        exports.push(quote!(Symbol { ordinal: #ordinal, shim: shims::#name }));
    }

    let exports_count = exports.len();

    Ok(quote! {
        pub mod #module {
            use super::*;

            mod impls {
                use memory::Extensions;
                use crate::{machine::Machine, winapi::{self, stack_args::*, types::*}};
                use winapi::#module::*;
                #(#impls)*
            }

            mod shims {
                use crate::shims::Shim;
                use super::impls;
                #(#shims)*
            }

            const EXPORTS: [Symbol; #exports_count] = [
                #(#exports),*
            ];

            pub const DLL: BuiltinDLL = BuiltinDLL {
                file_name: #dll_name,
                exports: &EXPORTS,
            };
        }
    })
}

/// Parse a single .rs file or a directory's collection of files.
fn parse_files(path: &std::path::Path) -> anyhow::Result<Vec<syn::File>> {
    // path may be a .rs file or a directory (module).
    let mut paths: Vec<std::path::PathBuf> = if path.extension().is_none() {
        std::fs::read_dir(path)?
            .map(|e| e.unwrap().path())
            .collect()
    } else {
        vec![path.to_path_buf()]
    };
    paths.sort();

    let mut files = Vec::new();
    for path in paths {
        let buf = std::fs::read_to_string(&path)?;
        let file = syn::parse_file(&buf)?;
        files.push(file);
    }
    Ok(files)
}

fn generate_builtins_module(mods: Vec<TokenStream>) -> anyhow::Result<Vec<u8>> {
    let out = quote! {
        /// Generated code, do not edit.

        use crate::shims;

        pub struct Symbol {
            pub ordinal: Option<usize>,
            pub shim: shims::Shim,
        }

        pub struct BuiltinDLL {
            pub file_name: &'static str,
            pub exports: &'static [Symbol],
        }

        #(#mods)*
    };

    // Verify output parses correctly.
    if let Err(err) = syn::parse2::<syn::File>(out.clone()) {
        anyhow::bail!("parsing macro-generated code: {}", err);
    };

    let mut buf = Vec::new();
    // parse2 seems to fail if it sees these, so dump them here.
    write!(&mut buf, "#![allow(non_snake_case)]\n").unwrap();
    write!(&mut buf, "#![allow(non_upper_case_globals)]\n").unwrap();
    write!(&mut buf, "#![allow(unused_imports)]\n").unwrap();
    write!(&mut buf, "#![allow(unused_variables)]\n").unwrap();
    let text = rustfmt(&out.to_string())?;
    buf.extend_from_slice(text.as_bytes());

    Ok(buf)
}

fn rustfmt(tokens: &str) -> anyhow::Result<String> {
    // Stolen from https://github.com/microsoft/windows-rs/blob/master/crates/tools/lib/src/lib.rs
    let mut child = std::process::Command::new("rustfmt")
        .arg("--edition")
        .arg("2018")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
    let mut stdin = child.stdin.take().ok_or(anyhow::anyhow!("no stdin"))?;
    stdin.write_all(tokens.as_bytes())?;
    drop(stdin);
    let output = child.wait_with_output()?;

    if !output.status.success() {
        anyhow::bail!("rustfmt failed: {}", std::str::from_utf8(&output.stderr)?);
    }
    Ok(String::from_utf8(output.stdout)?)
}

fn process_builtin_dll(path: &std::path::Path) -> anyhow::Result<TokenStream> {
    let module_name = path.file_stem().unwrap().to_string_lossy();
    eprintln!("{}.dll", module_name);

    let files = parse_files(path)?;
    let mut dllexports = Vec::new();
    for file in &files {
        gen::gather_dllexports(&file.items, &mut dllexports)?;
    }
    dllexports.sort_by_key(|(func, _)| &func.sig.ident);
    generate_shims_module(&module_name, dllexports)
}

#[derive(argh::FromArgs)]
/// dllexport generator
struct Args {
    /// output path
    #[argh(option)]
    builtins: String,

    /// files to process
    #[argh(positional)]
    inputs: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();

    let mut mods = Vec::new();
    for path in &args.inputs {
        let path = std::path::Path::new(path);
        let gen = match process_builtin_dll(&path) {
            Ok(gen) => gen,
            Err(err) => anyhow::bail!("processing module: {}", err),
        };
        mods.push(gen);
    }

    let builtins_module = generate_builtins_module(mods)?;
    std::fs::write(&args.builtins, &builtins_module)?;

    Ok(())
}
