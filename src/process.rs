use crate::utils::*;
use std::env::{current_dir, current_exe};
use std::io::{Error, Result};
use std::process::{Child, Command};
use std::thread::{sleep, spawn};
use std::time::Duration;
use web_view::{builder, Content, WVResult, WebView};

pub type ProcessDieFlag = Arc<RwLock<bool>>;

pub fn start_webview<'a, S, C, T, I, L>(title: S, get_content: C, handler: I, state: L)
where
    S: AsRef<str>,
    C: Fn() -> Content<T> + Copy + Send + 'static,
    T: AsRef<str>,
    I: FnMut(&mut WebView<L>, &str) -> WVResult + Send + Copy + 'static,
    L: Send + Copy + 'static,
{
    let title = format!("Clash {}", title.as_ref());

    if let Err(e) = builder()
        .title(&title)
        .content(get_content())
        .size(960, 540)
        .resizable(false)
        .debug(true)
        .user_data(state)
        .invoke_handler(handler)
        .build()
        .and_then(WebView::run)
    {
        msgbox(&format!("Fail to init webview: {}", e));
    }
}

pub fn start_subprocess(mode: &str) -> Result<ProcessDieFlag> {
    manage_process(Command::new(current_exe()?).arg(mode).spawn()?)
}

pub fn start_clash() -> Result<ProcessDieFlag> {
    manage_process(
        Command::new("clash")
            .args(&[
                "-d",
                current_dir()
                    .unwrap_or(".".into())
                    .to_string_lossy()
                    .as_ref(),
            ])
            .spawn()?,
    )
}

lazy_static! {
    static ref MANAGED_COUNT: Arc<RwLock<u16>> = Arc::default();
}

pub fn managed_count() -> u16 {
    *MANAGED_COUNT.read().unwrap()
}

pub fn manage_process(process: Child) -> Result<ProcessDieFlag> {
    let process = Arc::new(Mutex::new(process));
    let can_kill = Arc::new(RwLock::new(false));
    let can_kill_ret = can_kill.clone();
    *MANAGED_COUNT.write().unwrap() += 1;
    spawn(move || {
        loop {
            if *can_kill.read().unwrap() {
                process.lock().unwrap().kill()?;
                break;
            } else {
                match process.lock().unwrap().try_wait() {
                    Ok(Some(_)) | Err(_) => break,
                    Ok(None) => sleep(Duration::from_millis(100)),
                }
            }
        }
        *MANAGED_COUNT.write().unwrap() -= 1;
        return Ok::<_, Error>(());
    });
    Ok(can_kill_ret)
}
