//! Embeds the optional novafoods-fonts build input (`NOVAFOODS_FONTS_DIR`): both `fonts/manifest.txt` and the
//! opt-in Latin presentation fonts in `fonts/latin-manifest.txt` (Inter, Roboto, …). Unset or missing,
//! `NOVAFOODS_FONTS` is empty and the app uses system fonts. Recipe: novafoods-fonts `docs/integration.md`.
//! Noto Sans CJK only with `NOVAFIG_EMBED_CJK` set (the web build's only way to show CJK UI text).
use std::fmt::Write as _;
use std::path::PathBuf;

fn main() {
    println!("cargo::rerun-if-env-changed=NOVAFOODS_FONTS_DIR");
    println!("cargo::rerun-if-env-changed=NOVAFOODS_FONTS_REQUIRED");
    println!("cargo::rerun-if-env-changed=NOVAFIG_EMBED_CJK");
    let mut src = String::from("pub static NOVAFOODS_FONTS: &[CraftFont] = &[\n");
    if let Some(dir) = std::env::var_os("NOVAFOODS_FONTS_DIR").map(PathBuf::from) {
        let mut ok = false;
        for manifest in ["fonts/latin-manifest.txt", "fonts/manifest.txt"] {
            match novafoods_fonts(&dir, manifest) {
                Ok(entries) => {
                    ok = true;
                    src.push_str(&entries);
                }
                Err(e) if std::env::var_os("NOVAFOODS_FONTS_REQUIRED").is_some() => {
                    println!("cargo::error=NOVAFOODS_FONTS_DIR={}: {e}", dir.display());
                }
                Err(e) => println!("cargo::warning=novafoods-fonts {manifest} skipped: NOVAFOODS_FONTS_DIR={}: {e}", dir.display()),
            }
        }
        if !ok {
            println!("cargo::warning=building without novafoods-fonts");
        }
    }
    src.push_str("];\n");
    let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap_or_default()).join("novafoods_fonts.rs");
    if let Err(e) = std::fs::write(&out, src) {
        println!("cargo::error=writing {}: {e}", out.display());
    }
}

fn novafoods_fonts(dir: &std::path::Path, manifest_rel: &str) -> Result<String, String> {
    let manifest = dir.join(manifest_rel);
    println!("cargo::rerun-if-changed={}", manifest.display());
    let text = std::fs::read_to_string(&manifest).map_err(|e| format!("{}: {e}", manifest.display()))?;
    let wasm = std::env::var("CARGO_CFG_TARGET_ARCH").is_ok_and(|a| a == "wasm32");
    let mut out = String::new();
    for line in text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let f: Vec<&str> = line.split(" | ").map(str::trim).collect();
        let [family, style, file, scripts, ..] = f.as_slice() else {
            return Err(format!("malformed manifest line: {line}"));
        };
        // CJK fonts are large; the desktop app finds system CJK fonts instead unless asked.
        let cjk = scripts.contains("Hans");
        if cjk && std::env::var_os("NOVAFIG_EMBED_CJK").is_none() {
            continue;
        }
        // Web builds have a size budget: Inter, plus Noto Sans CJK (every CJK script, for CJK UI
        // text) when asked.
        if wasm && *family != "Inter" && !cjk {
            continue;
        }
        let path = dir.join(file).canonicalize().map_err(|e| format!("{file}: {e}"))?;
        println!("cargo::rerun-if-changed={}", path.display());
        let _ = writeln!(out, "    CraftFont {{ family: {family:?}, style: {style:?}, bytes: include_bytes!({:?}) }},", path.display().to_string());
    }
    Ok(out)
}
