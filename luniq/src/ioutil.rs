use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

pub fn find_input(input: Option<&str>) -> Result<Box<dyn BufRead>, String> {
    return match input {
        None => input_from_stdin(),
        Some(file) => input_from_file(file),
    };
}

pub fn find_output(output: Option<&str>) -> Result<Box<dyn Write>, String> {
    return match output {
        None => output_to_stdout(),
        Some(file) => output_to_file(file),
    };
}

fn input_from_file(file_name: &str) -> Result<Box<dyn BufRead>, String> {
    if file_name == "-" {
        return input_from_stdin();
    }
    let input_path = Path::new(file_name);
    let display = &input_path.display();
    if !input_path.exists() {
        return Err(format!("{}: not found", display));
    }
    return match File::open(&input_path) {
        Err(why) => Err(why.to_string()),
        Ok(file) => Ok(Box::new(BufReader::new(file))),
    };
}

fn input_from_stdin() -> Result<Box<dyn BufRead>, String> {
    return Ok(Box::new(BufReader::new(io::stdin())));
}

fn output_to_file(output: &str) -> Result<Box<dyn Write>, String> {
    return match OpenOptions::new().write(true).create(true).open(output) {
        Err(msg) => Err(msg.to_string()),
        Ok(file) => Ok(Box::new(BufWriter::new(file))),
    };
}

fn output_to_stdout() -> Result<Box<dyn Write>, String> {
    return Ok(Box::new(BufWriter::new(io::stdout())));
}
