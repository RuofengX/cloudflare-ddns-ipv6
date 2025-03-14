mod api;
mod config;
mod ip;

use std::{thread, time::Duration};

use config::Config;

fn update(api_key: &str, zone: &str, domain: &str) -> Result<(), String> {
    let ip = ip::get_ipv6()?;

    let zone_id = api::get_zone_id_by_name(api_key, zone)?;
    let record = api::ensure_record(api_key, &zone_id, domain, &ip)?;
    println!("I: create/update record success, record: {record:?}");
    Ok(())
}

fn main() -> Result<(), String> {
    let config = Config::from_env()?;
    let api_key = &config.bearer_key;
    let zone = &config.zone;
    let domain = &config.domain;

    loop {
        println!("I: wake");
        update(api_key, zone, domain)?;
        println!("I: sleep 60 seconds");
        thread::sleep(Duration::from_secs(60));
    }
}
