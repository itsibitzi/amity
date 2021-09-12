use super::command::SDMMCCommand;

pub struct SDMMCDriver {}

impl SDMMCDriver {
    pub fn new() -> SDMMCDriver {
        SDMMCDriver {}
    }

    // This is copied from the internet - i don't really understand how it relates to
    // the CRC defined in the spec...
    // Spec can be found https://elinux.org/images/d/d3/Mmc_spec.pdf page 50
    // Apparently a lot of folks just don't send a crc?
    fn crc7(data: &[u8]) -> u8 {
        let mut crc: u8 = 0;
        for mut byte in data.iter().cloned() {
            for _ in 0..8 {
                crc <<= 1;
                if byte & 0x80 ^ (crc & 0x80) != 0 {
                    crc ^= 0x09;
                }
                byte <<= 1;
            }
        }
        return (crc << 1) | 1;
    }

    pub fn command(&self, cmd: SDMMCCommand, arg: i32) {
        log::debug!("SD/MMC command={} arg={}", cmd, arg);
        let arg_bytes = arg.to_be_bytes();
        let mut buf: [u8; 6] = [0; 6];
        buf[0] = cmd.0 | 0x40;
        buf[1] = arg_bytes[0];
        buf[2] = arg_bytes[1];
        buf[3] = arg_bytes[2];
        buf[4] = arg_bytes[3];
        buf[5] = SDMMCDriver::crc7(&buf[0..5]);
    }
}
