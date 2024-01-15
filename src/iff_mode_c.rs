#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct MCRecord {
    mc_record: u16,
}

impl MCRecord {
    pub fn set(&mut self, mc_record: u16) {
        self.mc_record = mc_record;
    }

    pub fn get(&self) -> u16 {
        self.mc_record
    }

    /**
     * below_msl = false for altitude above MSL.
     * below_msl = true for altitude below MSL.
     */
    pub fn set_altitude_msl(&mut self, below_msl: bool) {
        if below_msl {
            self.mc_record = self.mc_record | MCRecord::ALTITUDE_MSL;
        } else {
            self.mc_record = self.mc_record & (!MCRecord::ALTITUDE_MSL);
        }
    }

    /**
     * Returns false for altitude above MSL.
     * Returns true for altitude below MSL.
     */
    pub fn get_altitude_msl(&self) -> bool {
        let field = self.mc_record & MCRecord::ALTITUDE_MSL;
        if field == MCRecord::ALTITUDE_MSL {
            true
        } else {
            false
        }
    }

    /**
     * Range from 0 feet to 126000 feet in 100 foot increments.
     */
    pub fn set_altitude(&mut self, altitude: u16) {
        // Set all mc altitude bits to zero
        self.mc_record = self.mc_record & !MCRecord::ALTITUDE_MASK;

        // Shift altitude bits
        let element = altitude << 1;

        // Set mc altitude bits
        self.mc_record = self.mc_record | element;
    }

    /**
     * Returns from 0 feet to 126000 feet in 100 foot increments.
     */
    pub fn get_altitude(&self) -> u16 {
        // Get all mc altitude bits
        let altitude = self.mc_record & MCRecord::ALTITUDE_MASK;

        // Shift altitude bits
        let element = altitude >> 1;
        element
    }

    pub fn set_on_off(&mut self, state: bool) {
        if state {
            self.mc_record = self.mc_record | MCRecord::ONOFF_STATUS;
        } else {
            self.mc_record = self.mc_record & (!MCRecord::ONOFF_STATUS);
        }
    }

    pub fn get_on_off(&self) -> bool {
        let field = self.mc_record & MCRecord::ONOFF_STATUS;
        if field == MCRecord::ONOFF_STATUS {
            true
        } else {
            false
        }
    }

    pub fn set_damage(&mut self, state: bool) {
        if state {
            self.mc_record = self.mc_record | MCRecord::DAMAGE_STATUS;
        } else {
            self.mc_record = self.mc_record & (!MCRecord::DAMAGE_STATUS);
        }
    }

    pub fn get_damage(&self) -> bool {
        let field = self.mc_record & MCRecord::DAMAGE_STATUS;
        if field == MCRecord::DAMAGE_STATUS {
            true
        } else {
            false
        }
    }

    pub fn set_malfunction(&mut self, state: bool) {
        if state {
            self.mc_record = self.mc_record | MCRecord::MALFUNCTION_STATUS;
        } else {
            self.mc_record = self.mc_record & (!MCRecord::MALFUNCTION_STATUS);
        }
    }

    pub fn get_malfunction(&self) -> bool {
        let field = self.mc_record & MCRecord::MALFUNCTION_STATUS;
        if field == MCRecord::MALFUNCTION_STATUS {
            true
        } else {
            false
        }
    }

    const ALTITUDE_MSL: u16 = 0b0000_0000_0000_0001;
    const ALTITUDE_MASK: u16 = 0b0000_1111_1111_1110;
    const ONOFF_STATUS: u16 = 0b0010_0000_0000_0000;
    const DAMAGE_STATUS: u16 = 0b0100_0000_0000_0000;
    const MALFUNCTION_STATUS: u16 = 0b1000_0000_0000_0000;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn altitude_msl() {
        let mut mc = MCRecord::default();
        mc.set_altitude_msl(true);

        assert_eq!(mc.get(), MCRecord::ALTITUDE_MSL);
    }

    #[test]
    fn altitude() {
        let mut mc = MCRecord::default();
        mc.set_altitude(42);

        assert_eq!(mc.get(), 42 << 1);

        let altitude = mc.get_altitude();
        assert_eq!(altitude, 42);
    }

    #[test]
    fn on_off_state() {
        let mut mc = MCRecord::default();
        mc.set_on_off(true);

        assert_eq!(mc.get(), MCRecord::ONOFF_STATUS);
    }

    #[test]
    fn damage_state() {
        let mut mc = MCRecord::default();
        mc.set_damage(true);

        assert_eq!(mc.get(), MCRecord::DAMAGE_STATUS);
    }

    #[test]
    fn malfunction_state() {
        let mut mc = MCRecord::default();
        mc.set_malfunction(true);

        assert_eq!(mc.get(), MCRecord::MALFUNCTION_STATUS);
    }
}
