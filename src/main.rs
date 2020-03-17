// #![windows_subsystem = "windows"]
#![feature(proc_macro_hygiene, with_options)]

mod content;
mod process;
mod utils;

use std::process::Command;

use content::*;
use process::*;
use systray::Application;
use utils::*;
use web_view::Content;

pub type WindowResult = Result<(), exitfailure::ExitFailure>;

fn subprocess_menu(
    mode: &str,
    flags: Arc<RwLock<Vec<ProcessDieFlag>>>,
) -> impl FnMut(&mut Application) -> Result<(), systray::Error> + '_ {
    move |_| {
        if let Err(e) = start_subprocess(mode).map(|flag| flags.write().unwrap().push(flag)) {
            msgbox(&format!("Fail to init dashboard: {}", e));
        }
        Ok::<_, systray::Error>(())
    }
}

fn run_tray(process: ProcessDieFlag) -> WindowResult {
    let flags = Arc::new(RwLock::new(vec![process]));
    let mut app = Application::new()?;

    app.set_icon_from_resource("icon")?;
    app.add_menu_item("Config", subprocess_menu("c", flags.clone()))?;
    app.add_menu_item("Dashboard", subprocess_menu("d", flags.clone()))?;
    app.add_menu_item("Quit", move |window| {
        window.quit();
        for process in flags.write().unwrap().iter() {
            *process.write().unwrap() = true;
        }
        while managed_count() > 0 {
            use std::thread::sleep;
            use std::time::Duration;
            sleep(Duration::from_millis(200));
        }
        Ok::<_, systray::Error>(())
    })?;
    app.wait_for_message()?;
    Ok(())
}

fn main() -> WindowResult {
    match std::env::args().take(2).last().as_deref() {
        Some("c") => start_webview(
            "Config",
            720,
            180,
            || Content::Html(get_config_content()),
            |_webview, _arg| Ok(()),
            (),
        ),
        Some("d") => start_webview(
            "Dashboard",
            1280,
            720,
            || Content::Html(get_dashboard_content()),
            |_webview, _arg| Ok(()),
            (),
        ),
        _ => {
            if Command::new("checknetisolation")
                .args(&[
                    "LoopbackExempt",
                    "-s",
                    "-n=Microsoft.Win32WebViewHost_cw5n1h2txyewy",
                ])
                .output()
                .map(|out| {
                    String::from_utf8(out.stdout)
                        .map(|out| {
                            out.to_ascii_lowercase()
                                .contains("microsoft.win32webviewhost_cw5n1h2txyewy")
                        })
                        .unwrap_or_default()
                })
                .unwrap_or_default()
            {
                msgbox("You must enable WebView loopback access to use the dashboard, there will be a request for administrator privileges later, please click Allow");
                if !runas(
                    "checknetisolation",
                    &[
                        "LoopbackExempt",
                        "-a",
                        "-n=Microsoft.Win32WebViewHost_cw5n1h2txyewy",
                    ]
                    .join(" "),
                ) {
                    msgbox("Fail to disable loopback access restrictions");
                }
            }
            if get_current_port(true).is_some() {
                match start_clash() {
                    Ok(child) => run_tray(child)?,
                    Err(e) => msgbox(&format!("Fail to run clash: {}", e)),
                };
            } else {
                msgbox("Fail to set clash dashboard port");
            }
        }
    }

    Ok(())
}
