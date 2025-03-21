use std::{thread, time::Duration};

use log::{info, warn, error};

mod api;
mod config;
mod ip;

fn update(api_key: &str, zone: &str, domain: &str) -> Result<(), String> {
    let ip = ip::get_ipv6()?;

    let record = api::ensure_record(api_key, &zone, domain, &ip)?;
    warn!("create/update record success, record: {record:?}");
    Ok(())
}

fn main() -> Result<(), String> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let config = config::Config::from_env()?;
    let api_key = &config.bearer_key;
    let zone = &config.zone;
    let domain = &config.domain;

    loop {
        info!("wake");
        match update(api_key, zone, domain) {
            Err(e) => {
                error!("fail to update dns record: {e}")
            }
            _ => (),
        }
        info!("sleep 60 seconds");
        thread::sleep(Duration::from_secs(60));
    }
}
