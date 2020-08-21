pub mod os {
    #[cfg(windows)]
    pub mod windows;
    #[cfg(windows)]
    pub use windows as target;
}

