//! Windows: embed the icon and version info in novafig.exe (no-op elsewhere).
fn main() {
    println!("cargo::rerun-if-changed=../../assets/app-icon/novafig.ico");
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "windows" {
        let mut res = winresource::WindowsResource::new();
        let ico = "../../assets/app-icon/novafig.ico";
        if std::path::Path::new(ico).exists() {
            res.set_icon(ico);
        }
        res.set("ProductName", "Nova Fig");
        res.set("FileDescription", "Nova Fig");
        res.set("CompanyName", "Nova Foods");
        if let Err(e) = res.compile() {
            println!("cargo::warning=winresource: {e}");
        }
    }
    let sha = std::env::var("NOVAFIG_BUILD_SHA").unwrap_or_default();
    println!("cargo::rustc-env=NOVAFIG_BUILD_SHA={sha}");
    println!("cargo::rerun-if-env-changed=NOVAFIG_BUILD_SHA");
}
