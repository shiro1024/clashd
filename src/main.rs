// #![windows_subsystem = "windows"]
#![feature(proc_macro_hygiene)]

mod utils;

use std::env::current_dir;
use std::process::{Child, Command, Stdio};
use std::thread::spawn;
use systray::Application;
use utils::*;
use web_view::{builder, Content, WVResult, WebView};

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

fn get_config_content() -> String {
    flate!(static CONTENT: str from "lib/config/dist/index.html");
    CONTENT.clone()
}

fn get_dashboard_content() -> String {
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

fn run_webview<'a, S, C, T, I, L>(
    title: S,
    atomic: Arc<RwLock<bool>>,
    get_content: C,
    handler: I,
    state: L,
) -> impl FnMut(&mut Application) -> Result<(), systray::Error>
where
    S: AsRef<str>,
    C: Fn() -> Content<T> + Copy + Send + 'static,
    T: AsRef<str>,
    I: FnMut(&mut WebView<L>, &str) -> WVResult + Send + Copy + 'static,
    L: Send + Copy + 'static,
{
    move |_| {
        let title = format!("Clash {}", title.as_ref());
        if !*atomic.read().unwrap() {
            *atomic.write().unwrap() = true;
            let handler = handler;
            let state = state;
            let thread_atomic = atomic.clone();
            let title = title.clone();
            spawn(move || {
                if let Err(e) = builder()
                    .title(&title)
                    .content(get_content())
                    .size(960, 540)
                    .resizable(false)
                    .debug(true)
                    .user_data(state)
                    .invoke_handler(handler)
                    .build()
                    .and_then(|mut webview| loop {
                        if *thread_atomic.read().unwrap() {
                            match webview.step() {
                                Some(Ok(_)) => (),
                                Some(e) => e?,
                                None => {
                                    *thread_atomic.write().unwrap() = false;
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
    }
}

fn run_tray(mut process: Child) -> WindowResult {
    let running_config = Arc::new(RwLock::new(false));
    let running_dashboard = Arc::new(RwLock::new(false));
    let mut app = Application::new()?;

    app.set_icon_from_resource("icon")?;
    app.add_menu_item(
        "Config",
        run_webview(
            "Config",
            running_dashboard.clone(),
            || Content::Html(get_config_content()),
            |_webview, _arg| Ok(()),
            (),
        ),
    )?;
    app.add_menu_item(
        "Dashboard",
        run_webview(
            "Dashboard",
            running_config.clone(),
            || Content::Html(get_dashboard_content()),
            |_webview, _arg| Ok(()),
            (),
        ),
    )?;
    app.add_menu_item("Quit", move |window| {
        *running_config.write().unwrap() = false;
        *running_dashboard.write().unwrap() = false;
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
