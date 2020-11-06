#[derive(Debug)]
pub struct Position{
    _line: u64
}

impl Position{
    pub fn new() -> Position{
        Position{_line: 1}
    }

    pub fn line(&self) -> u64 {
        self._line
    }

    pub fn set_line(&mut self, line: u64) -> &mut Position {
        self._line = line;
        self
    }

    pub fn reset(&mut self) -> &mut Position{
        self._line = 1;
        self
    }
}