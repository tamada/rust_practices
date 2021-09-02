#[macro_use]
extern crate clap;

use clap::{Arg, ArgMatches};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;

struct Option {
    append_flag: bool,
    ignore_sigint: bool,
}

fn create_option(matches: &ArgMatches) -> Option {
    return Option {
        append_flag: matches.is_present("append"),
        ignore_sigint: matches.is_present("ignore-sigint"),
    };
}

fn perform_tee(output: File, _option: Option) {
    let mut dest1 = BufWriter::new(io::stdout());
    let mut dest2 = BufWriter::new(output);
    let mut buffer = [0; 256];
    let mut reader = BufReader::new(io::stdin());
    loop {
        match reader.read(&mut buffer) {
            Err(msg) => panic!("{}", msg.to_string()),
            Ok(n) => match n {
                0 => break,
                n => {
                    let buf = &buffer[..n];
                    let _ = dest1.write(buf);
                    let _ = dest2.write(buf);
                }
            },
        };
    }
}

fn perform(matches: ArgMatches) {
    let option = create_option(&matches);
    let dest_name = match matches.value_of("FILE") {
        None => panic!("not found"),
        Some(file_name) => file_name,
    };
    let dest_path = Path::new(dest_name);
    let display = &dest_path.display();
    let output = match OpenOptions::new()
        .write(true)
        .append(option.append_flag)
        .create(!dest_path.exists())
        .open(dest_name)
    {
        Err(msg) => panic!("{}: {}", display, msg.to_string()),
        Ok(output) => output,
    };
    perform_tee(output, option);
}

fn main() {
    let matches = app_from_crate!()
        .arg(Arg::from_usage(
            "-a --append 'append the output to the files rather than overwriting them.'",
        ))
        .arg(Arg::from_usage(
            "-i --ignore-sigint 'ignore the SIGINT signal.'",
        ))
        .arg(
            Arg::with_name("FILE")
                .help("A path name of an output file.")
                .required(true)
                .multiple(false)
                .index(1),
        )
        .get_matches();
    perform(matches);
}
