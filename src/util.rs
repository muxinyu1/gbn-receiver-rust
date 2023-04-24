use std::mem::size_of;

pub fn get_filename_and_frame_cnt(buffer: &[u8]) -> (String, i32) {
    const FILENAME_LEN_START_POS: usize = size_of::<i32>() + size_of::<usize>() + size_of::<i32>();
    const FILENAME_START_POS: usize = FILENAME_LEN_START_POS + size_of::<i32>();
    let filename_len = i32::from_le_bytes([
        buffer[FILENAME_LEN_START_POS],
        buffer[FILENAME_LEN_START_POS + 1],
        buffer[FILENAME_LEN_START_POS + 2],
        buffer[FILENAME_LEN_START_POS + 3],
    ]);
    let filename = std::str::from_utf8(
        &buffer[FILENAME_START_POS..(FILENAME_START_POS + filename_len as usize)],
    )
    .unwrap()
    .to_string();
    const FRAME_CNT_START_POS: usize = size_of::<i32>() + size_of::<usize>();
    let frame_cnt = i32::from_le_bytes([
        buffer[FRAME_CNT_START_POS],
        buffer[FRAME_CNT_START_POS + 1],
        buffer[FRAME_CNT_START_POS + 2],
        buffer[FRAME_CNT_START_POS + 3],
    ]);
    return (filename, frame_cnt);
}

pub fn get_data_size(buffer: &[u8]) -> usize {
    let usize_bytes: [u8; 8] = buffer[size_of::<i32>()..(size_of::<i32>() + size_of::<usize>())].try_into().expect("获取data_size错误");
    return usize::from_le_bytes(usize_bytes);
}
