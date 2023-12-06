use log::{debug, error};
use regex::Regex;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::string::String;

#[derive(Deserialize, Debug)]
struct NodeHealth {
    status: String,
    reason: Option<String>,
}

pub fn check_rabbitmq_node_health(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
) -> Vec<String> {
    let client = Client::new();
    let response = client
        .get(&format!("http://{}:{}/api/healthchecks/node", host, port))
        .basic_auth(username, Some(password))
        .send();
    match response {
        Ok(resp) => {
            let json_resp: NodeHealth = resp.json().unwrap();
            match json_resp.reason {
                Some(reason) => {
                    // 正则表达式: 提取 [] 内的内容
                    let re = Regex::new(r"\[(?P<partitions>.*?)\]").unwrap();
                    let result = re.captures(&reason).unwrap();
                    debug!("{:?}", &result["partitions"]);
                    return result["partitions"]
                        .split(',')
                        .map(|s| s.to_string())
                        .collect();
                }
                None => {
                    return vec![];
                }
            }
        }
        Err(err) => {
            error!("{:?}", err);
            return vec![];
        }
    };
}
