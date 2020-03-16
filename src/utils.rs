use std::io::{Error, Read};

pub use include_flate::*;
pub use lazy_static::*;
pub use log::{debug, error, info, warn, Level};
pub use std::fs::File;
use std::iter::once;
pub use std::path::{Path, PathBuf};
use std::ptr::null_mut;
pub use std::sync::{Arc, RwLock};

pub fn runas(process: &str, args: &str) -> bool {
    use winapi::um::shellapi::ShellExecuteW;
    use winapi::um::winuser::SW_SHOWNORMAL;
    let runas: Vec<u16> = "runas".encode_utf16().chain(once(0)).collect();
    let process: Vec<u16> = process.encode_utf16().chain(once(0)).collect();
    let args: Vec<u16> = args.encode_utf16().chain(once(0)).collect();
    (unsafe {
        ShellExecuteW(
            null_mut(),
            runas.as_ptr(),
            process.as_ptr(),
            args.as_ptr(),
            null_mut(),
            SW_SHOWNORMAL,
        )
    } as usize)
        > 32
}

pub fn msgbox(content: &str) {
    use winapi::um::winuser::{MessageBoxW, MB_ICONINFORMATION, MB_SYSTEMMODAL};

    let lp_text: Vec<u16> = content.encode_utf16().chain(once(0)).collect();
    let lp_caption: Vec<u16> = env!("CARGO_PKG_NAME")
        .encode_utf16()
        .chain(once(0))
        .collect();

    unsafe {
        MessageBoxW(
            null_mut(),
            lp_text.as_ptr(),
            lp_caption.as_ptr(),
            MB_ICONINFORMATION | MB_SYSTEMMODAL,
        );
    }
}

pub fn read_full_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    let mut buf = vec![];
    File::open(path.as_ref()).and_then(|mut file| file.read_to_end(&mut buf))?;
    Ok(buf)
}
