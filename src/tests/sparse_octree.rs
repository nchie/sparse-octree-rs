
#[test]
fn set_and_get_node() {
    use sparse_octree::{ ChildId, SparseOctree, Node, NodeLocation };


    let mut octree = SparseOctree::<u64>::new();
    let location1 = NodeLocation::new_root() //0b1_000_000
        .child(ChildId::BLF).unwrap()
        .child(ChildId::BLF).unwrap();
    let location2 = NodeLocation::new_root() //0b1_000_001_111
        .child(ChildId::BLF).unwrap()
        .child(ChildId::BRF).unwrap()
        .child(ChildId::TRB).unwrap(); 

    octree.set(location1, 1).unwrap();
    octree.set(location2, 2).unwrap();
    
    // Succeeds
    assert!(octree.get_single(location1) == Some(&1));
    assert!(octree.get_single(location2) == Some(&2));

    // Fails because the parent is a leaf
    let child_location = location1.child(ChildId::BLB).unwrap();
    assert!(octree.set(child_location, 0) == Err(()));

    // Make sure their common branch has set its child flags correctly
    let child_id1 = location1.child_id();
    let child_id2 = location2.parent().unwrap().child_id();
    let common_parent_location = location1.parent().unwrap();
    assert!(octree.get_node(common_parent_location) == Some(&Node::Branch(child_id1.flag() | child_id2.flag())));
}

#[test]
pub fn sort_count_and_slice() {
    use sparse_octree::{ SparseOctree, Node, NodeLocation };
    let mut octree = SparseOctree::<&str>::new();
    let root = NodeLocation::new_root();

    octree.set(root
                .child(0b000.into()).unwrap(), "1 000").unwrap();

    octree.set(root
                .child(0b001.into()).unwrap()
                    .child(0b000.into()).unwrap(), "1 001 000").unwrap();
    octree.set(root
                .child(0b001.into()).unwrap()
                    .child(0b001.into()).unwrap(), "1 001 001").unwrap();

    octree.set(root
                .child(0b010.into()).unwrap(), "1 010").unwrap();
    octree.set(root
                .child(0b011.into()).unwrap(), "1 011").unwrap();

    octree.set(root
                .child(0b111.into()).unwrap()
                    .child(0b000.into()).unwrap(), "1 111 000").unwrap();

    octree.set(root
                .child(0b100.into()).unwrap()
                    .child(0b001.into()).unwrap()
                        .child(0b001.into()).unwrap(), "1 100 001 001").unwrap();

    assert_eq!(octree.get_slice(root), None); // Expect none if unsorted

    octree.sort();

    let search_node = root.child(111.into()).unwrap();
    assert_eq!(octree.get_slice(root).unwrap().len(), octree.len());
    assert_eq!(octree.get_slice(search_node).unwrap(), &[Node::Branch(1), Node::Leaf("1 111 000")]);
    //assert!(false);
}

#[test]
pub fn sort() {
    use sparse_octree::{ ChildId, SparseOctree, NodeLocation };

    let mut octree = SparseOctree::<u64>::new();
    let parent_location = NodeLocation::new_root();

    let location1 = parent_location.child(ChildId::BLF).unwrap();
    let location2 = parent_location.child(ChildId::BLB).unwrap();
    let location3 = parent_location.child(ChildId::TLF).unwrap();
    let location4 = parent_location.child(ChildId::TRB).unwrap();
    
    octree.set(location2, 2).unwrap();
    octree.set(location4, 4).unwrap();
    octree.set(location1, 1).unwrap();
    octree.set(location3, 3).unwrap();

    //println!("storage: {:?}, map: {:?}", octree.storage, octree.map);
    octree.sort();
    //println!("storage: {:?}, map: {:?}", octree.storage, octree.map);

    // Test querying
    assert_eq!(octree.get_single(location1).unwrap(), &1u64);
    assert_eq!(octree.get_single(location2).unwrap(), &2u64);
    assert_eq!(octree.get_single(location3).unwrap(), &3u64);
    assert_eq!(octree.get_single(location4).unwrap(), &4u64);
}