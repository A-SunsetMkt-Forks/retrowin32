//! DirectDraw shared API.  All the ddraw1 through ddraw7 interfaces back onto shared
//! implementation defined here.

use super::palette::Palette;
pub use super::types::*;
pub use crate::winapi::com::GUID;
use crate::{
    host,
    machine::Machine,
    winapi::{
        bitmap::transmute_pixels_mut,
        ddraw::{ddraw1, ddraw7},
        heap::Heap,
        types::{HWND, RECT},
    },
    SurfaceOptions,
};
use memory::{Extensions, Mem};
use std::collections::HashMap;

pub struct Surface {
    pub host: Box<dyn host::Surface>,
    pub width: u32,
    pub height: u32,
    pub palette: Option<Palette>,
    /// x86 address to pixel buffer, or 0 if unused.
    pub pixels: u32,
    pub bytes_per_pixel: u32,
    /// Address of attached surface, e.g. back buffer.
    pub attached: u32,
}

impl Surface {
    fn new(machine: &mut Machine, hwnd: HWND, opts: &SurfaceOptions) -> Self {
        if opts.width == 0 || opts.height == 0 {
            panic!("cannot create 0-sized surface");
        }
        Surface {
            host: machine.host.create_surface(hwnd.to_raw(), &opts),
            width: opts.width,
            height: opts.height,
            palette: None,
            pixels: 0,
            bytes_per_pixel: opts.bytes_per_pixel,
            attached: 0,
        }
    }

    pub fn create(machine: &mut Machine, hwnd: HWND, desc: &DDSURFACEDESC2) -> Vec<Surface> {
        assert!(std::mem::size_of::<DDSURFACEDESC2>() == desc.dwSize as usize);

        let mut surfaces = Vec::new();

        let mut opts = crate::host::SurfaceOptions::default();
        if desc.dwFlags.contains(DDSD::WIDTH) {
            opts.width = desc.dwWidth;
        }
        if desc.dwFlags.contains(DDSD::HEIGHT) {
            opts.height = desc.dwHeight;
        }

        if let Some(caps) = desc.caps() {
            if caps.dwCaps.contains(DDSCAPS::PRIMARYSURFACE) {
                opts.primary = true;
            }
        }

        if opts.width == 0 || opts.height == 0 {
            // Take width/height from window dimensions
            if let Some(wnd) = machine.state.user32.windows.get(hwnd) {
                opts.width = wnd.width;
                opts.height = wnd.height;
            }
        }

        if opts.bytes_per_pixel == 0 {
            opts.bytes_per_pixel = machine.state.ddraw.bytes_per_pixel;
        }

        surfaces.push(Surface::new(machine, hwnd, &opts));

        if let Some(count) = desc.back_buffer_count() {
            opts.primary = false;
            for _ in 0..count {
                surfaces.push(Surface::new(machine, hwnd, &opts));
            }
        }

        surfaces
    }

    pub fn to_rect(&self) -> RECT {
        RECT {
            left: 0,
            top: 0,
            right: self.width as i32,
            bottom: self.height as i32,
        }
    }

    pub fn flush(&mut self, mem: Mem, palette: Option<&Palette>) {
        assert!(self.pixels != 0);

        // We need to copy self.pixels to convert its format to the RGBA expected by the write_pixels API.
        let mut pixels_bytes = Vec::with_capacity((self.width * self.height * 4) as usize);
        unsafe { pixels_bytes.set_len(pixels_bytes.capacity()) };
        let pixels_quads: &mut [[u8; 4]] = transmute_pixels_mut(&mut pixels_bytes);
        match self.bytes_per_pixel {
            1 => {
                let pixels = mem.iter_pod::<u8>(self.pixels, self.width * self.height);
                let Some(palette) = palette else {
                    // On startup possibly no palette, which means leave black (?).
                    return;
                };
                let palette = palette.borrow();
                for (pSrc, pDst) in pixels.zip(pixels_quads) {
                    let p = &palette[pSrc as usize];
                    *pDst = [p.peRed, p.peGreen, p.peBlue, 0xFF];
                }
            }
            2 => {
                let pixels = mem.iter_pod::<u16>(self.pixels, self.width * self.height);

                for (pSrc, pDst) in pixels.zip(pixels_quads) {
                    // TODO: this depends on whether 16bpp is 555 or 565 etc.
                    let r = ((pSrc & 0b0111_1100_0000_0000) >> 7) as u8;
                    let g = ((pSrc & 0b0000_0011_1110_0000) >> 2) as u8;
                    let b = ((pSrc & 0b0000_0000_0001_1111) << 3) as u8;
                    *pDst = [r, g, b, 0xFF];
                }
            }
            4 => {
                let pixels = mem.iter_pod::<[u8; 4]>(self.pixels, self.width * self.height);
                // Ignore alpha channel in input; output is always opaque.
                for (pSrc, pDst) in pixels.zip(pixels_quads) {
                    *pDst = [pSrc[0], pSrc[1], pSrc[2], 0xFF];
                }
            }
            bpp => todo!("Unlock for {bpp}bpp"),
        }
        self.host.write_pixels(&pixels_bytes);

        // If surface is primary then updates should show immediately.
        // XXX probably need something other than attached here
        if self.attached == 0 {
            self.host.show();
        }
    }
}

pub struct State {
    pub heap: Heap,

    // TODO: this is per-IDirectDraw state.
    pub hwnd: HWND,
    pub surfaces: HashMap<u32, Surface>,

    pub bytes_per_pixel: u32,

    pub palettes: HashMap<u32, Palette>,
    /// XXX monolife attaches palette only to back surface, then flips; we need to rearrange
    /// how surface flipping works for the palettes to work out, so this is hacked for now.
    pub palette_hack: Option<Palette>,
}

impl State {
    pub fn new_init(machine: &mut Machine) -> Self {
        let mut ddraw = State::default();
        ddraw.heap = machine.state.kernel32.new_private_heap(
            &mut machine.emu.memory,
            4 << 20,
            "ddraw.dll heap".into(),
        );
        ddraw
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            heap: Heap::default(),
            hwnd: HWND::null(),
            surfaces: HashMap::new(),
            bytes_per_pixel: 4,
            palettes: HashMap::new(),
            palette_hack: Default::default(),
        }
    }
}

#[win32_derive::dllexport]
pub fn DirectDrawCreate(
    machine: &mut Machine,
    lpGuid: Option<&GUID>,
    lplpDD: Option<&mut u32>,
    pUnkOuter: u32,
) -> DD {
    DirectDrawCreateEx(machine, lpGuid, lplpDD, None, pUnkOuter)
}

#[win32_derive::dllexport]
pub fn DirectDrawCreateEx(
    machine: &mut Machine,
    lpGuid: Option<&GUID>,
    lplpDD: Option<&mut u32>,
    iid: Option<&GUID>,
    pUnkOuter: u32,
) -> DD {
    assert!(lpGuid.is_none());
    assert!(pUnkOuter == 0);

    if machine.state.ddraw.heap.addr == 0 {
        machine.state.ddraw = State::new_init(machine);
    }

    match iid {
        None => {
            // DirectDrawCreate
            *lplpDD.unwrap() = ddraw1::IDirectDraw::new(machine);
            return DD::OK;
        }
        Some(&ddraw7::IID_IDirectDraw7) => {
            *lplpDD.unwrap() = ddraw7::IDirectDraw7::new(machine);
            DD::OK
        }
        _ => {
            log::error!("DirectDrawCreateEx: unknown IID {iid:x?}");
            DD::ERR_GENERIC
        }
    }
}
