pub fn to_32bit_int(data: &Vec<u8>, index: usize) -> u32 {
    if index + 3 >= data.len() {
        0
    } else {
        (data[index] as u32) << 24 |
        (data[index + 1] as u32) << 16 |
        (data[index + 2] as u32) << 8 |
        (data[index + 3] as u32)
    }
}

pub fn to_64bit_int(data: &Vec<u8>, index: usize) -> u64 {
    if index + 7 >= data.len() {
        0
    } else {
        (data[index] as u64) << 56 |
        (data[index + 1] as u64) << 48 |
        (data[index + 2] as u64) << 40 |
        (data[index + 3] as u64) << 32 |
        (data[index + 4] as u64) << 24 |
        (data[index + 5] as u64) << 16 |
        (data[index + 6] as u64) << 8 |
        (data[index + 7] as u64)
    }
}

pub fn read_32bit_string(data: &Vec<u8>, index: usize) -> Option<String> {
    if index + 3 >= data.len() {
        None
    } else {
        let result = String::from_utf8(data[index..index+4].to_vec());
        match result {
            Ok(string) => Some(string),
            Err(_) => None,
        }
    }
}