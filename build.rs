use std::env;
use std::path::PathBuf;
use std::process::Command;
use winres;

fn main() {
    let lib_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("lib");
    let config_dir = lib_dir.join("config");
    let yacd_dir = lib_dir.join("yacd");
    for path in &[
        "src",
        "package.json",
        "package-lock.json",
        "babel.config.js",
        "vue.config.js",
    ] {
        println!("cargo:rerun-if-changed={}", config_dir.join(path).display());
    }
    for path in &[
        "src",
        "package.json",
        "package-lock.json",
        "babel.config.js",
        "webpack.config.js",
    ] {
        println!("cargo:rerun-if-changed={}", yacd_dir.join(path).display());
    }
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon_with_id("src/icon.ico", "icon");
        res.compile().unwrap();

        assert!(Command::new("cmd")
            .args(&["/c", "npm", "install"])
            .current_dir(&yacd_dir)
            .status()
            .map(|status| status.success())
            .unwrap_or(false));
        assert!(Command::new("cmd")
            .args(&["/c", "npm", "run", "build"])
            .current_dir(yacd_dir)
            .status()
            .map(|status| status.success())
            .unwrap_or(false));

        assert!(Command::new("cmd")
            .args(&["/c", "npm", "install"])
            .current_dir(&config_dir)
            .status()
            .map(|status| status.success())
            .unwrap_or(false));
        assert!(Command::new("cmd")
            .args(&["/c", "npm", "run", "build"])
            .current_dir(config_dir)
            .status()
            .map(|status| status.success())
            .unwrap_or(false));
    }
}
