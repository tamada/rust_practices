use crate::liner::Liner;

pub(super) struct SqueezeLiner {
    prev_blank: bool,
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

pub(super) struct SqueezeLinedLiner {
    prev_blank: bool,
    number: u32,
}

impl SqueezeLinedLiner {
    pub fn new() -> SqueezeLinedLiner {
        return SqueezeLinedLiner{prev_blank: false, number: 0}
    }
}

impl Liner for SqueezeLinedLiner {
    fn decorate(&mut self, line: String) -> Option<String> {
        self.number = self.number + 1;
        let r = line.trim();
        if r != "" {
            self.prev_blank = false;
            Some(super::lined_line(line, self.number))
        } else if self.prev_blank {
            None
        } else {
            self.prev_blank = true;
            Some(super::lined_line(line, self.number))
        }
    }
}

pub(super) struct SqueezeLinedWithNoBlankLiner {
    prev_blank: bool,
    number: u32,
}

impl SqueezeLinedWithNoBlankLiner {
    pub fn new() -> SqueezeLinedWithNoBlankLiner {
        return SqueezeLinedWithNoBlankLiner{prev_blank: false, number: 0}
    }
}

impl Liner for SqueezeLinedWithNoBlankLiner {
    fn decorate(&mut self, line: String) -> Option<String> {
        let r = line.trim();
        if self.prev_blank && r == "" {
            None
        } else if self.prev_blank && r != "" {
            self.number = self.number + 1;
            self.prev_blank = false;
            Some(super::lined_line(line, self.number))
        } else if !self.prev_blank && r == "" {
            self.prev_blank = true;
            return Some(format!("      "))
        } else {
            self.number = self.number + 1;
            Some(super::lined_line(line, self.number))
        }
    }
}
