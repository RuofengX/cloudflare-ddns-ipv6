use std::env;

pub struct Config {
    pub bearer_key: String,
    pub zone: String,
    pub domain: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let bearer_key =
            env::var("CF_BEARER_KEY").map_err(|_| "environment variable CF_BEARER_KEY not set")?;
        let zone = env::var("CF_ZONE").map_err(|_| "environment variable CF_ZONE not set")?;
        let domain = env::var("CF_DOMAIN").map_err(|_| "environment variable CF_DOMAIN not set")?;
        Ok(Config {
            bearer_key,
            zone,
            domain,
        })
    }
}
