use std::collections::HashMap;
use std::fmt;
use std::sync::RwLock;
use std::thread::{self, ThreadId};

use backtrace::Backtrace;

lazy_static! {
    static ref PANIC_INFOS: RwLock<HashMap<ThreadId, PanicInfo>> = RwLock::new(HashMap::new());
}

/// A struct providing information about a panic.
///
/// This is basically a owned variant of the `PanicInfo` from the Rust Standard Library.
#[derive(Debug)]
pub struct PanicInfo {
    thread_name: String,
    message: String,
    location: PanicLocation,
    backtrace: Backtrace,
}

impl PanicInfo {
    /// Returns the name of the thread from which the panic originated.
    pub fn thread_name(&self) -> &str {
        self.thread_name.as_str()
    }

    /// Returns the panic message.
    ///
    /// If no panic message string is available then `Box<Any>` will be returned instead.
    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    /// Returns the location from which the panic originated.
    pub fn location(&self) -> &PanicLocation {
        &self.location
    }

    /// Returns the backtrace of this panic.
    ///
    /// The symbols of this backtrace are already resolved.
    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }

    /// Returns a struct that implements Display for this PanicInfo
    /// which includes the full backtrace.
    pub fn full_display(&self) -> FullDisplay {
        FullDisplay { panic_info: self }
    }

    /// Returns a struct that implements Display for this PanicInfo
    /// which includes the backtrace with some frames and other information omitted.
    pub fn short_display(&self) -> ShortDisplay {
        ShortDisplay { panic_info: self }
    }
}

impl<'a> From<&'a std::panic::PanicInfo<'a>> for PanicInfo {
    fn from(panic_info: &'a std::panic::PanicInfo) -> Self {
        let message = match panic_info.payload().downcast_ref::<&'static str>() {
            Some(s) => s.to_string(),
            None => match panic_info.payload().downcast_ref::<String>() {
                Some(s) => s.to_owned(),
                None => String::from("Box<Any>"),
            }
        };

        PanicInfo {
            thread_name: thread::current().name().unwrap().to_string(),
            message,
            location: PanicLocation::from(panic_info.location().unwrap()),
            backtrace: Backtrace::new(),
        }
    }
}

/// Helper struct that implements `Display` for a [`PanicInfo`]
/// which includes the full backtrace.
///
/// [`PanicInfo`]: ./struct.PanicInfo.html
pub struct FullDisplay<'a> {
    panic_info: &'a PanicInfo,
}

impl fmt::Display for FullDisplay<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "thread '{}' panicked at '{}', {}",
            self.panic_info.thread_name, self.panic_info.message, self.panic_info.location)?;
        write!(formatter, "{:?}", self.panic_info.backtrace)?;
        Ok(())
    }
}

/// Helper struct that implements `Display` for a [`PanicInfo`]
/// which includes the backtrace with some frames and other information omitted.
///
/// [`PanicInfo`]: ./struct.PanicInfo.html
pub struct ShortDisplay<'a> {
    panic_info: &'a PanicInfo,
}

impl fmt::Display for ShortDisplay<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "thread '{}' panicked at '{}', {}",
            self.panic_info.thread_name, self.panic_info.message, self.panic_info.location)?;

        write!(formatter, "stack backtrace:")?;
        let frames_iter = self.panic_info.backtrace
            .frames()
            .iter()
            .filter(|frame| frame.ip() != std::ptr::null_mut())
            .filter(|frame| {
                frame.symbols()
                    .iter()
                    .any(|symbol| {
                        if let Some(name) = symbol.name() {
                            !name.to_string().starts_with("backtrace::")
                        } else {
                            false
                        }
                    })
            })
            .take_while(|frame| {
                frame.symbols()
                    .iter()
                    .any(|symbol| {
                        if let Some(name) = symbol.name() {
                            !name.to_string().contains("__rust_begin_short_backtrace")
                        } else {
                            true
                        }
                    })
            })
            .enumerate();

        for (frame_index, frame) in frames_iter {
                write!(formatter, "\n{:4}: ", frame_index)?;

                let symbols = frame.symbols();
                if symbols.len() == 0 {
                    write!(formatter, "<no info>")?;
                    continue;
                }

                for (symbol_index, symbol) in symbols.iter().enumerate() {
                    if symbol_index != 0 {
                        write!(formatter, "\n      ")?;
                    }

                    let write_file_info;
                    if let Some(symbol_name) = symbol.name() {
                        write!(formatter, "{:#}", symbol_name)?;

                        let name_string = symbol_name.to_string();
                        let name = if name_string.starts_with("<") {
                            &name_string[1..]
                        } else {
                            &name_string
                        };

                        write_file_info = !(
                            name.starts_with("std::")
                                || name.starts_with("__rust")
                                || name.starts_with("rust_begin_unwind")
                                || name.starts_with("core::")
                                || name.starts_with("alloc::")
                                || name.starts_with("cuke_runner")
                        );
                    } else {
                        write!(formatter, "<unknown>")?;
                        write_file_info = true;
                    }

                    if write_file_info {
                        if let (Some(file), Some(line)) = (symbol.filename(), symbol.lineno()) {
                            write!(formatter, "\n             at {}:{}", file.display(), line)?;
                        }
                    }
                }
            }

        Ok(())
    }
}

/// A struct containing information about the location of a panic.
///
/// This is basically a owned variant of the panic `Location` from the Rust Standard Library.
#[derive(Debug)]
pub struct PanicLocation {
    file: String,
    line: u32,
    column: u32,
}

impl PanicLocation {
    /// Returns the name of the source file from which the panic originated.
    pub fn file(&self) -> &str {
        self.file.as_str()
    }

    /// Returns the line number from which the panic originated.
    pub fn line(&self) -> u32 {
        self.line
    }

    /// Returns the column from which the panic originated.
    pub fn column(&self) -> u32 {
        self.column
    }
}

impl<'a> From<&'a std::panic::Location<'a>> for PanicLocation {
    fn from(location: &'a std::panic::Location) -> Self {
        PanicLocation {
            file: location.file().to_owned(),
            line: location.line(),
            column: location.column(),
        }
    }
}

impl fmt::Display for PanicLocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}:{}", self.file, self.line, self.column)
    }
}

#[doc(hidden)]
pub fn register_cuke_runner_hook() {
    std::panic::set_hook(Box::new(|std_panic_info| {
        let thread_id = thread::current().id();
        let cuke_panic_info = PanicInfo::from(std_panic_info);

        let panic_in_cuke_runner_test = cuke_panic_info.backtrace.frames()
            .iter()
            .any(|frame| {
                frame.symbols()
                    .iter()
                    .any(|symbol| {
                        if let Some(symbol_name) = symbol.name() {
                            symbol_name.to_string().starts_with(
                                "cuke_runner::runtime::step_definition::StepDefinition::execute")
                        } else {
                            false
                        }
                    })
            });
        if panic_in_cuke_runner_test {
            PANIC_INFOS.write().unwrap().insert(thread_id, cuke_panic_info);
        } else {
            eprintln!("{}", cuke_panic_info.short_display());
        }
    }));
}

#[doc(hidden)]
pub fn remove_current_panic_info() -> Option<PanicInfo> {
    let thread_id = thread::current().id();
    PANIC_INFOS.write().unwrap().remove(&thread_id)
}
