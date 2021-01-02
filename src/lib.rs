mod action;

use std::{error::Error, io::{self,}};
use action::blur::Blur;
use clap::{ArgMatches, value_t};

pub struct FileArg<'a> {
    pub infile: &'a str,
    pub outfile: &'a str,
}

impl<'a> FileArg<'a> {
    pub fn new(infile: &'a str, outfile: &'a str) -> FileArg<'a> {
        FileArg { infile, outfile }
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

        // Let's run 
        // if matches.is_present("inputfile") {
        //     if matches.is_present("outputfile") {
        //         if matches.is_present("sigma") {
        //             let filearg = FileArg::new(matches.value_of("inputfile").unwrap(),
        //                                             matches.value_of("outputfile").unwrap());
        //             let blur = Blur::new(filearg, matches.value_of("sigma").unwrap_or("2").parse::<f32>().expect("wrong"));
                    
        //             if let Ok(()) = SubCommand::command(&blur) {
        //                 println!("blur finished succesfully")
        //             }
        //         }
        //     } 
        // }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

        
}
