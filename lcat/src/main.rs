use std::fs::File;
use std::io::Read;
use std::io::Stdin;
use std::path::Path;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::exit;
use clap::Parser;

use cli::CliOpts;
use liner::create_liner;
use liner::Liner;

mod cli;
mod liner;

fn print_file(mut liner: &mut Box<dyn Liner>, path: PathBuf) -> Result<(), Error> {
    let file_path = Path::new(&path);
    let file = match File::open(file_path) {
        Err(_) => return Err(Error::FileNotFound(path)),
        Ok(file) => file,
    };
    let input = BufReader::new(file);
    print_impl(&mut liner, input)
}

fn print_impl<T: Read+Sized>(liner: &mut Box<dyn Liner>, input: BufReader<T>) -> Result<(), Error> {
    for line in input.lines() {
        if let Some(line) = liner.decorate(line.unwrap()) {
            println!("{}", line);
        }
    }
    Ok(())
}

#[derive(Debug)]
pub enum Error {
    FileNotFound(PathBuf),
}

fn perform(opts: CliOpts) -> Result<(), Vec<Error>> {
    let target = opts.files.clone();
    let mut liner = create_liner(&opts);
    let show_header = target.len() > 1 && atty::is(atty::Stream::Stdout);
    let mut errs = vec![];
    if target.len() == 0 {
        let reader = BufReader::<Stdin>::new(std::io::stdin());
        match print_impl(&mut liner, reader) {
            Err(e) => errs.push(e),
            _ => (),
        }
    } else {
        for file in target {
            if show_header {
                println!("----------\n{}\n----------\n", file.display());
            }
            match print_file(&mut liner, file) {
                Err(e) => errs.push(e),
                _ => (),
            }
        }
    }
    if errs.len() > 0 {
        Err(errs)
    } else {
        Ok(())
    }
}

fn main() {
    let opts = CliOpts::parse();
    match perform(opts) {
        Err(errs) => {
            for err in errs {
                println!("Error: {:?}", err)
            }
            exit(1);
        }
        _ => (),
    }
}