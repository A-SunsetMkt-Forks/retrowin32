use std::{cell::RefCell, rc::Rc};

#[cfg(feature = "sdl")]
extern crate sdl2;

pub struct GUI {
    video: sdl2::VideoSubsystem,
    pump: sdl2::EventPump,
    win: Option<WindowRef>,
}
impl GUI {
    pub fn new() -> anyhow::Result<Self> {
        let sdl = sdl2::init().map_err(|err| anyhow::anyhow!(err))?;
        let video = sdl.video().map_err(|err| anyhow::anyhow!(err))?;
        let pump = sdl.event_pump().map_err(|err| anyhow::anyhow!(err))?;

        Ok(GUI {
            video,
            pump: pump,
            win: None,
        })
    }

    pub fn pump_messages(&mut self) -> bool {
        for event in self.pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => return false,
                _ => {}
            }
            println!("ev {:?}", event);
        }
        true
    }

    pub fn create_window(&mut self) -> Box<dyn win32::Window> {
        let win = Window::new(&self.video);
        let win_ref = WindowRef(Rc::new(RefCell::new(win)));
        self.win = Some(win_ref.clone());
        Box::new(win_ref)
    }

    pub fn create_surface(&mut self, opts: &win32::SurfaceOptions) -> Box<dyn win32::Surface> {
        if opts.primary {
            Box::new(Surface::Window(self.win.as_ref().unwrap().clone()))
        } else {
            Box::new(Surface::Texture(Texture::new(
                self.win.as_ref().unwrap(),
                opts,
            )))
        }
    }
}

struct Window {
    canvas: sdl2::render::WindowCanvas,
}
impl Window {
    fn new(video: &sdl2::VideoSubsystem) -> Self {
        let win = video.window("retrowin32", 640, 480).build().unwrap();
        let canvas = win.into_canvas().build().unwrap();
        Window { canvas }
    }
}

#[derive(Clone)]
struct WindowRef(Rc<RefCell<Window>>);
impl win32::Window for WindowRef {
    fn set_title(&mut self, title: &str) {
        self.0
            .borrow_mut()
            .canvas
            .window_mut()
            .set_title(title)
            .unwrap();
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.0
            .borrow_mut()
            .canvas
            .window_mut()
            .set_size(width, height)
            .unwrap();
    }
}

enum Surface {
    Window(WindowRef),
    Texture(Texture),
}
impl win32::Surface for Surface {
    fn write_pixels(&mut self, pixels: &[[u8; 4]]) {
        match self {
            Surface::Window(_) => unimplemented!(),
            Surface::Texture(t) => t.write_pixels(pixels),
        }
    }

    fn get_attached(&self) -> Box<dyn win32::Surface> {
        match self {
            Surface::Window(w) => Box::new(Surface::Window(w.clone())),
            Surface::Texture(_) => unimplemented!(),
        }
    }

    fn flip(&mut self) {
        match self {
            Surface::Window(w) => w.0.borrow_mut().canvas.present(),
            Surface::Texture(_) => unimplemented!(),
        }
    }

    fn bit_blt(
        &mut self,
        dx: u32,
        dy: u32,
        src: &dyn win32::Surface,
        sx: u32,
        sy: u32,
        w: u32,
        h: u32,
    ) {
        let src_rect = sdl2::rect::Rect::new(sx as i32, sy as i32, w, h);
        let src = unsafe { &*(src as *const dyn win32::Surface as *const Surface) };
        let tex = match src {
            Surface::Window(_) => unimplemented!(),
            Surface::Texture(t) => &t.texture,
        };
        let dst_rect = sdl2::rect::Rect::new(dx as i32, dy as i32, w, h);
        match self {
            Surface::Window(wr) => {
                wr.0.borrow_mut()
                    .canvas
                    .copy(tex, src_rect, dst_rect)
                    .unwrap()
            }
            Surface::Texture(_) => unimplemented!(),
        };
    }
}

struct Texture {
    texture: sdl2::render::Texture,
    width: u32,
    height: u32,
}
impl Texture {
    fn new(win: &WindowRef, opts: &win32::SurfaceOptions) -> Self {
        let texture = win
            .0
            .borrow()
            .canvas
            .texture_creator()
            .create_texture_static(
                sdl2::pixels::PixelFormatEnum::ABGR8888,
                opts.width,
                opts.height,
            )
            .unwrap();
        Texture {
            texture,
            width: opts.width,
            height: opts.height,
        }
    }

    fn write_pixels(&mut self, pixels: &[[u8; 4]]) {
        let pixels_u8 =
            unsafe { std::slice::from_raw_parts(pixels.as_ptr() as *const u8, pixels.len() * 4) };
        let rect = sdl2::rect::Rect::new(0, 0, self.width, self.height);
        self.texture
            .update(rect, pixels_u8, self.width as usize * 4)
            .unwrap();
    }
}