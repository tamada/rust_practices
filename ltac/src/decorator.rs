use clap::ArgMatches;

pub fn construct_decorator(matches: &ArgMatches) -> Box<dyn Decorator> {
    let increment = if matches.is_present("reverse-numbers") {
        -1
    } else {
        1
    };
    if matches.is_present("numbers") {
        return Box::new(LineDecorator {
            line_number: 0,
            increment: increment,
        });
    } else if matches.is_present("squeeze") {
        return Box::new(SqueezeLineDecorator {
            is_prev_blank: false,
        });
    }
    return Box::new(NoDecorator {});
}
pub trait Decorator {
    fn init(&mut self, lines: &Vec<String>);
    fn decorate(&mut self, line: String) -> Option<String>;
}

pub struct NoDecorator {}

impl Decorator for NoDecorator {
    fn init(&mut self, _: &Vec<String>) {}
    fn decorate(&mut self, line: String) -> Option<String> {
        Some(line)
    }
}

pub struct LineDecorator {
    line_number: i32,
    increment: i32,
}

impl Decorator for LineDecorator {
    fn init(&mut self, lines: &Vec<String>) {
        if self.increment < 0 {
            self.line_number = lines.len() as i32 + 1;
        }
    }
    fn decorate(&mut self, line: String) -> Option<String> {
        self.line_number = self.line_number + self.increment;
        Some(format!("{:6}   {}", self.line_number, line))
    }
}

pub struct SqueezeLineDecorator {
    is_prev_blank: bool,
}

impl Decorator for SqueezeLineDecorator {
    fn init(&mut self, _: &Vec<String>) {}
    fn decorate(&mut self, line: String) -> Option<String> {
        let current = line.trim();
        if current != "" {
            self.is_prev_blank = false;
            return Some(line);
        }
        if self.is_prev_blank {
            return None;
        }
        self.is_prev_blank = true;
        return Some(line);
    }
}
