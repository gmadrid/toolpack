use std::sync::Mutex;

static INSTANCE: once_cell::sync::Lazy<Mutex<Spew>> = once_cell::sync::Lazy::new(|| {
    Mutex::new(Spew {
        level: SpewLevel::STANDARD,
    })
});

#[macro_export]
macro_rules! quiet {
    ($($arg:tt)+) => ($crate::spew_at_level($crate::SpewLevel::QUIET, format_args!($($arg)+)));
}

#[macro_export]
macro_rules! spew {
    ($($arg:tt)+) => ($crate::spew_at_level($crate::SpewLevel::STANDARD, format_args!($($arg)+)));
}

#[macro_export]
macro_rules! verbose {
    ($($arg:tt)+) => ($crate::spew_at_level($crate::SpewLevel::VERBOSE, format_args!($($arg)+)));
}

pub fn spew_at_level(level: SpewLevel, fa: std::fmt::Arguments) {
    INSTANCE
        .lock()
        .unwrap()
        .spew_at_level(level, std::fmt::format(fa))
}

pub fn set_level(level: SpewLevel) {
    INSTANCE.lock().unwrap().set_level(level)
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum SpewLevel {
    QUIET,
    STANDARD,
    VERBOSE,
}

struct Spew {
    level: SpewLevel,
}

impl Spew {
    fn set_level(&mut self, level: SpewLevel) {
        self.level = level
    }

    fn spew_at_level(&self, level: SpewLevel, s: impl AsRef<str>) {
        if level <= self.level {
            println!("{}", s.as_ref());
        }
    }
}
