//! Interfaces expected of the x86 host.

use wasm_bindgen::prelude::wasm_bindgen;

/// DirectDraw surface.
pub trait Surface {
    /// Write RGBA pixel data directly.
    /// Used for copying an image to the surface via GDI calls, and for Lock/Unlock pixel writes.
    fn write_pixels(&mut self, pixels: &[[u8; 4]]);

    /// Get the back Surface from a primary Surface.
    fn get_attached(&self) -> Box<dyn Surface>;

    /// Show the back Surface on a primary Surface.
    fn flip(&mut self);

    // TODO: the trait object here means we end up needing to cast, but the alternative
    // isn't object safe, bleh.
    fn bit_blt(&mut self, dx: u32, dy: u32, src: &dyn Surface, sx: u32, sy: u32, w: u32, h: u32);
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct SurfaceOptions {
    pub width: u32,
    pub height: u32,
    pub primary: bool,
}
impl Default for SurfaceOptions {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            primary: false,
        }
    }
}

/// Floating window.
pub trait Window {
    fn set_title(&mut self, title: &str);
    fn set_size(&mut self, width: u32, height: u32);
}

pub trait File {
    fn seek(&mut self, ofs: u32) -> bool;
    fn read(&mut self, buf: &mut [u8], len: &mut u32) -> bool;
}

pub trait Host {
    fn exit(&mut self, code: u32);
    fn time(&self) -> u32;

    fn open(&self, path: &str) -> Box<dyn File>;
    fn write(&self, buf: &[u8]) -> usize;

    fn create_window(&mut self) -> Box<dyn Window>;
    fn create_surface(&mut self, opts: &SurfaceOptions) -> Box<dyn Surface>;
}