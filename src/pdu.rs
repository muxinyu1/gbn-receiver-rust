struct Pdu {
    seq_num: i32,
    data_size: i32,
    data: Vec<u8>,
    checksum: i16,
}

impl Pdu {
    pub fn new() -> Pdu{
        return Pdu{
            seq_num: 0,
            data_size: 0,
            data: vec![],
            checksum: 0
        };
    }

    pub fn set_seq_num(&mut self, seq_num: i32) {
        self.seq_num = seq_num;
    }

    pub fn set_data_size(&mut self, data_size: i32) {
        self.data_size = data_size;
    }

    pub fn data(&self) -> &Vec<u8>{
        return &self.data;
    }

    pub fn set_checksum(&mut self, checksum: i16) {
        self.checksum = checksum;
    }
}