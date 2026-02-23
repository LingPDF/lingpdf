fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("resources/windows/icon.ico");
        res.compile().unwrap();
    }

    #[cfg(not(windows))]
    println!("cargo:rerun-if-changed=resources/windows/icon.ico");
}
