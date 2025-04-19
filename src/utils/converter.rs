pub mod converter {
    pub fn to_32bit_int(data: &Vec<u8>, index: usize) -> u32 {
        if index + 3 >= data.len() {
            0
        } else {
            (data[index] as u32) << 24 | (data[index + 1] as u32) << 16 | (data[index + 2] as u32) << 8 | (data[index + 3] as u32)
        }
    }

    pub fn read_32bit_string(data: &Vec<u8>, index: usize) -> Option<String> {
        if index + 3 >= data.len() {
            None
        } else {
            let result = String::from_utf8(data[index..index+4].to_vec()).unwrap();
            Some(result)
        }
    }
}