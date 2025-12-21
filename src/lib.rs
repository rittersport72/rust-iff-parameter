pub mod iff_mode_123;
pub mod iff_mode_1e;
pub mod iff_mode_2;
pub mod iff_mode_3a;
pub mod iff_mode_5_basic_data;
pub mod iff_mode_c;

#[cfg(test)]
mod tests {
    use super::*;
    use iff_mode_1e::M1eRecord;
    use iff_mode_123::{M123Record, octal_4_digit};
    use iff_mode_5_basic_data::M5Record;
    use iff_mode_c::MCRecord;

    #[test]
    fn test_m3a_record() {
        let mut octal_code = octal_4_digit::Octal4Digit::default();
        octal_code.set((1, 2, 3, 4));

        let mut m123 = M123Record::default();
        m123.set(0b0101_0101_0101_0101);
        m123.set_code(octal_code);
        m123.set_on_off(true);
        m123.set_damage(true);
        m123.set_malfunction(true);

        let mut octal_code = octal_4_digit::Octal4Digit::default();
        octal_code.set((1, 2, 3, 4));

        assert_eq!(m123.get_code(), octal_code);
        assert_eq!(m123.get_on_off(), true);
        assert_eq!(m123.get_damage(), true);
        assert_eq!(m123.get_malfunction(), true);
    }

    #[test]
    fn test_m5_record() {
        let mut octal_code = octal_4_digit::Octal4Digit::default();
        octal_code.set((1, 2, 3, 4));

        let mut m1e = M1eRecord::default();
        m1e.set_code(octal_code);

        let mut m5 = M5Record::default();
        m5.set_pin(42);
        m5.set_enhanced_mode_1(m1e);
        m5.set_national_origin(711);

        assert_eq!(m5.get_pin(), 42);
        assert_eq!(m5.get_enhanced_mode_1(), m1e);
        assert_eq!(m5.get_national_origin(), 711);
    }

    #[test]
    fn test_mc_record() {
        let mut mc = MCRecord::default();
        mc.set(0b0101_0101_0101_0101);
        mc.set_altitude_msl(true);
        mc.set_altitude(42);
        mc.set_on_off(true);
        mc.set_damage(true);
        mc.set_malfunction(true);

        assert_eq!(mc.get_altitude_msl(), true);
        assert_eq!(mc.get_altitude(), 42);
        assert_eq!(mc.get_on_off(), true);
        assert_eq!(mc.get_damage(), true);
        assert_eq!(mc.get_malfunction(), true);
    }
}
