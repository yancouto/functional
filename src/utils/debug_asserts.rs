use std::backtrace::Backtrace;

pub trait DebugUnwrap {
    /// Only on debug mode, panics if the value can't be unwraped.
    /// On production builds, only log.
    fn debug_unwrap(self);
    /// Same as debug_unwrap, but with custom text.
    fn debug_expect(self, text: &str);
}

pub trait DebugUnwrapOrDefault<T: Default> {
    /// On debug mode, panic if the value can't be unwraped.
    /// On production builds, return default value.
    fn debug_unwrap_or_default(self) -> T;
}

impl<T> DebugUnwrap for Option<T> {
    fn debug_unwrap(self) { self.debug_expect("Optional has unexpected None value!"); }

    fn debug_expect(self, text: &str) {
        if cfg!(debug_assertions) {
            if self.is_none() {
                println!("{}", Backtrace::force_capture());
            }
            self.expect(text);
        } else {
            if self.is_none() {
                log::error!("{}", text);
            }
        }
    }
}

impl<T: Default, E: std::fmt::Debug> DebugUnwrapOrDefault<T> for Result<T, E> {
    fn debug_unwrap_or_default(self) -> T {
        if cfg!(debug_assertions) {
            self.unwrap()
        } else {
            self.unwrap_or_default()
        }
    }
}

impl<T, E: std::fmt::Debug> DebugUnwrap for Result<T, E> {
    fn debug_unwrap(self) { self.debug_expect("Result has unexpected error"); }

    fn debug_expect(self, text: &str) {
        if cfg!(debug_assertions) {
            if self.is_err() {
                println!("{}", Backtrace::force_capture());
            }
            self.expect(text);
        } else {
            if let Err(err) = self {
                log::error!("{}: {:?}", text, err);
            }
        }
    }
}
