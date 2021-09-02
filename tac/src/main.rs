#[macro_use]
extern crate clap;

use clap::{Arg, ArgMatches};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

mod decorator;

fn read_all(path: &String) -> Option<Vec<String>> {
    let file_path = Path::new(&path);
    let file = match File::open(&file_path) {
        Err(why) => panic!("{}: {}", file_path.display(), why.to_string()),
        Ok(file) => file,
    };
    let input = BufReader::new(file);
    let mut vec = Vec::new();
    for line in input.lines() {
        vec.insert(0, line.unwrap());
    }
    return Some(vec);
}

fn perform_each(path: String, decorator: &mut Box<dyn decorator::Decorator>) {
    let lines = match read_all(&path) {
        None => panic!("{}: read error", path),
        Some(vec) => vec,
    };
    decorator.init(&lines);
    for line in lines {
        match decorator.decorate(line) {
            Some(line) => println!("{}", line),
            None => (),
        };
    }
}

fn perform(matches: ArgMatches) {
    let paths = matches
        .values_of("FILEs")
        .unwrap()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let mut decorator = decorator::construct_decorator(&matches);
    for path in paths {
        perform_each(path, &mut decorator)
    }
}

fn main() {
    let matches = app_from_crate!()
        .arg(Arg::from_usage("-n --numbers 'prints the line with the line numbers, start at 1.'"))
        .arg(Arg::from_usage("-r --reverse-numbers 'print the line with the line numbers, start at the length of the file.'"))
        .arg(Arg::from_usage("-s --squeeze 'squeezes multipled adjacent empty lines, causing the output to the single.'"))
        .arg(Arg::with_name("FILEs")
            .help("files for printing the contents.")
            .required(false)
            .multiple(true)
            .index(1))
        .get_matches();
    perform(matches);
}
