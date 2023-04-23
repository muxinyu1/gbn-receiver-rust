use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Ack {
    seq_num: i32
}

impl Ack {
    pub fn new(seq_num: i32) -> Ack {
        return Ack{seq_num};
    }
}