pub mod mp4_box;
mod utils;

use std::error::Error;
use std::fs;
use mp4_box::FileType;
use mp4_box::BoxHeader;

fn main() -> Result<(), Box<dyn Error>> {

    let data: Vec<u8> = fs::read("example.mp4")?;

    let mut headers: Vec<BoxHeader> = Vec::new();

    let mut ftyp: Option<FileType> = None;

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
        // println!("{:10} | {:10}", header.size, header.name);
        match header.name.as_str() {
            "ftyp" => {
                ftyp = Some(FileType::new(header, &data, 8).unwrap());
            },
            _ => {
                println!("Unknown type")
            }
        }
    });

    ftyp.expect("I just made this shit lmao").print();


    Ok(())
}