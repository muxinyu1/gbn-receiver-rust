use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    port: i32,
    data_size: usize,
    saved_filename: String,
    saved_folder: String
}

impl Config {
    pub fn port(&self) -> i32 {
        return self.port;
    }
    pub fn data_size(&self) -> usize {
        return self.data_size;
    }
    pub fn saved_filename(&self) -> &str {return self.saved_filename.as_str()}

    pub fn set_saved_filename(&mut self, new_filename: String) {
        self.saved_filename = new_filename;
    }

    pub fn saved_folder(&self) -> &str{
        return self.saved_folder.as_str();
    }

    pub fn set_data_size(&mut self, data_size: usize) {
        self.data_size = data_size;
    }
}