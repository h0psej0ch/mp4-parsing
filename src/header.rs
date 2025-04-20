use crate::utils::converter::{read_32bit_string, to_32bit_int};

pub trait Header {
    fn name(&self) -> &str;
    fn size(&self) -> u32;
    fn print(&self);
}

#[derive(Clone)]
pub struct BoxHeader {
    pub size: u32,
    pub name: String,
    pub start_index: usize
}

impl BoxHeader {
    pub fn new(data: &Vec<u8>, index: usize) -> Option<BoxHeader> {
        let size = to_32bit_int(data, index);
        println!("{}", size);
        let name = read_32bit_string(data, index + 4);
        match name {
            Some(name) => {
                println!("{:?}", name);
                Some(BoxHeader { size, name, start_index: index + 8 })
            }
            None => None
        }
    }
}

impl Header for BoxHeader {
    fn name(&self) -> &str {
        self.name.as_str()
    }
    fn size(&self) -> u32 {
        self.size
    }
    fn print(&self) {
        println!("Default BoxHeader: {}", self.name);
    }
}