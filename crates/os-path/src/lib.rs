pub mod backend;
pub mod path;
pub mod path_buf;

#[cfg(test)]
mod tests;

pub use backend::{Component, PathBackend, Posix, Win32};
pub use path::PlatformPath;
pub use path_buf::PlatformPathBuf;

/// Native platform backend (Win32 on Windows, Posix on other)
#[cfg(windows)]
pub type Native = Win32;
#[cfg(not(windows))]
pub type Native = Posix;

/// Native platform path — same as `path` module default
pub type OsPath = PlatformPath<Native>;
pub type OsPathBuf = PlatformPathBuf<Native>;

/// Explicit Win32 path (backslash, drive letters, UNC)
pub type Win32Path = PlatformPath<Win32>;
pub type Win32PathBuf = PlatformPathBuf<Win32>;

/// Explicit POSIX path (forward slash, no drive letters)
pub type PosixPath = PlatformPath<Posix>;
pub type PosixPathBuf = PlatformPathBuf<Posix>;
