fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");
    // Embed the exe icon + version info when targeting Windows.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("ProductName", "FromSoft Trophy Helper");
        res.set("FileDescription", "FromSoft Trophy Helper");
        res.compile().expect("failed to compile Windows resources");
    }
}
