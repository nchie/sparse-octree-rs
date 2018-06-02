use sparse_octree::LocationalCode;

#[derive(Debug, Copy, Clone)]
pub enum ChildCode {
    BLF = 0b000,
    BRF = 0b001,
    BLB = 0b010,
    BRB = 0b011,

    TLF = 0b100,
    TRF = 0b101,
    TLB = 0b110,
    TRB = 0b111
}

impl From<u64> for ChildCode {
    fn from(mut from: u64) -> ChildCode {
        from &= 0b111;
        match from {
            0b000 => ChildCode::BLF,
            0b001 => ChildCode::BRF,
            0b010 => ChildCode::BLB,
            0b011 => ChildCode::BRB,

            0b100 => ChildCode::TLF,
            0b101 => ChildCode::TRF,
            0b110 => ChildCode::TLB,
            0b111 => ChildCode::TRB,
            _ => panic!("This can't happen!")
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum ChildrenFlags {
    BLF = 0b0000_0001,
    BRF = 0b0000_0010,
    BLB = 0b0000_0100,
    BRB = 0b0000_1000,

    TLF = 0b0001_0000,
    TRF = 0b0010_0000,
    TLB = 0b0100_0000,
    TRB = 0b1000_0000
}