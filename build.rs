fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set("ProductName",    "Calculator-NG");
        res.set("FileDescription","Mini Calculator");
        res.set("CompanyName",    "Galago-labs");
        res.set("LegalCopyright", "© 2026 Galago-labs");
        res.set_version_info(winresource::VersionInfo::PRODUCTVERSION, 0x0001_0000_0000_0000);
        res.set_version_info(winresource::VersionInfo::FILEVERSION,    0x0001_0000_0000_0000);
        // Hide console window
        println!("cargo:rustc-link-arg=/SUBSYSTEM:WINDOWS");
        println!("cargo:rustc-link-arg=/ENTRY:mainCRTStartup");
        res.compile().expect("Failed to compile Windows resources");
    }
}
