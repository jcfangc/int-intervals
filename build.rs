use std::process::Command;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(rustc_1_96)");

    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("failed to run rustc --version");
    let version = String::from_utf8_lossy(&output.stdout);

    // "rustc 1.96.0 (..." → extract major.minor
    if let Some(ver) = version
        .strip_prefix("rustc ")
        .and_then(|s| s.split(' ').next())
    {
        let parts: Vec<&str> = ver.split('.').collect();
        if parts.len() >= 2 {
            let major: u32 = parts[0].parse().unwrap_or(0);
            let minor: u32 = parts[1].parse().unwrap_or(0);
            if major > 1 || (major == 1 && minor >= 96) {
                println!("cargo:rustc-cfg=rustc_1_96");
            }
        }
    }
}
