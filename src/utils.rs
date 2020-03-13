use std::fs::File;
use std::io::{Error, Read};

pub use include_flate::*;
pub use lazy_static::*;
pub use log::{debug, error, info, warn, Level};
pub use std::path::{Path, PathBuf};
pub use std::sync::{Arc, RwLock};

pub fn msgbox(content: &str) {
    use std::iter::once;
    use std::ptr::null_mut;
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
