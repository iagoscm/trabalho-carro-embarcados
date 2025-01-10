#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Car {
    Seta = 0x00,
    Cruise = 0x01,
    Farol = 0x02;
}

