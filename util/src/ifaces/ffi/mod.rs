#[cfg(target_family = "windows")]
mod windows;
#[cfg(target_family = "windows")]
pub use self::windows::ifaces;

#[cfg(all(target_family = "unix", not(feature = "xtensa")))]
mod unix;
#[cfg(all(target_family = "unix", not(feature = "xtensa")))]
pub use self::unix::ifaces;

#[cfg(feature = "xtensa")]
mod xtensa;
#[cfg(feature = "xtensa")]
pub use self::xtensa::ifaces;
