
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct OctalCode(pub u8, pub u8, pub u8, pub u8);

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct M3aRecord {
    m3a_code_record: u16,
}

impl M3aRecord {
    pub fn set(&mut self, m3a_code_record: u16) {
        self.m3a_code_record = m3a_code_record;
    }

    pub fn get(&self) -> u16 {
        self.m3a_code_record
    }

    pub fn set_code(&mut self, code: OctalCode) {
        // Set all m3a code bits to zero
        self.m3a_code_record = self.m3a_code_record & !M3aRecord::CODE_MASK;

        // Concatenate code bits
        let element_1 = code.0 as u16;
        let element_2 = (code.1 as u16) << 3;
        let element_3 = (code.2 as u16) << 6;
        let element_4 = (code.3 as u16) << 9;
        let code = element_1 | element_2 | element_3 | element_4;

        // Set m3a code bits
        self.m3a_code_record = self.m3a_code_record | code;
    }

    pub fn get_code(&self) -> OctalCode {
        // Get all m3a code bits
        let code = self.m3a_code_record & M3aRecord::CODE_MASK;

        // Separate code bits
        let element_1 = code & M3aRecord::CODE_ELEMENT_1;
        let element_2 = (code & M3aRecord::CODE_ELEMENT_2) >> 3;
        let element_3 = (code & M3aRecord::CODE_ELEMENT_3) >> 6;
        let element_4 = (code & M3aRecord::CODE_ELEMENT_4) >> 9;

        // Set OctalCode
        let code = OctalCode(element_1 as u8, element_2 as u8, element_3 as u8, element_4 as u8);
        code
    }

    pub fn set_on_off(&mut self, state: bool) {
        if state {
            self.m3a_code_record = self.m3a_code_record | M3aRecord::ONOFF_STATUS;
        } else {
            self.m3a_code_record = self.m3a_code_record & (!M3aRecord::ONOFF_STATUS);
        }
    }

    pub fn get_on_off(&self) -> bool {
        let field = self.m3a_code_record & M3aRecord::ONOFF_STATUS;
        if field == M3aRecord::ONOFF_STATUS {
            true
        } else {
            false
        }
    }

    pub fn set_damage(&mut self, state: bool) {
        if state {
            self.m3a_code_record = self.m3a_code_record | M3aRecord::DAMAGE_STATUS;
        } else {
            self.m3a_code_record = self.m3a_code_record & (!M3aRecord::DAMAGE_STATUS);
        }
    }

    pub fn get_damage(&self) -> bool {
        let field = self.m3a_code_record & M3aRecord::DAMAGE_STATUS;
        if field == M3aRecord::DAMAGE_STATUS {
            true
        } else {
            false
        }
    }

    pub fn set_malfunction(&mut self, state: bool) {
        if state {
            self.m3a_code_record = self.m3a_code_record | M3aRecord::MALFUNCTION_STATUS;
        } else {
            self.m3a_code_record = self.m3a_code_record & (!M3aRecord::MALFUNCTION_STATUS);
        }
    }

    pub fn get_malfunction(&self) -> bool {
        let field = self.m3a_code_record & M3aRecord::MALFUNCTION_STATUS;
        if field == M3aRecord::MALFUNCTION_STATUS {
            true
        } else {
            false
        }
    }

    const CODE_ELEMENT_1: u16 = 0b0000_0000_0000_0111;
    const CODE_ELEMENT_2: u16 = 0b0000_0000_0011_1000;
    const CODE_ELEMENT_3: u16 = 0b0000_0001_1100_0000;
    const CODE_ELEMENT_4: u16 = 0b0000_1110_0000_0000;
    const CODE_MASK: u16 = 0b0000_1111_1111_1111;

    const ONOFF_STATUS: u16 = 0b0010_0000_0000_0000;
    const DAMAGE_STATUS: u16 = 0b0100_0000_0000_0000;
    const MALFUNCTION_STATUS: u16 = 0b1000_0000_0000_0000;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code() {
        let mut m3a = M3aRecord::default();
        m3a.set_code(OctalCode(1, 2, 3, 4));

        assert_eq!(m3a.get(), 0b0000_1000_1101_0001);

        let code = m3a.get_code();
        assert_eq!(code, OctalCode(1, 2, 3, 4));
    }

    #[test]
    fn on_off_state() {
        let mut m3a = M3aRecord::default();
        m3a.set_on_off(true);

        assert_eq!(m3a.get(), M3aRecord::ONOFF_STATUS);
    }

    #[test]
    fn damage_state() {
        let mut m3a = M3aRecord::default();
        m3a.set_damage(true);

        assert_eq!(m3a.get(), M3aRecord::DAMAGE_STATUS);
    }

    #[test]
    fn malfunction_state() {
        let mut m3a = M3aRecord::default();
        m3a.set_malfunction(true);

        assert_eq!(m3a.get(), M3aRecord::MALFUNCTION_STATUS);
    }
}
