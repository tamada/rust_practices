use crate::liner::Liner;

pub struct PlainLiner {
}

impl PlainLiner {
    pub fn new() -> PlainLiner {
        PlainLiner{}
    }
}
impl Liner for PlainLiner {
    fn decorate(&mut self, line: String) -> Option<String> {
        return Some(line)
    }
}
