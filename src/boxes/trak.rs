use crate::header::BoxHeader;
use crate::header::Header;
use crate::utils::converter::*;

pub struct TrackBox {
    header: BoxHeader
}

impl TrackBox {
    pub fn new(header: BoxHeader, data: &Vec<u8>) -> Option<TrackBox> {
        Some(Self {
            header
        })
    }
}

impl Header for TrackBox {
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