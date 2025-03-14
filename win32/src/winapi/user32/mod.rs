#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod builtin;
mod dialog;
mod menu;
mod message;
mod misc;
mod paint;
mod rect;
mod resource;
mod timer;
mod window;
mod wndclass;

use std::{cell::RefCell, rc::Rc};

pub use builtin::DLL;

use super::{handle::Handles, HWND};

pub use super::{gdi32::HDC, kernel32::ResourceKey};
pub use dialog::*;
pub use menu::*;
pub use message::*;
pub use misc::*;
pub use paint::*;
pub use rect::*;
pub use resource::*;
pub use timer::*;
pub use window::*;
pub use wndclass::*;

#[derive(Default)]
pub struct State {
    /// Window classes, kept in an array so we can look them up by name.
    // These generally don't change, but SetWindowLong lets you poke at most of their fields,
    // so RefCell it is.
    wndclasses: Vec<Rc<RefCell<WndClass>>>,

    /// Count of user-registered message ids, see RegisterWindowMessageA.
    pub user_window_message_count: u32,

    pub windows: Handles<HWND, Rc<RefCell<Window>>>,
    active_window: HWND,

    messages: MessageQueue,
    timers: Timers,
}
