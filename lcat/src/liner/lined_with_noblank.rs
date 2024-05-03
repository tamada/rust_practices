use crate::liner::Liner;

pub struct LinedWithNoBlankLiner {
    number: u32,
}

impl LinedWithNoBlankLiner {
    pub fn new() -> LinedWithNoBlankLiner {
        return LinedWithNoBlankLiner{number: 0}
    }
}

impl Liner for LinedWithNoBlankLiner {
    fn decorate(&mut self, line: String) -> Option<String> {
        if line.trim() == "" {
            return Some(format!("      "))
        }
        self.number = self.number + 1;
        Some(super::lined_line(line, self.number))
    }
}
