use sparse_octree::ChildId;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct NodeLocation { code: u64 }

const MAX_DEPTH: u32 = 21;

impl NodeLocation {
    pub fn new(mut x: i32, mut y: i32, mut z: i32, depth: u32) -> Option<NodeLocation> {
        let mut max = 2i32.pow(depth as u32)/2; // Max dimension for given depth
        if  x >= max || x < -max ||
            y >= max || y < -max ||
            z >= max || z < -max {
            // x, y and z have to be within the max dimensions for given depth
            return None;
        }

        // u64 only supports a depth of up to 21
        if depth > MAX_DEPTH {
            return None;
        }

        let mut code = 1u64; // Start from root
        for _ in 0..depth {
            code <<= 3; // Shift 3 for every depth level
            max /= 2; // Every depth level halves max dimensions

            // Fill in code for current child
            if x >= 0 { code |= 0b001 }; // Positive x has bit 0 set
            if y >= 0 { code |= 0b100 }; // Positive y has bit 2 set
            if z >= 0 { code |= 0b010 }; // Positive z has bit 1 set

            // Shift the relevant octant to the center
            if x >= 0 { x -= max } else { x += max }
            if y >= 0 { y -= max } else { y += max }
            if z >= 0 { z -= max } else { z += max }
        }
        Some(NodeLocation{ code })
    }

    pub fn new_root() -> NodeLocation {
        NodeLocation{ code: 1 }
    }

    pub fn parent(&self) -> Option<NodeLocation> {
        // If self.code is 1, we're already at the root node
        match self.code {
            1 => None,
            _ => Some(NodeLocation{ code: self.code >> 3 })
        }
    }

    pub fn child(&self, child: ChildId) -> Option<NodeLocation> {
        match self.depth() {
            depth if depth >= MAX_DEPTH => None,
            _ => Some(NodeLocation{ code: (self.code << 3) | child as u64 })
        }
    }

    pub fn child_id(&self) -> ChildId {
        ChildId::from(self.code)
    }

    pub fn coordinates(&self) -> (i32, i32, i32, u32) {
        let (mut x, mut y, mut z, mut depth) = (0, 0, 0, 0);
        let mut code = self.code;

        while code > 1 {
            let max = 2i32.pow(depth) / 2;
            let mut min = max;
            // If last iteration is negative, we should subtract an extra because of the '>=' in the coord generation
            if max < 1 { min = 1 }

            if (code&0b001)>0 { x += max } else { x -= min } // If 1st bit is set, x is positive
            if (code&0b100)>0 { y += max } else { y -= min } // If 3rd bit is set, y is positive
            if (code&0b010)>0 { z += max } else { z -= min } // If 2nd bit is set, z is positive


            code >>= 3;    
            depth += 1;
        }

        (x, y, z, depth)
    }

    pub fn depth(&self) -> u32 {
        // In the case of u64, subtracting max depth with (leading zeros+1)/3 will give current depth 
        // and should be quicker than shifting
        MAX_DEPTH-((self.code.leading_zeros()+1)/3)
    }

    pub fn disown(&self) -> (Option<NodeLocation>, ChildId) {
        (self.parent(), self.child_id())
    }
}

impl From<NodeLocation> for ChildId {
    fn from(from: NodeLocation) -> ChildId {
        from.code.into()
    }
}

impl Ord for NodeLocation {
    // This way of ordering locations will sort them "depth-first" and thus we can easily hand out octants as slices when queried a location!
    fn cmp(&self, other: &NodeLocation) -> Ordering {
        // Count leading zeros for both
        let self_lzc = self.code.leading_zeros();
        let other_lzc = other.code.leading_zeros();

        // Shift them so that the leading root bit is the most significant
        let self_shifted = self.code << self_lzc;
        let other_shifted = other.code << other_lzc;
        
        if self_shifted == other_shifted {
            // If they're equal once shifted (which only happens when the code only has a single 1), we'll sort by leading amount of zeros instead
            other_lzc.cmp(&self_lzc)
        } else {
            // Else, go by the shifted values
            self_shifted.cmp(&other_shifted)
        }
    }
}

impl PartialOrd for NodeLocation {
    fn partial_cmp(&self, other: &NodeLocation) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for NodeLocation {
    fn eq(&self, other: &NodeLocation) -> bool {
        self.code == other.code
    }
}

#[test]
pub fn depth_tests() {
    let shallow_location = NodeLocation{ code: 0b1_101_000};
    assert_eq!(shallow_location.depth(), 2);
    assert_eq!(shallow_location.parent().unwrap().depth(), 1);
    assert_eq!(shallow_location.child(ChildId::BLB).unwrap().depth(), 3);

    let deep_location = NodeLocation{ code: 0x0800_0000_0000_0000 };
    assert_eq!(deep_location.depth(), 20);
}

#[test]
pub fn parent_tests() {
    let location = NodeLocation{ code: 0b1_101_000 };
    let parent = location.parent();
    assert_eq!(parent, Some(NodeLocation{ code: 0b1_101 }));

    let grandparent = parent.unwrap().parent().unwrap();
    assert_eq!(grandparent, NodeLocation::new_root());

    assert_eq!(grandparent.parent(), None);
}

#[test]
pub fn child_tests() {
    let location = NodeLocation{ code: 0x0800_0000_0000_0000 };

    let child = location.child(ChildId::BLF);
    assert_eq!(child, Some(NodeLocation { code: 0x4000_0000_0000_0000 }));

    let grandchild = child.unwrap().child(ChildId::BLF);
    assert_eq!(grandchild, None);
}



