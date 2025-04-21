use crate::header::BoxHeader;
use crate::header::Header;
use crate::utils::converter::*;
use crate::boxes::mvhd::MovieHeader;

pub struct MoovBox {
    header: BoxHeader,
    sub_boxes: Vec<Box<dyn Header>>
}

impl MoovBox {
    pub fn new(header: BoxHeader, data: &Vec<u8>) -> Option<MoovBox> {

        let mut sub_boxes: Vec<Box<dyn Header>> = Vec::new();
        let mut size: usize = 0;
        while size + 8 < header.size as usize {
            let next_header = BoxHeader::new(data, header.start_index + size)?;
            match next_header.name.as_str() {
                "mvhd" => {
                    size += next_header.size as usize;
                    sub_boxes.push(Box::new(MovieHeader::new(data, header.start_index + 8, next_header)?));
                    sub_boxes[sub_boxes.len() -1].print();
                },
                _ => {
                    size += next_header.size as usize;
                }
            }
        }

        println!("{}", to_32bit_int(data, header.start_index));
        println!("{:?}", read_32bit_string(data, header.start_index + 4));
        Some(Self {
            header,
            sub_boxes
        })
    }
}

impl Header for MoovBox {
    fn name(&self) -> &str {
        self.header.name.as_str()
    }

    fn size(&self) -> u32 {
        self.header.size
    }

    fn print(&self) {
        //TODO
    }
}