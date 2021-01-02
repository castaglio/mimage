use std::io;
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
        let mut img2= DynamicImage::new_bgr8(0, 0);
        match image::open(self.filearg.infile) {
            Ok(img) => {
                img2 = img.blur(self.sigma)
            },
            Err(err) => {
                match err {
                    ImageError::IoError(ioerror) => {
                        println!("Error file {}: {}", self.filearg.infile, ioerror);
                        return Err(ioerror)
                    } ,
                    _ => println!("Error {}", err),
                }
            }
        }
        img2.save(self.filearg.outfile)
    }
}

