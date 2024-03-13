#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::{
    str16::expect_ascii,
    winapi::{types::DWORD, ImportSymbol},
};
use memory::Extensions;

// http://sandsprite.com/CodeStuff/Understanding_imports.html
//
// Code calling a DLL looks like:
//   call [XXX]
// where XXX is the address of an entry in the IAT:
//   IAT: [
//      AAA,
//      BBB,  <- XXX might point here
//   ]
// On load, each IAT entry points to the function name (as parsed below).
// The loader overwrites the IAT with the addresses to the loaded DLL.

/// Import Directory Table (section 6.4.1)
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct IMAGE_IMPORT_DESCRIPTOR {
    /// ILT
    OriginalFirstThunk: DWORD,
    TimeDateStamp: DWORD,
    ForwarderChain: DWORD,
    Name: DWORD,
    /// IAT
    FirstThunk: DWORD,
}
unsafe impl memory::Pod for IMAGE_IMPORT_DESCRIPTOR {}

impl IMAGE_IMPORT_DESCRIPTOR {
    pub fn image_name<'m>(&self, image: &'m [u8]) -> &'m str {
        expect_ascii(image.slicez(self.Name))
    }

    pub fn ilt<'m>(&self, image: &'m [u8]) -> impl Iterator<Item = ILTEntry> + 'm {
        // Officially OriginalFirstThunk (ILT) should have all the data, but in one
        // executable they're all 0, possibly a Borland compiler thing.
        // Meanwhile, win2k's msvcrt.dll has invalid FirstThunk (IAT) data...
        let addr = if self.OriginalFirstThunk != 0 {
            self.OriginalFirstThunk
        } else {
            self.FirstThunk
        };

        // Import Lookup Table (section 6.4.2)
        image[addr as usize..]
            .into_iter_pod::<ILTEntry>()
            .take_while(|entry| entry.0 != 0)
    }

    pub fn iat_offset(&self) -> u32 {
        self.FirstThunk
    }
}

pub fn read_imports<'m>(buf: &'m [u8]) -> impl Iterator<Item = IMAGE_IMPORT_DESCRIPTOR> + 'm {
    buf.into_iter_pod::<IMAGE_IMPORT_DESCRIPTOR>()
        .take_while(|desc| desc.Name != 0)
}

#[repr(transparent)]
#[derive(Clone)]
pub struct ILTEntry(u32);
unsafe impl memory::Pod for ILTEntry {}

impl ILTEntry {
    pub fn as_import_symbol(self, image: &[u8]) -> ImportSymbol {
        let entry = self.0;
        if entry & (1 << 31) != 0 {
            let ordinal = entry & 0xFFFF;
            ImportSymbol::Ordinal(ordinal)
        } else {
            // First two bytes at offset are hint/name table index, used to look up
            // the name faster in the DLL; we just skip them.
            let sym_name = expect_ascii(image.slicez(entry + 2));
            ImportSymbol::Name(sym_name)
        }
    }
}
