use crate::header::BoxHeader;
use crate::header::Header;
use crate::utils::converter::*;

pub struct FileType {
    header: BoxHeader,
    major: String,
    major_version: u32,
    compatibles: Vec<String>
}

impl FileType {
    pub fn new(header: BoxHeader, data: &Vec<u8>) -> Option<FileType> {
        let major = read_32bit_string(data, header.start_index);
        let major_version = to_32bit_int(data, header.start_index + 4);
        let mut compatibles = Vec::new();
        for i in 2..(header.size as usize / 4) - 2 {
            match read_32bit_string(data, header.start_index + 4*i) {
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


}

impl Header for FileType {
    fn name(&self) -> &str {
        self.header.name.as_str()
    }

    fn size(&self) -> u32 {
        self.header.size
    }

    fn print(&self) {
        println!("FileType header - {}\n - Major: {}\n - Major_version: {}\n - Compatibles: {:?}", self.header.size, self.major, self.major_version, self.compatibles);
    }
}