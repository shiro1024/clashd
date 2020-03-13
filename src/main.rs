// #![windows_subsystem = "windows"]
#![feature(proc_macro_hygiene)]

mod utils;

use std::env::current_dir;
use std::process::{Child, Command, Stdio};
use std::thread::spawn;
use utils::*;
use web_view::{builder, Content};

pub type WindowResult = Result<(), exitfailure::ExitFailure>;

fn get_current_port() -> Option<u16> {
    use serde_yaml::{from_slice, Value};
    use std::collections::BTreeMap;
    use std::net::SocketAddr;
    read_full_file("config.yaml")
        .ok()
        .and_then(|s| from_slice::<BTreeMap<String, Value>>(&s).ok())
        .and_then(|map| {
            if let Some(Value::String(addr)) = map.get("external-controller") {
                addr.parse::<SocketAddr>().map(|ip| ip.port()).ok()
            } else {
                None
            }
        })
}

fn get_current_content() -> String {
    flate!(static DIST_CONTENT: str from "lib/yacd/public/index.html");
    lazy_static! {
        static ref CONTENT: Arc<RwLock<String>> = Arc::new(RwLock::new(DIST_CONTENT.to_string()));
        static ref PORT: Arc<RwLock<u16>> = Arc::new(RwLock::new(7892));
    }
    if let Some(port) = get_current_port() {
        if port != *PORT.read().unwrap() {
            let new_content = CONTENT.read().unwrap().replace(
                &format!("\"{}\"", PORT.read().unwrap()),
                &format!("\"{}\"", port),
            );
            *PORT.write().unwrap() = port;
            *CONTENT.write().unwrap() = new_content;
        }
    }
    CONTENT.read().unwrap().clone()
}

fn run_tray(mut process: Child) -> WindowResult {
    let running_browser = Arc::new(RwLock::new(false));
    let running_browser_inner = running_browser.clone();
    let mut app = systray::Application::new()?;

    app.set_icon_from_resource("icon")?;
    app.add_menu_item("Open Dashboard", move |_| {
        if !*running_browser_inner.read().unwrap() {
            *running_browser_inner.write().unwrap() = true;
            let running_browser_thread = running_browser_inner.clone();
            spawn(move || {
                if let Err(e) = builder()
                    .title("Clash Dashboard")
                    .content(Content::Html(get_current_content()))
                    .size(960, 540)
                    .resizable(false)
                    .debug(true)
                    .user_data(())
                    .invoke_handler(|_webview, _arg| Ok(()))
                    .build()
                    .and_then(|mut webview| loop {
                        if *running_browser_thread.read().unwrap() {
                            match webview.step() {
                                Some(Ok(_)) => (),
                                Some(e) => e?,
                                None => {
                                    *running_browser_thread.write().unwrap() = false;
                                    return Ok(webview.into_inner());
                                }
                            }
                        } else {
                            return Ok(webview.into_inner());
                        }
                    })
                {
                    msgbox(&format!("Fail to init webview: {}", e));
                }
            });
        }
        Ok::<_, systray::Error>(())
    })?;
    app.add_menu_item("Quit", move |window| {
        *running_browser.write().unwrap() = false;
        window.quit();
        if let Err(e) = process.kill() {
            msgbox(&format!("Fail to kill clash process: {}", e));
        }
        Ok::<_, systray::Error>(())
    })?;
    app.wait_for_message()?;
    Ok(())
}

fn main() -> WindowResult {
    if Command::new("checknetisolation")
        .args(&[
            "LoopbackExempt",
            "-a",
            "-n=Microsoft.Win32WebViewHost_cw5n1h2txyewy",
        ])
        .stdout(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
    {
        match Command::new("clash")
            .args(&[
                "-d",
                current_dir()
                    .unwrap_or(".".into())
                    .to_string_lossy()
                    .as_ref(),
            ])
            .spawn()
        {
            Ok(child) => run_tray(child)?,
            Err(e) => msgbox(&format!("Fail to run clash: {}", e)),
        }
    } else {
        msgbox("Fail to disable loopback access restrictions");
    }

    Ok(())
}
