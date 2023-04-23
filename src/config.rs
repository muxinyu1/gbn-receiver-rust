use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    port: i32,
    data_size: i32
}

impl Config {
    pub fn new(port: i32, data_size: i32) -> Self {
        return Config {port, data_size};
    }
    pub fn port(&self) -> i32 {
        return self.port;
    }
}