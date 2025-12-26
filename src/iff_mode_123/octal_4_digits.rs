#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Octal4Digits {
    code: (u8, u8, u8, u8),
}

impl Octal4Digits {
    pub fn get(&self) -> (u8, u8, u8, u8) {
        self.code
    }

    pub fn set(&mut self, code: (u8, u8, u8, u8)) {
        if code.0 > 7 || code.1 > 7 || code.2 > 7 || code.3 > 7 {
            println!("Octal digit is out of range!")
        }
        self.code = code;
    }
}
