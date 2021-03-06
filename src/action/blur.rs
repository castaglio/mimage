use std::io::{self, Error};
use image::*;

use super::super::{FileArg, SubCommand};

pub struct Blur <'a> {
    filearg: FileArg<'a>,
    sigma: f32,
}

impl<'a> Blur<'a> {
    pub fn new(filearg: FileArg, sigma: f32) -> Blur {
        Blur { filearg, sigma }
    }
}

impl<'a> SubCommand for Blur<'a> {
    fn command(&self) -> io::Result<()> {
        match image::open(self.filearg.infile) {
            Ok(img) => {
                let img2 = img.blur(self.sigma);
                img2.save(self.filearg.outfile)
            },
            Err(err) => {
                match err {
                    ImageError::IoError(ioerror) => {
                        println!("Error file {}: {}", self.filearg.infile, ioerror);
                        return Err(ioerror)
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

