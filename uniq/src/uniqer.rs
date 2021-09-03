use clap::ArgMatches;

pub struct CliOption {
    ignore_case: bool,
    delete_lines: bool,
}

pub fn construct_uniqer(matches: &ArgMatches) -> DefaultUniqer {
    let cli_option = CliOption {
        ignore_case: matches.is_present("ignore-case"),
        delete_lines: matches.is_present("delete-lines"),
    };
    let uniqer: Box<dyn Uniqer> = if matches.is_present("adjacent") {
        Box::new(AdjacentUniqer {
            prev: String::new(),
        })
    } else {
        Box::new(PlainUniqer { lines: vec![] })
    };
    return DefaultUniqer {
        opt: cli_option,
        uniqer: uniqer,
    };
}

pub trait Uniqer {
    fn next(&mut self, line: String) -> Option<String>;
}

pub struct DefaultUniqer {
    uniqer: Box<dyn Uniqer>,
    opt: CliOption,
}

impl DefaultUniqer {
    fn line_or_none(&self, none_flag: bool, line: String) -> Option<String> {
        if none_flag {
            return None;
        }
        return Some(line);
    }
}

impl Uniqer for DefaultUniqer {
    fn next(&mut self, line: String) -> Option<String> {
        let target_line: String = if self.opt.ignore_case {
            line.to_lowercase()
        } else {
            line.to_string()
        };
        let result = self.uniqer.next(target_line);
        return match result {
            Some(line) => self.line_or_none(self.opt.delete_lines, line),
            None => self.line_or_none(!self.opt.delete_lines, line),
        };
    }
}

struct AdjacentUniqer {
    prev: String,
}

impl Uniqer for AdjacentUniqer {
    fn next(&mut self, line: String) -> Option<String> {
        if self.prev == line {
            return None;
        }
        self.prev = line.to_string();
        return Some(line);
    }
}

struct PlainUniqer {
    lines: Vec<String>,
}

impl Uniqer for PlainUniqer {
    fn next(&mut self, line: String) -> Option<String> {
        if self.lines.contains(&line) {
            return None;
        }
        self.lines.push(line.to_string());
        return Some(line);
    }
}
