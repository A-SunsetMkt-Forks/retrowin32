//! A system for enabling tracing of different subsystems of winapi.
//!
//! Each win32_derive::dllexport function can trace its incoming args
//! and return value.  The user can specify which functions to trace.
//! See 'Tracing' in HACKING.md.

use std::{cell::UnsafeCell, collections::HashMap, fmt::Write};

#[derive(Debug)]
struct Rule {
    key: String,
    enabled: bool,
}

struct State {
    rules: Vec<Rule>,
    enabled: HashMap<*const u8, bool>,
    include_return: bool,
}

impl State {
    fn new(scheme: &str) -> Self {
        let mut rules = Vec::new();
        let mut include_return = true;
        let scheme = if let Some(s) = scheme.strip_prefix("^") {
            include_return = false;
            s
        } else {
            scheme
        };
        for mut part in scheme.split(',') {
            if part.len() == 0 {
                continue;
            }
            if part == "*" {
                part = ""
            }
            let enabled = if part.starts_with('-') {
                part = &part[1..];
                false
            } else {
                true
            };
            rules.push(Rule {
                key: part.into(),
                enabled,
            });
        }
        State {
            rules,
            enabled: HashMap::new(),
            include_return,
        }
    }

    fn lookup(&mut self, context: &'static str) -> bool {
        // Confusing: for a static 'foo', foo.as_ptr() has different values
        // when referenced from different mods (e.g. from ddraw's various mods),
        // but only in Debug builds.
        // This code still works in any case.
        if let Some(&enabled) = self.enabled.get(&context.as_ptr()) {
            return enabled;
        }
        let mut enabled = false;
        for rule in &self.rules {
            if context.starts_with(&rule.key) {
                enabled = rule.enabled;
                // Don't break, so last match wins.
            }
        }
        self.enabled.insert(context.as_ptr(), enabled);
        return enabled;
    }
}

static mut STATE: UnsafeCell<Option<State>> = UnsafeCell::new(None);

#[allow(static_mut_refs)]
pub fn set_scheme(scheme: &str) {
    unsafe { *STATE.get_mut() = Some(State::new(scheme)) };
}

#[inline(never)]
#[allow(static_mut_refs)]
pub fn enabled(context: &'static str) -> bool {
    unsafe {
        match STATE.get_mut() {
            None => return false,
            Some(state) => state.lookup(context),
        }
    }
}

#[allow(static_mut_refs)]
fn include_return() -> bool {
    unsafe { STATE.get_mut().as_mut().unwrap().include_return }
}

/// Where to send trace output; defaults to the logger.
static mut OUTPUT: Option<fn(&Record)> = Some(Record::to_logger);

pub fn set_output(output: fn(&Record)) {
    unsafe { OUTPUT = Some(output) };
}

pub struct Record {
    pub file: &'static str,
    pub line: u32,
    pub context: &'static str,
    pub msg: String,
}

impl Record {
    pub fn new(
        pos: (&'static str, u32),
        context: &'static str,
        func: &str,
        args: &[(&str, &dyn std::fmt::Debug)],
    ) -> Record {
        let mut msg = format!("{}(", func);
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                msg.push_str(", ");
            }
            write!(&mut msg, "{}:{:x?}", arg.0, arg.1).unwrap();
        }
        msg.push_str(")");

        let (file, line) = pos;
        Record {
            file,
            line,
            context,
            msg,
        }
    }

    #[allow(static_mut_refs)]
    pub fn enter(self) -> Option<Record> {
        if !include_return() {
            unsafe { OUTPUT.as_ref().unwrap()(&self) };
            return None;
        }
        Some(self)
    }

    #[allow(static_mut_refs)]
    pub fn exit(mut self, result: &dyn std::fmt::Debug) {
        if !include_return() {
            return;
        }
        write!(self.msg, " -> {:x?}", result).ok();
        unsafe { OUTPUT.as_ref().unwrap()(&self) };
    }

    fn to_logger(&self) {
        log::logger().log(
            &log::Record::builder()
                .level(log::Level::Info)
                .file(Some(self.file))
                .line(Some(self.line))
                .args(format_args!("{}/{}", self.context, self.msg))
                .build(),
        );
    }
}
