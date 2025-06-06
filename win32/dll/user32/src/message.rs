use crate::{get_state, timer::Timers, window::Window};
use bitflags::bitflags;
use memory::Extensions;
use std::{cell::RefCell, ops::RangeInclusive, rc::Rc};
use win32_system::{System, Wait, host};
use win32_winapi::*;

#[repr(C)]
#[derive(Clone)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: u32,
    pub lParam: u32,
    pub time: u32,
    // TODO: struct POINT
    pub pt_x: u32,
    pub pt_y: u32,
    // The docs mention an lPrivate field, but old versions of MSG don't have the lPrivate field.
    // We don't want to accidentally overwrite adjacent data when putting a MSG into memory, so
    // we leave it out.  See commit 30d1bb3ea9c955b82a724f36490dbe0914af5355.
}
unsafe impl memory::Pod for MSG {}

// Manually implement Debug so we decode the WM_FOO value.
impl std::fmt::Debug for MSG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wm = WM::try_from(self.message);
        f.debug_struct("MSG")
            .field("hwnd", &self.hwnd)
            .field("message", &wm)
            .field("wParam", &self.wParam)
            .field("lParam", &self.lParam)
            .field("time", &self.time)
            .field("pt_x", &self.pt_x)
            .field("pt_y", &self.pt_y)
            .finish()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, win32_derive::TryFromEnum)]
pub enum WM {
    NULL = 0,
    CREATE = 0x0001,
    MOVE = 0x0003,
    SIZE = 0x0005,
    ACTIVATE = 0x0006,
    SETFOCUS = 0x0007,
    SETTEXT = 0x000C,
    PAINT = 0x000F,
    CLOSE = 0x0010,
    QUIT = 0x0012,
    ACTIVATEAPP = 0x001C,
    SETFONT = 0x0030,
    WINDOWPOSCHANGED = 0x0047,
    SETICON = 0x0080,
    KEYDOWN = 0x0100,
    KEYUP = 0x0101,
    CHAR = 0x0102,
    TIMER = 0x0113,
    MOUSEMOVE = 0x0200,
    LBUTTONDOWN = 0x0201,
    LBUTTONUP = 0x0202,
    LBUTTONDBLCLK = 0x0203,
    RBUTTONDOWN = 0x0204,
    RBUTTONUP = 0x0205,
    RBUTTONDBLCLK = 0x0206,
    MBUTTONDOWN = 0x0207,
    MBUTTONUP = 0x0208,
    MBUTTONDBLCLK = 0x0209,
    USER = 0x0400,
}

fn msg_from_message(message: host::Message) -> MSG {
    let mut msg = MSG {
        hwnd: HWND::from_raw(message.hwnd),
        message: WM::QUIT as u32, // will be overwritten
        wParam: 0,
        lParam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };

    match &message.detail {
        host::MessageDetail::Quit => {
            msg.message = WM::QUIT as u32;
        }
        host::MessageDetail::Mouse(mouse) => {
            msg.message = match (mouse.button, mouse.down) {
                (host::MouseButton::None, _) => WM::MOUSEMOVE,
                (host::MouseButton::Left, true) => WM::LBUTTONDOWN,
                (host::MouseButton::Left, false) => WM::LBUTTONUP,
                (host::MouseButton::Right, true) => WM::RBUTTONDOWN,
                (host::MouseButton::Right, false) => WM::RBUTTONUP,
                (host::MouseButton::Middle, true) => WM::MBUTTONDOWN,
                (host::MouseButton::Middle, false) => WM::MBUTTONUP,
            } as u32;
            msg.wParam = 0; // TODO:  modifiers
            msg.lParam = (mouse.y << 16) | mouse.x;
            msg.pt_x = mouse.x;
            msg.pt_y = mouse.y;
        }
    }

    msg
}

/// A Windows message queue.
/// At a high level just a queue of MSG, but there are particulars around painting and timers.
/// https://learn.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues
/// The basic approach is that there are a few get_* methods that each can either peek or remove
/// the requested message.  Internally, paint/timers are never actually inserted into a queue
/// but the caller doesn't need to be aware.
///
/// TODO: should be per-thread.
/// TODO: this generally doesn't support multiple HWNDs either,
/// and will need to be revisited to make that work.
#[derive(Default)]
pub struct MessageQueue {
    msgs: std::collections::VecDeque<MSG>,
}

impl MessageQueue {
    fn push(&mut self, msg: MSG) {
        self.msgs.push_back(msg);
    }

    /// Get any queued message matching the filter criteria.
    fn get_queued(
        &mut self,
        hwnd: HWND,
        filter: &RangeInclusive<u32>,
        remove: bool,
    ) -> Option<MSG> {
        let pos = self.msgs.iter().position(|msg| {
            if !hwnd.is_null() && (!msg.hwnd.is_null() && msg.hwnd != hwnd) {
                return false;
            }
            filter.contains(&msg.message)
        })?;
        if remove {
            self.msgs.remove(pos)
        } else {
            Some(self.msgs[pos].clone())
        }
    }

    /// Get any pending WM_PAINT matching the filter criteria.
    fn get_paint(
        &mut self,
        hwnd: HWND,
        windows: &Handles<HWND, Rc<RefCell<Window>>>,
        _remove: bool,
    ) -> Option<MSG> {
        // Note: remove is intentionally ignored because we never enqueue a WM_PAINT.
        // This just accepts the flag to match the other get_* fns.
        let hwnd = if hwnd.is_null() {
            windows.iter().find(|(_, w)| w.borrow().is_dirty())?.0
        } else {
            if !windows.get(hwnd).unwrap().borrow().is_dirty() {
                return None;
            }
            hwnd
        };
        Some(MSG {
            hwnd,
            message: WM::PAINT as u32,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt_x: 0,
            pt_y: 0,
        })
    }

    /// Get any pending WM_TIMER matching the filter criteria.
    /// Err(wait) returns indicate how long to block until the next timer.
    fn get_timer(
        &mut self,
        host: &dyn host::Host,
        timers: &mut Timers,
        remove: bool,
    ) -> Result<MSG, Option<u32>> {
        if timers.is_empty() {
            // No pending timers.
            return Err(None);
        }

        let now = host.ticks();
        // TODO: support filtering by HWND.
        if let Some(timer) = timers.find_next(HWND::null(), now) {
            if !remove {
                todo!("how does peeking timers work");
            }
            return Ok(timer.generate_wm_timer(now));
        }

        let soonest = timers.soonest();
        Err(Some(soonest))
    }
}

/// Retrieves the next available message without blocking.
/// Returns Err(wait) if we need to wait.
fn poll_message(
    sys: &mut dyn System,
    hwnd: HWND,
    filter: Option<RangeInclusive<u32>>,
    remove: bool,
) -> Result<MSG, Option<u32>> {
    let filter = filter.unwrap_or(0..=0xFFFF_FFFF);
    let mut state = get_state(sys);
    if let Some(msg) = state.messages.get_queued(hwnd, &filter, remove) {
        return Ok(msg);
    }

    // TODO: obey filter here.
    if let Some(msg) = sys.host().get_message() {
        let msg = msg_from_message(msg);
        if !remove {
            state.messages.push(msg.clone());
        }
        return Ok(msg);
    }

    let state = &mut *state;
    if filter.contains(&(WM::PAINT as u32)) {
        if let Some(msg) = state.messages.get_paint(hwnd, &state.windows, remove) {
            return Ok(msg);
        }
    }

    if filter.contains(&(WM::TIMER as u32)) {
        return state
            .messages
            .get_timer(sys.host(), &mut state.timers, remove);
    } else {
        Err(None) // block
    }
}

async fn await_message(sys: &mut dyn System, wait: Option<u32>) {
    sys.block(wait).await;
}

bitflags! {
    #[derive(Debug, win32_derive::TryFromBitflags)]
    pub struct RemoveMsg: u32 {
        const PM_NOREMOVE = 0x0000;
        const PM_REMOVE = 0x0001;
        const PM_NOYIELD = 0x0002;
    }
}

#[win32_derive::dllexport]
pub fn PeekMessageA(
    sys: &mut dyn System,
    lpMsg: Option<&mut MSG>,
    hWnd: HWND,
    wMsgFilterMin: u32,
    wMsgFilterMax: u32,
    wRemoveMsg: Result<RemoveMsg, u32>,
) -> bool {
    let lpMsg = lpMsg.unwrap();

    let filter = if wMsgFilterMin == 0 && wMsgFilterMax == 0 {
        None
    } else {
        Some(wMsgFilterMin..=wMsgFilterMax)
    };
    let remove = wRemoveMsg.unwrap().contains(RemoveMsg::PM_REMOVE);

    if let Ok(msg) = poll_message(sys, hWnd, filter, remove) {
        *lpMsg = msg;
        return true;
    }
    false
}

#[win32_derive::dllexport]
pub fn PeekMessageW(
    sys: &mut dyn System,
    lpMsg: Option<&mut MSG>,
    hWnd: HWND,
    wMsgFilterMin: u32,
    wMsgFilterMax: u32,
    wRemoveMsg: Result<RemoveMsg, u32>,
) -> bool {
    PeekMessageA(sys, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax, wRemoveMsg)
}

async fn get_message(
    sys: &mut dyn System,
    lpMsg: Option<&mut MSG>,
    hWnd: HWND,
    wMsgFilterMin: u32,
    wMsgFilterMax: u32,
) -> i32 {
    let msg: MSG;
    loop {
        let filter = if wMsgFilterMin == 0 && wMsgFilterMax == 0 {
            None
        } else {
            Some(wMsgFilterMin..=wMsgFilterMax)
        };
        match poll_message(sys, hWnd, filter, true) {
            Ok(m) => {
                msg = m;
                break;
            }
            Err(wait_until) => await_message(sys, wait_until).await,
        }
    }

    let quit = msg.message == WM::QUIT as u32;
    *lpMsg.unwrap() = msg;
    if quit {
        return 0;
    }
    1
}

// Note: the docs say this returns BOOL, but really it can return -1/0/nonzero.
#[win32_derive::dllexport]
pub async fn GetMessageA(
    sys: &mut dyn System,
    lpMsg: Option<&mut MSG>,
    hWnd: HWND,
    wMsgFilterMin: u32,
    wMsgFilterMax: u32,
) -> i32 {
    get_message(sys, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax).await
}

// Note: the docs say this returns BOOL, but really it can return -1/0/nonzero.
#[win32_derive::dllexport]
pub async fn GetMessageW(
    sys: &mut dyn System,
    lpMsg: Option<&mut MSG>,
    hWnd: HWND,
    wMsgFilterMin: u32,
    wMsgFilterMax: u32,
) -> i32 {
    get_message(sys, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax).await
}

#[win32_derive::dllexport]
pub async fn WaitMessage(sys: &mut dyn System) -> bool {
    loop {
        match poll_message(sys, HWND::null(), None, false) {
            Ok(_) => break,
            Err(wait_until) => await_message(sys, wait_until).await,
        }
    }
    true
}

bitflags::bitflags! {
    /// GetQueueStatus flags, also used in MsgWaitForMultipleObjects.
    #[derive(Debug, win32_derive::TryFromBitflags)]
    pub struct QS: u32 {
        const KEY = 0x0001;
        const MOUSEMOVE = 0x0002;
        const MOUSEBUTTON = 0x0004;
        const POSTMESSAGE = 0x0008;
        const TIMER = 0x0010;
        const PAINT = 0x0020;
        const SENDMESSAGE = 0x0040;
        const HOTKEY = 0x0080;
        const ALLPOSTMESSAGE = 0x0100;

        // const RAWINPUT = 0x0400;
        // const TOUCH = 0x0800;
        // const POINTER = 0x1000;
    }
}

#[win32_derive::dllexport]
pub fn GetQueueStatus(sys: &dyn System, flags: Result<QS, u32>) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn TranslateMessage(sys: &dyn System, lpMsg: Option<&MSG>) -> bool {
    // TODO: translate key-related messages into enqueuing a WM_CHAR.
    false // no message translated
}

pub async fn dispatch_message(sys: &mut dyn System, msg: &MSG) -> u32 {
    assert!(!msg.hwnd.is_null());
    let wndproc = get_state(sys)
        .windows
        .get(msg.hwnd)
        .unwrap()
        .borrow()
        .wndclass
        .borrow()
        .wndproc;
    if wndproc == 0 {
        log::error!("window has no wndproc, skipping message dispatch");
        return 0;
    }
    // TODO: SetWindowLong can change the wndproc.
    sys.call_x86(
        wndproc,
        vec![
            msg.hwnd.to_raw(),
            msg.message as u32,
            msg.wParam,
            msg.lParam,
        ],
    )
    .await;
    // TODO: copy eax to return value
    0
}

#[win32_derive::dllexport]
pub async fn DispatchMessageA(sys: &mut dyn System, lpMsg: Option<&MSG>) -> u32 {
    let msg = lpMsg.unwrap();
    if msg.hwnd.is_null() {
        // No associated hwnd.
        return 0;
    }
    dispatch_message(sys, msg).await;
    0
}

#[win32_derive::dllexport]
pub async fn DispatchMessageW(sys: &mut dyn System, lpMsg: Option<&MSG>) -> u32 {
    let msg = lpMsg.unwrap();
    if msg.hwnd.is_null() {
        // No associated hwnd.
        return 0;
    }
    dispatch_message(sys, msg).await;
    0
}

#[win32_derive::dllexport]
pub fn PostQuitMessage(sys: &mut dyn System, nExitCode: i32) {
    get_state(sys).messages.push(MSG {
        hwnd: HWND::null(),
        message: WM::QUIT as u32,
        wParam: 0,
        lParam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    });
}

#[win32_derive::dllexport]
pub fn PostMessageW(sys: &mut dyn System, hWnd: HWND, Msg: u32, wParam: u32, lParam: u32) -> bool {
    get_state(sys).messages.push(MSG {
        hwnd: hWnd,
        message: Msg,
        wParam,
        lParam,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    });
    true
}

#[win32_derive::dllexport]
pub fn TranslateAcceleratorW(
    sys: &dyn System,
    hWnd: HWND,
    hAccTable: u32,
    lpMsg: Option<&MSG>,
) -> u32 {
    0 // success
}

#[win32_derive::dllexport]
pub async fn SendMessageA(
    sys: &mut dyn System,
    hWnd: HWND,
    Msg: Result<WM, u32>,
    wParam: u32,
    lParam: u32,
) -> u32 {
    let msg = MSG {
        hwnd: hWnd,
        message: Msg.unwrap() as u32,
        wParam,
        lParam,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };
    dispatch_message(sys, &msg).await
}

#[win32_derive::dllexport]
pub async fn SendMessageW(
    sys: &mut dyn System,
    hWnd: HWND,
    Msg: Result<WM, u32>,
    wParam: u32,
    lParam: u32,
) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub async fn MsgWaitForMultipleObjects(
    sys: &mut dyn System,
    nCount: u32,
    pHandles: u32,
    fWaitAll: bool,
    dwMilliseconds: u32,
    dwWakeMask: Result<QS, u32>,
) -> u32 {
    let objects = sys
        .mem()
        .iter_pod::<HANDLE<()>>(pHandles, nCount)
        .collect::<Vec<_>>();
    let mask = dwWakeMask.unwrap();
    if !mask.is_empty() {
        log::warn!("MsgWaitForMultipleObjects: ignoring dwWakeMask");
        // TODO: e.g. handles.push(msgqueueevent)
    }

    sys.wait_for_objects(
        &objects,
        fWaitAll,
        Wait::from_millis(sys.host(), dwMilliseconds),
    )
    .await
    .to_code()
}

#[win32_derive::dllexport]
pub fn CallWindowProcA(
    sys: &dyn System,
    lpPrevWndFunc: u32, /* WNDPROC */
    hWnd: HWND,
    Msg: u32,
    wParam: u32,
    lParam: u32,
) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn PostMessageA(sys: &mut dyn System, hWnd: HWND, Msg: u32, wParam: u32, lParam: u32) -> bool {
    if hWnd.is_null() {
        let thread_id = sys.get_thread_id();
        return PostThreadMessageA(sys, thread_id, Msg, wParam, lParam);
    }
    todo!()
}

#[win32_derive::dllexport]
pub fn PostThreadMessageA(
    sys: &mut dyn System,
    idThread: u32,
    Msg: u32,
    wParam: u32,
    lParam: u32,
) -> bool {
    let thread_id = sys.get_thread_id();
    if idThread != thread_id {
        // TODO: per-thread queues
        todo!();
    }

    get_state(sys).messages.push(MSG {
        hwnd: HWND::null(),
        message: Msg,
        wParam,
        lParam,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    });
    true
}

#[win32_derive::dllexport]
pub fn SendDlgItemMessageA(
    sys: &dyn System,
    hDlg: HWND,
    nIDDlgItem: i32,
    Msg: u32,
    wParam: u32,
    lParam: u32,
) -> u32 {
    todo!()
}
