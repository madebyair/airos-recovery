use std::path::Path;
use std::fs;
use serde_json;
use serde::{Deserialize, Serialize};
use std::process::Command;


#[derive(Serialize, Deserialize)]
struct DiskJson {
    boot: String,
    root: String
}

pub fn get_uuid(rtn: &str) -> Result<String, serde_json::Error> {
    let path;

    if Path::new("/cdrom/DISK_LAYOUT").exists() {
        path = "/cdrom/DISK_LAYOUT"
    } else {
        path = "/DISK_LAYOUT"
    }

    let json : DiskJson = serde_json::from_str(&*fs::read_to_string(path).unwrap())?;

    if rtn == "boot" {
        Ok(json.boot)
    } else {
        Ok(json.root)
    }
}

pub fn get_disk() -> String {
    let uuid = get_uuid("boot").unwrap();

    let command = Command::new("sh")
        .arg("-c")
        .arg(format!("blkid -t UUID={:?} -o device", uuid))
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute blkid. Are you root?"));

    String::from_utf8(command.stdout).expect("got non UTF-8 data").replace("1", "").replace("\n", "").replace(" ", "")
}