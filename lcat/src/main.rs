use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::io::BufReader;
use clap::{crate_authors,crate_version,App,ArgMatches,Arg};

extern crate isatty;
use isatty::stdout_isatty;

pub trait Liner {
    fn decorate(&mut self, line: String) -> Option<String>;
}

pub struct DefaultLiner {
}

impl DefaultLiner {
    pub fn new() -> DefaultLiner {
        DefaultLiner{}
    }
}
impl Liner for DefaultLiner {
    fn decorate(&mut self, line: String) -> Option<String> {
        return Some(line)
    }
}

pub struct LinedLiner {
    number: i32,
}

impl LinedLiner {
    pub fn new() -> LinedLiner {
        LinedLiner{number: 0}
    }
}

impl Liner for LinedLiner {
    fn decorate(&mut self, line: String) -> Option<String> {
        self.number = self.number + 1;
        return Some(format!("{:5}   {}", self.number, line))
    }
}

pub struct NonBlankLinedLiner {
    number: i32,
}

impl NonBlankLinedLiner {
    pub fn new() -> NonBlankLinedLiner {
        return NonBlankLinedLiner{number: 0}
    }
}

impl Liner for NonBlankLinedLiner {
    fn decorate(&mut self, line: String) -> Option<String> {
        if line == "" {
            return Some(format!("      "))
        }
        self.number = self.number + 1;
        Some(format!("{:5}   {}", self.number, line))
    }
}

pub struct SqueezeLiner {
    prev_blank: bool
}

impl SqueezeLiner {
    pub fn new() -> SqueezeLiner {
        return SqueezeLiner{prev_blank: false}
    }
}

impl Liner for SqueezeLiner {
    fn decorate(&mut self, line: String) -> Option<String> {
        let r = line.trim();
        if r != "" {
            self.prev_blank = false;
            return Some(line)
        }
        if self.prev_blank {
            return None
        }
        self.prev_blank = true;
        return Some(line)
    }
}

fn main() {
    let matches =  App::new("cat")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Concatnate the given files.")
        .arg(
            Arg::with_name("numbers")
                .short("n")
                .long("numbers")
                .help("Prints the line with line numbers, start at 1.")
        )
        .arg(
            Arg::with_name("non-blank")
                .short("b")
                .long("numbers-non-blank")
                .help("Prints the non-blank line with line numbers, start at 1.")
        )
        .arg(
            Arg::with_name("squeeze")
                .short("s")
                .long("squeeze")
                .help("Squeezes multiple adjacent empty lines, causing the output to be single .")
        )
        .arg(
            Arg::with_name("FILEs")
                .help("Files for printing the contents.")
                .required(false)
                .multiple(true)
                .index(1)
        )
        .get_matches();

    perform(matches)
}

fn perform(matches: ArgMatches) {
    let paths = matches
        .values_of("FILEs")
        .unwrap()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    for path in &paths {
        let mut liner = build(&matches);
        print_file(&mut liner, path.to_string(), paths.len() > 1);
    }
}

fn build(matches: &ArgMatches) -> Box<dyn Liner> {
    if matches.is_present("numbers") {
        return Box::new(LinedLiner::new())
    } else if matches.is_present("non-blank") {
        return Box::new(NonBlankLinedLiner::new())
    } else if matches.is_present("squeeze") {
        return Box::new(SqueezeLiner::new())
    }
    return Box::new(DefaultLiner::new())
}

fn print_file(liner: &mut Box<dyn Liner>, path: String, show_header: bool) {
    let file_path = Path::new(&path);
    let display = file_path.display();
    let file = match File::open(&file_path) {
        Err(why) => panic!("{}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    if show_header && stdout_isatty() {
        println!("----------");
        println!("{}", display);
        println!("----------");
    }
    let input = BufReader::new(file);
    for line in input.lines() {
        let output = liner.decorate(line.unwrap());
        if output.is_some() {
            println!("{}", output.unwrap())
        }
    }
}
