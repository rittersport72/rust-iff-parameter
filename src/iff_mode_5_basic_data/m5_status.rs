#[repr(packed(1))]
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct M5StatusRecord {
    m5_status_record: u16,
}

impl M5StatusRecord {
    pub fn set(&mut self, m3a_code_record: u16) {
        self.m5_status_record = m3a_code_record;
    }

    pub fn get(&self) -> u16 {
        self.m5_status_record
    }

    pub fn set_on_off(&mut self, state: bool) {
        if state {
            self.m5_status_record = self.m5_status_record | M5StatusRecord::ONOFF_STATUS;
        } else {
            self.m5_status_record = self.m5_status_record & (!M5StatusRecord::ONOFF_STATUS);
        }
    }

    pub fn get_on_off(&self) -> bool {
        let field = self.m5_status_record & M5StatusRecord::ONOFF_STATUS;
        if field == M5StatusRecord::ONOFF_STATUS {
            true
        } else {
            false
        }
    }

    pub fn set_damage(&mut self, state: bool) {
        if state {
            self.m5_status_record = self.m5_status_record | M5StatusRecord::DAMAGE_STATUS;
        } else {
            self.m5_status_record = self.m5_status_record & (!M5StatusRecord::DAMAGE_STATUS);
        }
    }

    pub fn get_damage(&self) -> bool {
        let field = self.m5_status_record & M5StatusRecord::DAMAGE_STATUS;
        if field == M5StatusRecord::DAMAGE_STATUS {
            true
        } else {
            false
        }
    }

    pub fn set_malfunction(&mut self, state: bool) {
        if state {
            self.m5_status_record = self.m5_status_record | M5StatusRecord::MALFUNCTION_STATUS;
        } else {
            self.m5_status_record = self.m5_status_record & (!M5StatusRecord::MALFUNCTION_STATUS);
        }
    }

    pub fn get_malfunction(&self) -> bool {
        let field = self.m5_status_record & M5StatusRecord::MALFUNCTION_STATUS;
        if field == M5StatusRecord::MALFUNCTION_STATUS {
            true
        } else {
            false
        }
    }

    const ONOFF_STATUS: u16 = 0b0010_0000_0000_0000;
    const DAMAGE_STATUS: u16 = 0b0100_0000_0000_0000;
    const MALFUNCTION_STATUS: u16 = 0b1000_0000_0000_0000;
}
