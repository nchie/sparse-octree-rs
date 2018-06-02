use std::collections::HashMap;

use sparse_octree::NodeLocation;

pub struct SparseOctree<T> {
    storage: Vec<Node<T>>,
    map: HashMap<NodeLocation, usize>
}

// Public
impl<T> SparseOctree<T> {
    pub fn new() -> SparseOctree<T> {
        SparseOctree::<T>{
            storage: Vec::new(),
            map: HashMap::<NodeLocation, usize>::new()
        }
    }

    pub fn get_node(&self, location: NodeLocation) -> Option<(NodeLocation, &T)> {
        if let Some((location, &Node::Leaf(ref node))) = self.get(location) {
            Some((location, node))
        } else {
            None
        }
    }

    pub fn set_node(&mut self, location: NodeLocation, t: T) -> Result<(), ()>
    {
        self.set(location, Node::Leaf(t))
    }
}

// Private
impl<T> SparseOctree<T> {
    fn set(&mut self, location: NodeLocation, node: Node<T>) -> Result<(), ()> {
        // Make sure ancestors are pointing towards this (fails if an ancestor is a leaf)
        self.update_ancestors(location)?;

        if let Some(index) = self.map.insert(location, self.storage.len()) {
            // If location already had a node, replace it
            self.storage[index] = node;
        } else {
            // Else create room for a new node
            self.storage.push(node);
        }
        Ok(())
    }

    fn update_ancestors(&mut self, location: NodeLocation) -> Result<(), ()> {
        if let (Some(parent_location), child_id) = location.disown() {
            // If it has a parent
            if let Some((_, node)) = self.get_mut(parent_location) {
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

    fn get(&self, location: NodeLocation) -> Option<(NodeLocation, &Node<T>)> {
        let index = self.map.get(&location);
        match index {
            // If map has an index for the code, a node exists
            Some(&x) => Some( (location, self.storage.get(x).unwrap()) ),  // Unwrap should be safe here as it should always exist
            None => None
        }
    }

    fn get_mut(&mut self, location: NodeLocation) -> Option<(NodeLocation, &mut Node<T>)> {
        let index = self.map.get(&location);
        //println!("get location: {:?}", location);
        match index {
            // If map has an index for the location, a node exists
            Some(&x) => Some( (location, self.storage.get_mut(x).unwrap()) ),  // Unwrap should be safe here as it should always exist
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
    assert!(octree.get_node(location1) == Some((location1, &1)));
    assert!(octree.get_node(location2) == Some((location2, &2)));

    // Fails because the parent is a leaf
    let child_location = location1.child(ChildId::BLB).unwrap();
    assert!(octree.set_node(child_location, 0) == Err(()));

    // Make sure their common branch has set its child flags correctly
    let child_id1 = location1.child_id();
    let child_id2 = location2.parent().unwrap().child_id();
    let common_parent_location = location1.parent().unwrap();
    assert!(octree.get(common_parent_location) == Some((common_parent_location, &Node::Branch(child_id1.flag() | child_id2.flag()))));
}
