use std::process;

use sysinfo::{System, SystemExt};

pub fn check_riot_client() {
    let s = System::new_all();
    if s.processes_by_exact_name("RiotClientUx.exe")
        .collect::<Vec<_>>()
        .len()
        < 1
    {
        println!("starting riot client");
        process::Command::new("C:/Riot Games/Riot Client/RiotClientServices.exe")
            .args(["--headless"])
            .spawn()
            .unwrap();
    } else {
        println!("riot client already open");
    }
}
