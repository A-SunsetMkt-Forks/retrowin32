//! Code generator for winapi functions.
//! Generates builtin.rs and win32/dll/*.

mod dll;
mod generate;
mod parse;

use std::path::Path;

use walkdir::WalkDir;

fn write_if_changed(path: &Path, contents: &[u8]) -> anyhow::Result<()> {
    if let Ok(old_contents) = std::fs::read(path) {
        if old_contents == contents {
            return Ok(());
        }
    }
    std::fs::write(path, contents)?;
    Ok(())
}

/// Parse a directory's collection of files.
fn parse_files(root: &Path) -> anyhow::Result<Vec<(String, syn::File)>> {
    let prefix = root.parent().unwrap();
    let mut files = Vec::new();
    for entry in WalkDir::new(root).sort_by_file_name() {
        let entry = entry?;
        if entry.file_type().is_dir() {
            continue;
        }
        let path = entry.path();
        let buf = std::fs::read_to_string(path)?;
        let file = syn::parse_file(&buf)?;
        let mut trace_name_path = path.strip_prefix(prefix).unwrap().with_extension("");
        if trace_name_path.ends_with("mod") {
            trace_name_path.pop();
        }
        files.push((trace_name_path.to_string_lossy().into_owned(), file));
    }
    Ok(files)
}

fn rustfmt(tokens: &str) -> anyhow::Result<String> {
    use std::io::Write;
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

/// Assign ordinals to all fns that don't have them already.
fn assign_ordinals(fns: &mut [parse::DllExport]) -> anyhow::Result<()> {
    let mut used_ordinals = std::collections::HashSet::new();
    for dllexport in fns.iter_mut() {
        if let Some(ordinal) = dllexport.meta.ordinal {
            if !used_ordinals.insert(ordinal) {
                return Err(syn::Error::new_spanned(dllexport.func, "duplicate ordinal").into());
            }
        }
    }

    let mut ordinal = 1;
    for dllexport in fns {
        if dllexport.meta.ordinal.is_none() {
            while used_ordinals.contains(&ordinal) {
                ordinal += 1;
            }
            dllexport.meta.ordinal = Some(ordinal);
            ordinal += 1;
        }
    }
    Ok(())
}

fn process_builtin_dll(path: &Path, dll_dir: &Path) -> anyhow::Result<()> {
    let module_name = path.file_stem().unwrap().to_string_lossy();
    eprintln!("{}.dll", module_name);

    let files = parse_files(path)?;
    let mut dllexports = parse::DllExports::default();
    for (name, file) in &files {
        parse::gather_dllexports(name, &file.items, &mut dllexports)?;
    }

    // Sort by name, then assign ordinals satisfying the ordinals that were specified,
    // then sort by ordinal to ensure the output is deterministic.
    dllexports.fns.sort_by(|a, b| a.flat_name.cmp(&b.flat_name));
    assign_ordinals(&mut dllexports.fns).unwrap();
    dllexports.fns.sort_by_key(|e| e.meta.ordinal.unwrap());

    dll::generate_dll(&module_name, &dllexports, |name, content| {
        write_if_changed(&dll_dir.join(name), &content)
    })?;

    let builtins_module = generate::shims_module(&module_name, dllexports);
    let text = rustfmt(&builtins_module.to_string())?;
    write_if_changed(&path.join("builtin.rs"), text.as_bytes())?;

    Ok(())
}

#[derive(argh::FromArgs)]
/// dllexport generator
struct Args {
    /// dir to write asm files
    #[argh(option)]
    dll_dir: String,

    /// files to process
    #[argh(positional)]
    inputs: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();

    for path in &args.inputs {
        process_builtin_dll(path.as_ref(), args.dll_dir.as_ref())
            .map_err(|err| anyhow::anyhow!("processing module: {}", err))?;
    }

    Ok(())
}
