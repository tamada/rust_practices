#[macro_use]
extern crate clap;

use clap::{Arg, ArgMatches};
use uniqer::Uniqer;

mod ioutil;
mod uniqer;

fn perform(matches: ArgMatches) {
    let mut uniqer = uniqer::construct_uniqer(&matches);
    let mut input = match ioutil::find_input(matches.value_of("INPUT")) {
        Err(why) => panic!("{}", why.to_string()),
        Ok(input) => input,
    };
    let mut output = match ioutil::find_output(matches.value_of("OUTPUT")) {
        Err(why) => panic!("{}", why.to_string()),
        Ok(output) => output,
    };
    let mut buffer = String::new();
    loop {
        match input.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                match uniqer.next(buffer.to_string()) {
                    None => (),
                    Some(line) => {
                        let _ = output.write(line.as_bytes());
                        ()
                    }
                };
                buffer.clear();
            }
            Err(why) => panic!("{}", why.to_string()),
        }
    }
}

fn main() {
    let matches = app_from_crate!()
        .arg(Arg::from_usage("-a --adjacent 'Deletes only adjacent duplicated lines.'"))
        .arg(Arg::from_usage("-d --delete-lines 'Only prints deleted lines.'"))
        .arg(Arg::from_usage("-i --ignore-case  'Case insensitive.'"))
        .arg(Arg::with_name("INPUT")
            .help("gives file name of input.  If argument is single dash ('-') or absent, the program read strings from stdin.")
            .required(false)
            .multiple(false)
            .index(1))
        .arg(Arg::with_name("OUTPUT")
            .help("represents the destination.")
            .required(false)
            .multiple(false)
            .index(2))
        .get_matches();
    perform(matches);
}
