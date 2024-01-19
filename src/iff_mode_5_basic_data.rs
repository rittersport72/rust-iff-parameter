use std::mem;
use m5_status::M5StatusRecord;
use crate::iff_mode_1e::M1eRecord;

pub mod m5_status;

#[repr(packed(1))]
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct M5Record {
    m5_status: M5StatusRecord,
    pin: u16,
    m5_message_formats: u32,
    enhanced_mode_1: M1eRecord,
    national_origin: u16,
    supplemental_data: u8,
    navigation_source: u8,
    figure_of_merit: u8,
    padding: u8,
}

impl M5Record {
    /*
     * Convert byte stream to struct. This uses unsafe.
     */
    pub fn from_bytes(&mut self, array: &[u8; M5Record::MESSAGE_LENGTH]) {
        unsafe {
            *self = mem::transmute_copy::<[u8; M5Record::MESSAGE_LENGTH], M5Record>(array);
        }
    }

    /*
     * Convert struct to byte stream. This uses unsafe.
     */
    pub fn to_bytes(&self) -> [u8; M5Record::MESSAGE_LENGTH] {
        unsafe { mem::transmute_copy::<M5Record, [u8; M5Record::MESSAGE_LENGTH]>(self) }
    }

    /*
     * Create fixed length array from slice.
     */
    pub fn array_of_byte_message(array: &[u8]) -> [u8; M5Record::MESSAGE_LENGTH] {
        array.try_into().expect("slice with incorrect length")
    }

    pub fn set_m5_status(&mut self, status: M5StatusRecord) {
        self.m5_status = status;
    }

    pub fn get_m5_status(&self) -> M5StatusRecord {
        return self.m5_status;
    }

    pub fn set_pin(&mut self, pin: u16) {
        self.pin = pin.to_be();
    }

    pub fn get_pin(&self) -> u16 {
        return u16::from_be(self.pin);
    }

    pub fn set_message_formats(&mut self, message_formats: u32) {
        self.m5_message_formats = message_formats.to_be();
    }

    pub fn get_message_formats(&self) -> u32 {
        return u32::from_be(self.m5_message_formats);
    }

    pub fn set_enhanced_mode_1(&mut self, enhanced_mode_1: M1eRecord) {
        self.enhanced_mode_1 = enhanced_mode_1;
    }

    pub fn get_enhanced_mode_1(&self) -> M1eRecord {
        return self.enhanced_mode_1;
    }

    pub fn set_national_origin(&mut self, national_origin: u16) {
        self.national_origin = national_origin.to_be();
    }

    pub fn get_national_origin(&self) -> u16 {
        return u16::from_be(self.national_origin);
    }

    pub fn set_supplemental_data(&mut self, supplemental_data: u8) {
        self.supplemental_data = supplemental_data.to_be();
    }

    pub fn get_supplemental_data(&self) -> u8 {
        return u8::from_be(self.supplemental_data);
    }

    pub fn set_navigation_source(&mut self, navigation_source: u8) {
        self.navigation_source = navigation_source.to_be();
    }

    pub fn get_navigation_source(&self) -> u8 {
        return u8::from_be(self.navigation_source);
    }

    pub fn set_figure_merit(&mut self, figure_of_merit: u8) {
        self.figure_of_merit = figure_of_merit.to_be();
    }

    pub fn get_figure_merit(&self) -> u8 {
        return u8::from_be(self.figure_of_merit);
    }

    /*
     * Message length in memory.
     */
    const MESSAGE_LENGTH: usize = mem::size_of::<Self>();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iff_mode_123::OctalCode;

    #[test]
    fn m5_status() {
        let mut m5_status = M5StatusRecord::default();
        m5_status.set_on_off(true);
        m5_status.set_damage(true);
        m5_status.set_malfunction(true);

        let mut m5 = M5Record::default();
        m5.set_m5_status(m5_status);

        // Convert struct to byte stream
        let array = m5.to_bytes();
        assert_eq!(array[1], 224);

        // New message
        let mut object = M5Record::default();

        // Convert byte stream to struct
        object.from_bytes(&array);

        assert_eq!(m5.get_m5_status(), object.get_m5_status());
    }

    #[test]
    fn pin() {
        let mut m5 = M5Record::default();
        m5.set_pin(42);

        // Convert struct to byte stream
        let array = m5.to_bytes();
        assert_eq!(array[3], 42);

        // New message
        let mut object = M5Record::default();

        // Convert byte stream to struct
        object.from_bytes(&array);

        assert_eq!(m5.get_pin(), object.get_pin());
    }

    #[test]
    fn enhanced_mode_1() {
        let mut m1e = M1eRecord::default();
        m1e.set_code(OctalCode(1, 2, 3, 4));

        let mut m5 = M5Record::default();
        m5.set_enhanced_mode_1(m1e);

        // Convert struct to byte stream
        let array = m5.to_bytes();
        assert_eq!(array[8], 209);
        assert_eq!(array[9], 8);

        // New message
        let mut object = M5Record::default();

        // Convert byte stream to struct
        object.from_bytes(&array);

        assert_eq!(m5.get_enhanced_mode_1(), object.get_enhanced_mode_1());
    }

    #[test]
    fn national_origin() {
        let mut m5 = M5Record::default();
        m5.set_national_origin(42);

        // Convert struct to byte stream
        let array = m5.to_bytes();
        assert_eq!(array[11], 42);

        // New message
        let mut object = M5Record::default();

        // Convert byte stream to struct
        object.from_bytes(&array);

        assert_eq!(m5.get_national_origin(), object.get_national_origin());
    }
}
