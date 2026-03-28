#[cfg(unix)]
mod unix;

#[cfg(unix)]
#[allow(unused_imports)]
pub use unix::*;

#[cfg(not(any(feature = "safaos", unix)))]
compile_warning!("Unsupported platform, features requiring user identification won't work!");

#[cfg(not(unix))]
mod stubs;

#[cfg(not(unix))]
#[allow(unused_imports)]
pub use unix::*;
