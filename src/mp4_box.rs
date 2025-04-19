use crate::utils::converter::converter::*;

#[derive(Clone)]
pub struct BoxHeader {
    pub size: u32,
    pub name: String
}

impl BoxHeader {
    pub fn new(data: &Vec<u8>, index: usize) -> Option<BoxHeader> {
        let size = to_32bit_int(data, index);
        let name = read_32bit_string(data, index + 4);
        match name {
            Some(name) => {
                Some(BoxHeader { size, name })
            }
            None => None
        }
    }
}

pub(crate) struct FileType {
    header: BoxHeader,
    major: String,
    major_version: u32,
    compatibles: Vec<String>
}

impl FileType {
    pub fn new(header: BoxHeader, data: &Vec<u8>, start_index: usize) -> Option<FileType> {
        let major = read_32bit_string(data, start_index);
        let major_version = to_32bit_int(data, start_index + 4);
        let mut compatibles = Vec::new();
        for i in 2..(header.size as usize / 4) - 2 {
            match read_32bit_string(data, start_index + 4*i) {
                Some(name) => compatibles.push(name),
                None => {}
            }
        }

        match major {
            Some(major) => {
                Some(Self {
                    header,
                    major,
                    major_version,
                    compatibles
                })
            }
            None => None
        }
    }

    pub fn print(&self) {
        println!("FileType header - {}\n - Major: {}\n - Major_version: {}\n - Compatibles: {:?}", self.header.size, self.major, self.major_version, self.compatibles);
    }
}
