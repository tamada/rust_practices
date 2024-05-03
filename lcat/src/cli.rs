use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    version, author, about,
    arg_required_else_help = true,
)]
pub struct CliOpts {
    #[arg(short, long, default_value_t = false, help = "Prints the line with line numbers, start at 1. If -n and -b are both specified, -b is ignored.")]
    pub number: bool,

    #[arg(short = 'b', long = "numbers-non-blank", default_value_t = false, help = "Prints the non-blank line with line numbers, start at 1.")]
    pub non_blank: bool, 

    #[arg(short, long, default_value_t = false, help = "Squeezes multiple adjacent empty lines, causing the output to be single.")]
    pub squeeze: bool,

    #[arg(value_name = "FILE", help = "Files for printing the contents.")]
    pub files: Vec<PathBuf>,    
}

