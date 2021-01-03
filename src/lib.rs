mod action;

use std::{error::Error, io::{self,}};
use clap::{ArgMatches, value_t};

use action::blur::Blur;
use action::crop::Crop;
use action::fractal::Fractal;

pub struct FileArg<'a> {
    pub infile: &'a str,
    pub outfile: &'a str,
}

impl<'a> FileArg<'a> {
    pub fn new(infile: &'a str, outfile: &'a str) -> FileArg<'a> {
        FileArg { infile, outfile }
    }
}

pub struct Coord {
    pub x: u32,
    pub y: u32,
}

impl Coord {
    pub fn new(x: u32, y: u32) -> Coord {
        Coord { x, y }
    }
}
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Size {
        Size { width, height }
    }
}

pub trait SubCommand {
    fn command(&self) -> io::Result<()>;
}

pub fn run(matches: ArgMatches) -> Result<(), Box<dyn Error>> {

    // Let's run blur
    if let Some(matches) = matches.subcommand_matches("blur") {
        let inputfile = value_t!(matches.value_of("inputfile"), String).unwrap_or_else(|e| e.exit());
        let outputfile = value_t!(matches.value_of("outputfile"), String).unwrap_or_else(|e| e.exit());
        let sigma = value_t!(matches.value_of("sigma"), f32).unwrap_or_else(|e| e.exit());

        let filearg = FileArg::new(&inputfile, &outputfile);
        let blur = Blur::new(filearg, sigma);

        match SubCommand::command(&blur) {
            Ok(()) => println!("blur finished succesfully"),
            Err(err) => {
                println!("blur finished with error: {}", err);
                std::process::exit(1)
            }
        }

        return Ok(())
    }

    // Let's run crop
    if let Some(matches) = matches.subcommand_matches("crop") {
        let inputfile = value_t!(matches.value_of("inputfile"), String).unwrap_or_else(|e| e.exit());
        let outputfile = value_t!(matches.value_of("outputfile"), String).unwrap_or_else(|e| e.exit());
        let x = value_t!(matches.value_of("x"), u32).unwrap_or_else(|e| e.exit());
        let y = value_t!(matches.value_of("y"), u32).unwrap_or_else(|e| e.exit());
        let coord = Coord::new(x, y);
        let width = value_t!(matches.value_of("width"), u32).unwrap_or_else(|e| e.exit());
        let height = value_t!(matches.value_of("height"), u32).unwrap_or_else(|e| e.exit());
        let size = Size::new(width, height);
        let filearg = FileArg::new(&inputfile, &outputfile);
        let crop = Crop::new(filearg, coord, size);

        match SubCommand::command(&crop) {
            Ok(()) => println!("crop finished succesfully"),
            Err(err) => {
                println!("crop finished with error: {}", err);
                std::process::exit(1)
            }
        }

        return Ok(())
    }

    // Let's run fractal
    if let Some(matches) = matches.subcommand_matches("fractal") {
        let inputfile = "".to_string();
        let outputfile = value_t!(matches.value_of("outputfile"), String).unwrap_or_else(|e| e.exit());
        let filearg = FileArg::new(&inputfile, &outputfile);
        let crop =  Fractal::new(filearg);

        match SubCommand::command(&crop) {
            Ok(()) => println!("fractal finished succesfully"),
            Err(err) => {
                println!("fractal finished with error: {}", err);
                std::process::exit(1)
            }
        }

        return Ok(())
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

        
}
