mod iff_mode_3a;

#[cfg(test)]
mod tests {
    use super::*;
    use iff_mode_3a::{OctalCode, M3aRecord};

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
}
