use crate::header::BoxHeader;
use crate::header::Header;
use crate::utils::converter::*;

pub struct UserDataBox {
    header: BoxHeader
}

impl UserDataBox {
    pub fn new(header: BoxHeader, data: &Vec<u8>) -> Option<UserDataBox> {
        Some(Self {
            header
        })
    }
}

impl Header for UserDataBox {
    fn name(&self) -> &str {
        self.header.name.as_str()
    }

    fn size(&self) -> u32 {
        self.header.size
    }

    fn print(&self) {
        println!("TrackBox: {}", self.name());
    }
}