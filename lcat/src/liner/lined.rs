use crate::liner::Liner;

pub struct LinedLiner {
    number: u32,
}

impl LinedLiner {
    pub fn new() -> LinedLiner {
        LinedLiner{number: 0}
    }
}

impl Liner for LinedLiner {
    fn decorate(&mut self, line: String) -> Option<String> {
        self.number = self.number + 1;
        return Some(super::lined_line(line, self.number))
    }
}
