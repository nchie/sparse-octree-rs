use sparse_octree::ChildId;
use sparse_octree::SparseOctree;
use sparse_octree::NodeLocation;



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
    let root = NodeLocation::new_root();

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

