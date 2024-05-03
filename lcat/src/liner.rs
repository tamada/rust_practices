mod plain;
mod lined;
mod lined_with_noblank;
mod squeeze;

use crate::cli::CliOpts;

pub trait Liner {
    fn decorate(&mut self, line: String) -> Option<String>;
}

pub fn create_liner(opts: &CliOpts) -> Box<dyn Liner> {
    if opts.number && !opts.non_blank && opts.squeeze {
        Box::new(squeeze::SqueezeLinedLiner::new())
    } else if !opts.number && opts.non_blank && opts.squeeze {
        Box::new(squeeze::SqueezeLinedWithNoBlankLiner::new())
    } else if !opts.number && !opts.non_blank && !opts.squeeze {
        Box::new(plain::PlainLiner::new())
    } else if !opts.number && !opts.non_blank && opts.squeeze {
        Box::new(squeeze::SqueezeLiner::new())
    } else if opts.number && !opts.non_blank && !opts.squeeze {
        Box::new(lined::LinedLiner::new())
    } else if !opts.number && opts.non_blank && !opts.squeeze {
        Box::new(lined_with_noblank::LinedWithNoBlankLiner::new())
    } else if opts.number && opts.non_blank && opts.squeeze {
        Box::new(squeeze::SqueezeLinedLiner::new())
    } else {
        Box::new(lined::LinedLiner::new())
    }
}

pub fn lined_line(line: String, number: u32) -> String {
    format!("{:5}   {}", number, line)
}