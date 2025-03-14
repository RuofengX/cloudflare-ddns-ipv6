use reqwest::{Method, blocking::Body};
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};

fn cf_api<T: DeserializeOwned>(
    api_key: &str,
    method: Method,
    path: &str,
    body: Option<Body>,
) -> Result<T, String> {
    let c = reqwest::blocking::Client::new();
    let url = format!("https://api.cloudflare.com/client/v4/{path}");
    let mut rb = c.request(method, url).bearer_auth(api_key);
    if let Some(body) = body {
        rb = rb.body(body);
    };
    let req = rb
        .build()
        .map_err(|e| format!("request build error: {e}"))?;

    let resp = c
        .execute(req)
        .map_err(|e| format!("request send error: {e}"))?
        .json::<T>()
        .map_err(|e| format!("response parse error: {e}"))?;
    Ok(resp)
}

#[derive(Debug, Deserialize)]
struct CommonResponse<T> {
    success: bool,
    errors: Vec<String>,
    result: Option<T>,
}

#[derive(Debug, Deserialize)]
pub struct Zone {
    pub id: String,
    pub name: String,
}

fn list_zones(api_key: &str) -> Result<Vec<Zone>, String> {
    let resp: CommonResponse<Vec<Zone>> = cf_api(api_key, Method::GET, "zones", None)?;
    if resp.success {
        Ok(resp.result.unwrap())
    } else {
        Err(format!("{:?}", resp.errors))
    }
}

pub fn get_zone_id_by_name(api_key: &str, name: &str) -> Result<String, String> {
    let zones = list_zones(api_key)?;
    zones
        .into_iter()
        .filter_map(|z| if z.name == name { Some(z.id) } else { None })
        .next()
        .ok_or(format!("zone(domain) name {name} not found"))
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Record {
    pub id: String,
    pub r#type: String,
    pub name: String,
    pub content: String,
    pub modified_on: String,
}

fn list_records(api_key: &str, zone_id: &str) -> Result<Vec<Record>, String> {
    let path = format!("zones/{zone_id}/dns_records");
    let resp: CommonResponse<Vec<Record>> = cf_api(api_key, Method::GET, &path, None)?;
    if resp.success {
        Ok(resp.result.unwrap())
    } else {
        Err(format!("{:?}", resp.errors))
    }
}

fn get_record_id_by_domain(
    api_key: &str,
    zone_id: &str,
    domain: &str,
) -> Result<Option<String>, String> {
    let records = list_records(api_key, zone_id)?;
    let ret = records
        .into_iter()
        .filter_map(|r| if r.name == domain { Some(r.id) } else { None })
        .next();
    Ok(ret)
}

#[derive(Debug, Serialize)]
struct RecordDesc {
    r#type: String,
    name: String,
    content: String,
    ttl: u32,
    comment: String,
}
impl RecordDesc {
    fn new_v6(name: &str, content: &str) -> Self {
        Self {
            r#type: "AAAA".to_string(),
            name: name.to_string(),
            content: content.to_string(),
            ttl: 60,
            comment: "auto created by cloudflare-ddns-ipv6".to_string(),
        }
    }
}
pub fn ensure_record(
    api_key: &str,
    zone_id: &str,
    domain: &str,
    content: &str,
) -> Result<Record, String> {
    let body = RecordDesc::new_v6(domain, content);
    let body = serde_json::to_string(&body)
        .map_err(|e| format!("encoding record error: {e}"))?
        .into();
    let resp: CommonResponse<Record> =
        if let Some(record_id) = get_record_id_by_domain(api_key, zone_id, domain)? {
            let path = format!("zones/{zone_id}/dns_records/{record_id}");
            cf_api(api_key, Method::PUT, &path, Some(body))?
        } else {
            let path = format!("zones/{zone_id}/dns_records");
            cf_api(api_key, Method::POST, &path, Some(body))?
        };
    if resp.success {
        Ok(resp.result.unwrap())
    } else {
        Err(format!(
            "create/update record fail with error response: \n{resp:#?}"
        ))
    }
}
