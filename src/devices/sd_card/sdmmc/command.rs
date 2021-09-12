use core::fmt::{Display, Error, Formatter};

pub struct SDMMCCommand(pub u8);

impl SDMMCCommand {
    pub const GO_IDLE_STATE: SDMMCCommand = SDMMCCommand(0x00);
    pub const SEND_IF_COND: SDMMCCommand = SDMMCCommand(0x08);
}

impl Display for SDMMCCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let name = match *self {
            SDMMCCommand::GO_IDLE_STATE => "GO_IDLE_STATE",
        };
        write!(f, "{}", name)
    }
}
