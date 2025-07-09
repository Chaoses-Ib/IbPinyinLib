fn main() {
    #[cfg(windows)]
    {
        // Windows SDK is required
        let res = winres::WindowsResource::new();
        // Meta information (like program version and description) is taken from `Cargo.toml`'s `[package]` section.
        res.compile().unwrap();
    }
}
