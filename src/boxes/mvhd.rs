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
    duration: u64,
    playback_speed: u32,
    user_volume: u16,
    // Reserved 10 bytes
    window_geometry: [[u32 ; 3] ; 3],
    preview_start: u32,
    preview_length: u32,
    poster_frame_time: u32,
    selection_start: u32,
    selection_length: u32,
    current_time: u32,
    next_track_id: u32
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
        i += 4;
        if version == 1 {
            duration = to_64bit_int(data, i);
            i += 8;
        } else {
            duration = to_32bit_int(data, i) as u64;
            i += 4;
        }

        let playback_speed: u32 = to_32bit_int(data, i);
        let user_volume: u16 = to_16bit_int(data, i+4);
        // Skip 10 reserved bytes
        let window_geometry: [[u32; 3] ; 3] = [
            [to_32bit_int(data, i+16),to_32bit_int(data, i+20),to_32bit_int(data, i+24)],
            [to_32bit_int(data, i+28),to_32bit_int(data, i+32),to_32bit_int(data, i+36)],
            [to_32bit_int(data, i+40),to_32bit_int(data, i+44),to_32bit_int(data, i+48)]
        ];
        let preview_start: u32 = to_32bit_int(data, i+52);
        let preview_length: u32 = to_32bit_int(data, i+56);
        let poster_frame_time: u32 = to_32bit_int(data, i+60);
        let selection_start: u32 = to_32bit_int(data, i+64);
        let selection_length: u32 = to_32bit_int(data, i+68);
        let current_time: u32 = to_32bit_int(data, i+72);
        let next_track_id: u32 = to_32bit_int(data, i+76);
        Some(Self {
            header,
            version,
            flags,
            creation_time,
            modified_time,
            time_scale,
            duration,
            playback_speed,
            user_volume,
            window_geometry,
            preview_start,
            preview_length,
            poster_frame_time,
            selection_start,
            selection_length,
            current_time,
            next_track_id
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
        println!(
            "MovieHeader:
    - Size: {}
    - Version: {}
    - Flags: {:?}
    - Creation Time: {}
    - Modification Time: {}
    - Time Scale: {}
    - Duration: {}
    - Playback Speed: {}
    - User Volume: {}
    - Window Geometry: {:?}
    - Preview Start: {}
    - Preview Length: {}
    - Poster Frame Time: {}
    - Selection Start: {}
    - Selection Length: {}
    - Current Time: {}
    - Next Track ID: {}",
    self.header.size,
    self.version,
    self.flags,
    self.creation_time,
    self.modified_time,
    self.time_scale,
    self.duration,
    self.playback_speed,
    self.user_volume,
    self.window_geometry,
    self.preview_start,
    self.preview_length,
    self.poster_frame_time,
    self.selection_start,
    self.selection_length,
    self.current_time,
    self.next_track_id
        );
    }
}