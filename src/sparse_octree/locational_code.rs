use sparse_octree::ChildCode;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct LocationalCode(u64);

const MAX_DEPTH: u32 = 21;

impl LocationalCode {
    pub fn new(x: u32, y: u32, z: u32) -> LocationalCode {
        // TODO: Implement in some smart way
        LocationalCode(1)
    }

    pub fn parent(&self) -> Option<LocationalCode> {
        match self.depth() {
            1 => None,
            _ => Some(LocationalCode(self.0 >> 3))
        }
    }

    pub fn child(&self, code: ChildCode) -> Option<LocationalCode> {
        match self.depth() {
            depth if depth >= MAX_DEPTH => None,
            _ => Some(LocationalCode((self.0 << 3) | code as u64))
        }
    }

    pub fn depth(&self) -> u32 {
        let mut code = self.0;
        let mut depth = 0;

        while code > 1 // Make sure code can NEVER be 0!
        {
            code >>= 3;
            depth += 1;
        };  

        depth
    }

    pub fn disown(&self) -> (Option<LocationalCode>, ChildCode) {
        (self.parent(), ChildCode::from(self.0))
    }
}

impl From<LocationalCode> for ChildCode {
    fn from(from: LocationalCode) -> ChildCode {
        from.0.into()
    }
}

#[test]
pub fn depth_tests() {
    let shallow_location = LocationalCode(0b1_101_000);
    assert_eq!(shallow_location.depth(), 2);
    assert_eq!(shallow_location.parent().unwrap().depth(), 1);
    assert_eq!(shallow_location.child(ChildCode::BLB).unwrap().depth(), 3);

    let deep_location = LocationalCode(0x0800_0000_0000_0000);
    assert_eq!(deep_location.depth(), 20);
}

#[test]
pub fn parent_tests() {
    let location = LocationalCode(0b1_101_000);
    let parent = location.parent();
    assert_eq!(parent, Some(LocationalCode(0b1_101)));

    let grandparent = parent.unwrap().parent();
    assert_eq!(grandparent, None);
}

#[test]
pub fn child_tests() {
    let location = LocationalCode(0x0800_0000_0000_0000);

    let child = location.child(ChildCode::BLF);
    assert_eq!(child, Some(LocationalCode(0x4000_0000_0000_0000)));

    let grandchild = child.unwrap().child(ChildCode::BLF);
    assert_eq!(grandchild, None);
}