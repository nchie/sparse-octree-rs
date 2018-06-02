use sparse_octree::ChildId;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct NodeLocation(u64);

const MAX_DEPTH: u32 = 21;

impl NodeLocation {
    pub fn new(mut x: i32, mut y: i32, mut z: i32, depth: u32) -> Option<NodeLocation> {
        let max = 2i32.pow(depth as u32)/2; // Max dimension for given depth
        if  x >= max || x < -max ||
            y >= max || y < -max ||
            z >= max || z < -max {
            // x, y and z have to be within the max dimensions for this depth
            return None;
        }

        // u64 only supports up to 21 in depth
        if depth > MAX_DEPTH {
            return None;
        }

        let mut code = 1u64; // Start from root
        for depth_level in (0..depth).rev() {
            code <<= 3; // Shift 3 for every depth level

            // Get the 
            let max = 2i32.pow(depth_level) / 2; // Should be cheap as it's basically just a lshift and division

            // Fill in code for current child
            if x >= 0 { code |= 0b001 }; // Positive x has bit 0 set
            if y >= 0 { code |= 0b100 }; // Positive y has bit 2 set
            if z >= 0 { code |= 0b010 }; // Positive z has bit 1 set

            // Shift the relevant 1/8th of the square to the middle
            if x >= 0 { x -= max } else { x += max }
            if y >= 0 { y -= max } else { y += max }
            if z >= 0 { z -= max } else { z += max }
        }
        Some(NodeLocation(code))
    }

    // TODO: Remove
    pub fn new_root() -> NodeLocation {
        NodeLocation(1)
    }

    pub fn parent(&self) -> Option<NodeLocation> {
        match self.depth() {
            1 => None,
            _ => Some(NodeLocation(self.0 >> 3))
        }
    }

    pub fn child(&self, code: ChildId) -> Option<NodeLocation> {
        match self.depth() {
            depth if depth >= MAX_DEPTH => None,
            _ => Some(NodeLocation((self.0 << 3) | code as u64))
        }
    }

    pub fn child_id(&self) -> ChildId {
        ChildId::from(self.0)
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

    pub fn disown(&self) -> (Option<NodeLocation>, ChildId) {
        (self.parent(), self.child_id())
    }
}

impl From<NodeLocation> for ChildId {
    fn from(from: NodeLocation) -> ChildId {
        from.0.into()
    }
}

#[test]
pub fn depth_tests() {
    let shallow_location = NodeLocation(0b1_101_000);
    assert_eq!(shallow_location.depth(), 2);
    assert_eq!(shallow_location.parent().unwrap().depth(), 1);
    assert_eq!(shallow_location.child(ChildId::BLB).unwrap().depth(), 3);

    let deep_location = NodeLocation(0x0800_0000_0000_0000);
    assert_eq!(deep_location.depth(), 20);
}

#[test]
pub fn parent_tests() {
    let location = NodeLocation(0b1_101_000);
    let parent = location.parent();
    assert_eq!(parent, Some(NodeLocation(0b1_101)));

    let grandparent = parent.unwrap().parent();
    assert_eq!(grandparent, None);
}

#[test]
pub fn child_tests() {
    let location = NodeLocation(0x0800_0000_0000_0000);

    let child = location.child(ChildId::BLF);
    assert_eq!(child, Some(NodeLocation(0x4000_0000_0000_0000)));

    let grandchild = child.unwrap().child(ChildId::BLF);
    assert_eq!(grandchild, None);
}


#[test]
pub fn depth_max_dimensions() {
    // Depth of 1 allows coordinates between -1 to 1-1
    assert_ne!(NodeLocation::new(-1, 0, -1, 1), None); // Succeeds
    assert_eq!(NodeLocation::new(-2, 1, -1, 1), None); // Fails

    // Depth of 2 allows coordinates between -2 to 2-1
    assert_ne!(NodeLocation::new(-2, 1, -2, 2), None); // Succeeds
    assert_eq!(NodeLocation::new(-2, 2, -2, 2), None); // Fails

    // Depth of 4 allows coordinates between -8 to 8-1
    assert_ne!(NodeLocation::new(7, 7, -8, 4), None); // Succeeds
    assert_eq!(NodeLocation::new(-7, 9, 7, 4), None); // Fails

    // Depth of 21 allows coordinates from -1048576 to 1048576-1
    assert_ne!(NodeLocation::new(1048575, -1048576, 1048575, 21), None); // Succeeds
    assert_eq!(NodeLocation::new(1048577, -1048576, 1048576, 21), None); // Fails
}

#[test]
pub fn create_from_coordinates() {
    let root = NodeLocation(0b1);

    // (3, 3, 3) @ d3 should be top right back > top right back
    assert_eq!(NodeLocation::new(3, 3, 3, 3).unwrap(), root
        .child(ChildId::TRB).unwrap()
        .child(ChildId::TRB).unwrap()
        .child(ChildId::TRB).unwrap()
    );

    // (-4, -4, -4) @ d3 should be bottom left front > bottom left front
    assert_eq!(NodeLocation::new(-4, -4, -4, 3).unwrap(), root
        .child(ChildId::BLF).unwrap()
        .child(ChildId::BLF).unwrap()
        .child(ChildId::BLF).unwrap()
    );

    assert_eq!(NodeLocation(0b1_111_111_000), root
        .child(ChildId::TRB).unwrap()
        .child(ChildId::TRB).unwrap()
        .child(ChildId::BLF).unwrap()
    );

    // (3, 3, 3) @ d3
    assert_eq!(NodeLocation::new(2, 2, 2, 3).unwrap(), root
        .child(ChildId::TRB).unwrap()
        .child(ChildId::TRB).unwrap()
        .child(ChildId::BLF).unwrap()
    );

    // (-3, -3, -3) @ d3
    assert_eq!(NodeLocation::new(-3, -3, -3, 3).unwrap(), root
        .child(ChildId::BLF).unwrap()
        .child(ChildId::BLF).unwrap()
        .child(ChildId::TRB).unwrap()
    );

    // (0, 0, 0) @ d4 = TOP LEFT BACK -> BOTTOM LEFT FRONT > BOTTOM LEFT FRONT > BOTTOM LEFT FRONT
    assert_eq!(NodeLocation::new(-8, 0, 0, 4).unwrap(), root
        .child(ChildId::TLB).unwrap()
        .child(ChildId::BLF).unwrap()
        .child(ChildId::BLF).unwrap()
        .child(ChildId::BLF).unwrap()
    );

    // (-7, -7, -7) @ d4
    assert_eq!(NodeLocation::new(-7, -7, -7, 4).unwrap(), root
        .child(ChildId::BLF).unwrap()
        .child(ChildId::BLF).unwrap()
        .child(ChildId::BLF).unwrap()
        .child(ChildId::TRB).unwrap()
    );
}



