use crate::utils::*;

lazy_static! {
    static ref PORT: Arc<RwLock<u16>> = Arc::new(RwLock::new(7892));
}

pub fn get_current_port(write: bool) -> Option<u16> {
    use serde_yaml::{from_slice, to_writer, Value};
    use std::collections::BTreeMap;
    use std::net::SocketAddr;
    read_full_file("config.yaml")
        .ok()
        .and_then(|s| from_slice::<BTreeMap<String, Value>>(&s).ok())
        .and_then(|mut map| {
            if let Some(Value::String(addr)) = map.get("external-controller") {
                addr.parse::<SocketAddr>().map(|ip| ip.port()).ok()
            } else if write {
                map.insert(
                    "external-controller".to_string(),
                    Value::String(format!("127.0.0.1:{}", PORT.read().unwrap())),
                );
                File::with_options()
                    .write(true)
                    .truncate(true)
                    .open("config.yaml")
                    .ok()
                    .and_then(|file| to_writer(file, &map).map(|_| *PORT.read().unwrap()).ok())
            } else {
                None
            }
        })
}

pub fn get_config_content() -> String {
    flate!(static CONTENT: str from "lib/config/dist/index.html");
    CONTENT.clone()
}

pub fn get_dashboard_content() -> String {
    flate!(static DIST_CONTENT: str from "lib/yacd/public/index.html");
    lazy_static! {
        static ref CONTENT: Arc<RwLock<String>> = Arc::new(RwLock::new(DIST_CONTENT.to_string()));
    }
    if let Some(port) = get_current_port(false) {
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
