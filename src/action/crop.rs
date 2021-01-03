use std::io::{self, Error};
use image::*;

use super::super::{FileArg, SubCommand, Coord, Size};
pub struct Crop <'a> {
    filearg: FileArg<'a>,
    coord: Coord,
    size: Size,
}

impl<'a> Crop<'a> {
    pub fn new(filearg: FileArg, coord: Coord, size: Size) -> Crop {
        Crop { filearg, coord, size }
    }
}

impl<'a> SubCommand for Crop<'a> {
    fn command(&self) -> io::Result<()> {
        match image::open(self.filearg.infile) {
            Ok(mut img) => {
                let img2 = img.crop(self.coord.x, self.coord.y, self.size.width, self.size.height);
                img2.save(self.filearg.outfile)
            },
            Err(err) => {
                match err {
                    ImageError::IoError(ioerror) => {
                        println!("Error file {}: {}", self.filearg.infile, ioerror);
                        Err(ioerror)
                    } ,
                    _ => {
                        println!("Error {}", err);
                        Err(Error::new(io::ErrorKind::Other, err.to_string()))
                    }
                }
            }
        }
    }
}
