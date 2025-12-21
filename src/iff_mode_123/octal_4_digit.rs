#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Octal4Digit {
    code: (u8, u8, u8, u8),
}

impl Octal4Digit {
    pub fn get(&self) -> (u8, u8, u8, u8) {
        self.code
    }

    pub fn set(&mut self, code: (u8, u8, u8, u8)) {
        self.code = code;
    }
}
