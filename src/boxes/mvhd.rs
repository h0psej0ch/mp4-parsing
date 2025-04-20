use crate::header::BoxHeader;
use crate::header::Header;
use crate::utils::converter::*;

pub struct MovieHeader {
    header: BoxHeader,
    version: u8,
    flags: [u8; 3],
    creation_time: u64,
    modified_time: u64,
    time_scale: u32,
    duration: u64
}

impl MovieHeader {
    pub fn new(data: &Vec<u8>, index: usize, header: BoxHeader) -> Option<MovieHeader> {
        let mut i = index;
        let version = data[i];
        let flags = [data[i+1], data[i+2], data[i+3]];
        i += 4;
        let creation_time: u64;
        let modified_time: u64;
        if version == 1 {
            creation_time = to_64bit_int(data, i);
            modified_time = to_64bit_int(data, i+8);
            i += 16;
        } else {
            creation_time = to_32bit_int(data, i) as u64;
            modified_time = to_32bit_int(data, i+4) as u64;
            i += 8;
        }
        let duration: u64;
        let time_scale = to_32bit_int(data, i);
        if version == 1 {
            duration = to_64bit_int(data, i);
            i += 8;
        } else {
            duration = to_32bit_int(data, i) as u64;
            i += 4;
        }

        Some(Self {
            header,
            version,
            flags,
            creation_time,
            modified_time,
            time_scale,
            duration
        })
    }
}

impl Header for MovieHeader {
    fn name(&self) -> &str {
        self.header.name.as_str()
    }

    fn size(&self) -> u32 {
        self.header.size
    }

    fn print(&self) {
        println!("MovieHeader:\n- Size: {}\n- Version: {}\n- Flags: {:?}\n- Creation Time: {}\n- Modification Time: {}\n- Time Scale: {}\n- Duration: {}\n",
                 self.header.size, self.version, self.flags, self.creation_time, self.modified_time, self.time_scale, self.duration);
    }
}