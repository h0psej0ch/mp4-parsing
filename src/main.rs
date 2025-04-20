mod boxes;
mod utils;
mod header;

use std::error::Error;
use std::fs;

use boxes::ftyp::FileType;
use boxes::moov::MoovBox;

use header::BoxHeader;
use header::Header;

fn main() -> Result<(), Box<dyn Error>> {

    let data: Vec<u8> = fs::read("example.mp4")?;

    let mut headers: Vec<BoxHeader> = Vec::new();

    let mut ftyp: Option<FileType> = None;
    let mut moov: Option<MoovBox> = None;

    let mut index = 0;
    'big: while index < data.len() {
        let header = BoxHeader::new(&data, index);
        match header {
            Some(header) => {
                headers.push(header.clone());
                index += header.size as usize;
            }
            None => {break 'big}
        }
    }

    headers.into_iter().for_each(|header| {
        match header.name.as_str() {
            "ftyp" => {
                ftyp = Some(FileType::new(header, &data).unwrap());
            },
            "moov" => {
                moov = MoovBox::new(header, &data)
            }
            _ => {
                println!("Unknown type")
            }
        }
    });

    ftyp.expect("I just made this shit lmao").print();


    Ok(())
}