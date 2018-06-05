#[derive(Debug, Copy, Clone)]
pub enum ChildId {
    BLF = 0,
    BRF = 1,
    BLB = 2,
    BRB = 3,

    TLF = 4,
    TRF = 5,
    TLB = 6,
    TRB = 7
}

impl ChildId {
    pub fn flag(&self) -> u8 {
        1 << *self as u8
    }
}

impl From<u64> for ChildId {
    fn from(mut from: u64) -> ChildId {
        from &= 0b111;
        match from {
            0b000 => ChildId::BLF,
            0b001 => ChildId::BRF,
            0b010 => ChildId::BLB,
            0b011 => ChildId::BRB,

            0b100 => ChildId::TLF,
            0b101 => ChildId::TRF,
            0b110 => ChildId::TLB,
            0b111 => ChildId::TRB,
            _ => panic!("This can't happen!")
        }
    }
}
