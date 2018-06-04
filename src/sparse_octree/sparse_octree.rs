use std::collections::HashMap;

use sparse_octree::NodeLocation;

pub struct SparseOctree<T: Clone> {
    storage: Vec<Node<T>>,
    map: HashMap<NodeLocation, usize>,
    max_depth: u32,
    sorted: bool
}

// Public
impl<T: Clone> SparseOctree<T> {
    pub fn new() -> SparseOctree<T> {
        SparseOctree::<T>{
            storage: Vec::new(),
            map: HashMap::<NodeLocation, usize>::new(),
            max_depth: 0,
            sorted: false
        }
    }

    pub fn get_node(&self, location: NodeLocation) -> Option<&T> {
        if let Some(&Node::Leaf(ref node)) = self.get(location) {
            Some( node )
        } else {
            None
        }
    }

    pub fn set_node(&mut self, location: NodeLocation, t: T) -> Result<(), ()>
    {
        self.set(location, Node::Leaf(t))
    }

    pub fn get_nodes(&self, location: NodeLocation) -> Option<&[Node<T>]> {
        if !self.sorted { return None };
        if !self.map.contains_key(&location) { return None } // TODO: Change to return empty slice, or possibly ancestor leaf?

        let index = self.map[&location];
        let count = self.count_from_index(index);
        Some(&self.storage[index..index+count])
    }

    pub fn sort(&mut self) {
        use std;
        // TODO: Improve performance without storing location inside every node
        let mut kv_vec = Vec::with_capacity(self.storage.len());

        // Swap out the old storage with an empty but allocated new one
        let mut old_storage = Vec::with_capacity(self.storage.len());
        std::mem::swap(&mut self.storage, &mut old_storage);

        // Clear the old map into a vec
        for (k, v) in self.map.drain() {
            kv_vec.push((k, v));
        }

        // Sort the vec
        kv_vec.sort_by_key(|kv| { kv.0 });

        // Insert the nodes from old_storage into the new, but in correct order
        for (_, v) in kv_vec.iter() {
            self.storage.push(old_storage[*v].clone()); // TODO: Change to unchecked? Get rid of clone?
        }
        
        // Insert new mappings
        for (i, (k, _)) in kv_vec.iter().enumerate() {
            self.map.insert(*k, i);
        }

        self.sorted = true;
    }
}

// Private
impl<T: Clone> SparseOctree<T> {
    fn count_from_index(&self, index: usize) -> usize {

        match &self.storage[index] {
            Node::Branch(ref f) => {
                let mut count = 1; 

                // Count children and loop through them recursively, incrementing the index by the amount of nodes read
                let bit_count = f.count_ones() as usize;
                for _ in 0..bit_count {
                    count+=self.count_from_index(index+count);
                }
                count
            },
            Node::Leaf(_) => 1
        }
    }

    fn set(&mut self, location: NodeLocation, node: Node<T>) -> Result<(), ()> {
        // Make sure ancestors are pointing towards this (fails if an ancestor is a leaf)
        self.update_ancestors(location)?;

        if location.depth() > self.max_depth { 
            self.max_depth = location.depth() 
        };

        if let Some(index) = self.map.insert(location, self.storage.len()) {
            // If location already had a node, replace it
            self.storage[index] = node;
        } else {
            // Else create room for a new node
            self.storage.push(node);

            // Inserting into the end means we're most likely not sorted anymore.
            self.sorted = false;
        }
        Ok(())
    }

    fn update_ancestors(&mut self, location: NodeLocation) -> Result<(), ()> {
        if let (Some(parent_location), child_id) = location.disown() {
            // If it has a parent
            if let Some(node) = self.get_mut(parent_location) {
                match *node {
                    Node::Branch(ref mut f) => {
                        // Not a first-child, set just set the flag and stop recursing!
                        *f |= child_id.flag(); 
                        Ok(())
                    },
                    Node::Leaf(_) => {
                        // Parent was a leaf, return Err since it's not valid for a leaf to have another leaf as ancestor
                        Err(())
                    }
                }
            } else {
                // If parent didn't exist, set it
                self.set(parent_location, Node::Branch(child_id.flag())) // TODO: Change to actual flag from ChildId
            }
        } else {
            // If no parent, stop recursing
            return Ok(()) 
        }
    }

    fn get(&self, location: NodeLocation) -> Option<&Node<T>> {
        let index = self.map.get(&location);
        match index {
            // If map has an index for the code, a node exists
            Some(&x) => Some( self.storage.get(x).unwrap() ),  // Unwrap should be safe here as it should always exist
            None => None
        }
    }

    fn get_mut(&mut self, location: NodeLocation) -> Option<&mut Node<T>> {
        let index = self.map.get(&location);
        //println!("get location: {:?}", location);
        match index {
            // If map has an index for the location, a node exists
            Some(&x) => Some( self.storage.get_mut(x).unwrap() ),  // Unwrap should be safe here as it should always exist
            None => None
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Node<T> {
    Branch(u8),
    Leaf(T)
}



#[test]
fn set_and_get() {
    use sparse_octree::ChildId;

    let mut octree = SparseOctree::<u64>::new();
    let location1 = NodeLocation::new_root() //0b1_000_000
        .child(ChildId::BLF).unwrap()
        .child(ChildId::BLF).unwrap();
    let location2 = NodeLocation::new_root() //0b1_000_001_111
        .child(ChildId::BLF).unwrap()
        .child(ChildId::BRF).unwrap()
        .child(ChildId::TRB).unwrap(); 

    octree.set_node(location1, 1).unwrap();
    octree.set_node(location2, 2).unwrap();
    
    // Succeeds
    assert!(octree.get_node(location1) == Some(&1));
    assert!(octree.get_node(location2) == Some(&2));

    // Fails because the parent is a leaf
    let child_location = location1.child(ChildId::BLB).unwrap();
    assert!(octree.set_node(child_location, 0) == Err(()));

    // Make sure their common branch has set its child flags correctly
    let child_id1 = location1.child_id();
    let child_id2 = location2.parent().unwrap().child_id();
    let common_parent_location = location1.parent().unwrap();
    assert!(octree.get(common_parent_location) == Some(&Node::Branch(child_id1.flag() | child_id2.flag())));
}

#[test]
pub fn sort() {
    use sparse_octree::ChildId;

    let mut octree = SparseOctree::<u64>::new();
    let parent_location = NodeLocation::new_root();

    let location1 = parent_location.child(ChildId::BLF).unwrap();
    let location2 = parent_location.child(ChildId::BLB).unwrap();
    let location3 = parent_location.child(ChildId::TLF).unwrap();
    let location4 = parent_location.child(ChildId::TRB).unwrap();
    
    octree.set_node(location2, 2).unwrap();
    octree.set_node(location4, 4).unwrap();
    octree.set_node(location1, 1).unwrap();
    octree.set_node(location3, 3).unwrap();

    println!("storage: {:?}, map: {:?}", octree.storage, octree.map);
    octree.sort();
    println!("storage: {:?}, map: {:?}", octree.storage, octree.map);

    // Make sure querying still works
    assert_eq!(octree.get_node(location1).unwrap(), &1u64);
    assert_eq!(octree.get_node(location2).unwrap(), &2u64);
    assert_eq!(octree.get_node(location3).unwrap(), &3u64);
    assert_eq!(octree.get_node(location4).unwrap(), &4u64);
}

#[test]
pub fn count1() {
    let mut octree = SparseOctree::<&str>::new();
    let root = NodeLocation::new_root();

    octree.set_node(root
                        .child(0b000.into()).unwrap(), "1 000").unwrap();

    octree.set_node(root
                        .child(0b001.into()).unwrap()
                            .child(0b000.into()).unwrap(), "1 001 000").unwrap();
    octree.set_node(root
                        .child(0b001.into()).unwrap()
                            .child(0b001.into()).unwrap(), "1 001 001").unwrap();

    octree.set_node(root
                        .child(0b010.into()).unwrap(), "1 010").unwrap();
    octree.set_node(root
                        .child(0b011.into()).unwrap(), "1 011").unwrap();

    octree.set_node(root
                        .child(0b111.into()).unwrap()
                            .child(0b000.into()).unwrap(), "1 111 000").unwrap();

    octree.set_node(root
                        .child(0b100.into()).unwrap()
                            .child(0b001.into()).unwrap()
                                .child(0b001.into()).unwrap(), "1 100 001 001").unwrap();

    octree.sort();

    let search_node = root.child(111.into()).unwrap();
    assert_eq!(octree.get_nodes(root).unwrap().len(), octree.storage.len());
    assert_eq!(octree.get_nodes(search_node).unwrap(), &[Node::Branch(1), Node::Leaf("1 111 000")]);
}
