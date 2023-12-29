mod iff_mode_3a;
mod iff_mode_5_basic_data;

#[cfg(test)]
mod tests {
    use super::*;
    use iff_mode_3a::{OctalCode, M3aRecord};
    use iff_mode_5_basic_data::M5Record;

    #[test]
    fn test_m3a_record() {
        let mut m3a = M3aRecord::default();
        m3a.set(0b0101_0101_0101_0101);
        m3a.set_code(OctalCode(1, 2, 3, 4));
        m3a.set_on_off(true);
        m3a.set_damage(true);
        m3a.set_malfunction(true);

        assert_eq!(m3a.get_code(), OctalCode(1, 2, 3, 4));
        assert_eq!(m3a.get_on_off(), true);
        assert_eq!(m3a.get_damage(), true);
        assert_eq!(m3a.get_malfunction(), true);
    }

    #[test]
    fn test_m5_record() {
        let mut m5 = M5Record::default();
        m5.set_pin(42);
        m5.set_enhanced_mode_1(26);
        m5.set_national_origin(711);

        assert_eq!(m5.get_pin(), 42);
        assert_eq!(m5.get_enhanced_mode_1(), 26);
        assert_eq!(m5.get_national_origin(), 711);
    }
}
