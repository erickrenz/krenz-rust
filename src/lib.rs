use color_eyre::Result;

/// Install the deault panic and error report hooks. Ensure rust backtrace is enabled.
///
/// # Examples
/// ```
/// use color_eyre::eyre::Result;
///
/// fn main() -> Result<()> {
///     krenz::color_eyre_install()?;
///     // ...
///     Ok(())
/// }
/// ```
pub fn color_eyre_install() -> Result<()> {
    // Install color_eyre exactly once
    if std::env::var("RUST_COLOR_EYRE_INSTALL").is_err() {
        std::env::set_var("RUST_COLOR_EYRE_INSTALL", "1");
        color_eyre::install()?;
    }

    // Enable backtraces, if not already enabled by user
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    Ok(())
}

/// Debug print variables and line numbers with fully customizeable formatting.
///
/// # Examples
///
/// ```
/// use krenz::dump;
/// let address = 2;
/// let value = 3;
/// dump!("address = {}  value = 0b{:08b}", address, value);
/// ```
#[macro_export]
macro_rules! dump {
    () => {
        eprintln!("[{}:{}]", file!(), line!());
    };
    ($($arg:tt)*) => {
        eprintln!("[{}:{}] {}", file!(), line!(), format!($($arg)*));
    };
    ($($val:expr),+$(,)?) => {
        ($($crate::dump!($val)),+,)
    };
}

/// Debug print variables and line numbers with a simple, default single line format.
///
/// # Examples
///
/// ```
/// use krenz::dmp;
/// let address = 2;
/// let value = 3;
/// dmp!(address, value);
/// ```
#[macro_export]
macro_rules! dmp {
    ($($a:expr),*) => {
        eprintln!(concat!("[", file!(), ":", line!(), "] ", $(stringify!($a), " = {:?}  "),*), $($a),*);
    }
}
