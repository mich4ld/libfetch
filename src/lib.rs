pub mod platform;
pub mod utils;
pub mod shared;

// LINUX:
#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub type Info = linux::Linux;

// ANDROID:
#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "android")]
pub type Info = android::Android;
