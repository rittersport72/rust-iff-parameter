
/// 4-bit enumeration for reply of mode 5 transponder if interrogated
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Mode5Reply {
    NoResponse,
    Valid,
    Invalid,
    UnableToVerify,
}

/// 2-bit enumeration for antenna selection
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum AntennaSelection {
    NoStatement,
    Top,
    Bottom,
    Diversity,
}

/// Mode 5 transponder status information
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

    pub fn set_mode5_reply(&mut self, reply: Mode5Reply) {
        // Set all mode5 reply bits to zero
        self.m5_status_record = self.m5_status_record & !Self::MODE5_REPLY;

        let field = reply as u16;

        // Set mode5 reply bits
        self.m5_status_record = self.m5_status_record | field;
    }

    pub fn get_mode5_reply(&self) -> Mode5Reply {
        // Get all mode5 reply bits
        let field = self.m5_status_record & Self::MODE5_REPLY;

        match field {
            0 => Mode5Reply::NoResponse,
            1 => Mode5Reply::Valid,
            2 => Mode5Reply::Invalid,
            3 => Mode5Reply::UnableToVerify,
            _ => Mode5Reply::NoResponse,
        }
    }

    pub fn set_antenna_selection(&mut self, select: AntennaSelection) {
        // Set all antenna selection bits to zero
        self.m5_status_record = self.m5_status_record & !Self::ANTENNA_SELECTION;

        let field = (select as u16) << 5;

        // Set antenna selection bits
        self.m5_status_record = self.m5_status_record | field;
    }

    pub fn get_antenna_selection(&self) -> AntennaSelection {
        // Get all antenna selection bits
        let field = self.m5_status_record & Self::ANTENNA_SELECTION;

        match field {
            0 => AntennaSelection::NoStatement,
            1 => AntennaSelection::Top,
            2 => AntennaSelection::Bottom,
            3 => AntennaSelection::Diversity,
            _ => AntennaSelection::NoStatement,
        }
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

    const MODE5_REPLY: u16 = 0b0000_0000_0000_1111;
    const ANTENNA_SELECTION: u16 = 0b0000_0000_0110_0000;
    const ONOFF_STATUS: u16 = 0b0010_0000_0000_0000;
    const DAMAGE_STATUS: u16 = 0b0100_0000_0000_0000;
    const MALFUNCTION_STATUS: u16 = 0b1000_0000_0000_0000;
}
