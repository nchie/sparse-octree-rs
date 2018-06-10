use std::collections::HashMap;
use std::collections::HashSet;

use sparse_octree::{ MAX_DEPTH, NodeLocation };
use sparse_octree::lookup::{ gen_lookup, remove_lookup, for_each_continuous_node };

pub struct SparseOctree<T: Clone> {
    storage: Vec<Node<T>>,
    map: HashMap<NodeLocation, usize>,
    depth: u32,
    unsorted: HashSet<NodeLocation>,
    unused: Vec<usize>
}

// Public
impl<T: Clone> SparseOctree<T> {
    pub fn new() -> SparseOctree<T> {
        SparseOctree::<T>{
            storage: Vec::new(),
            map: HashMap::new(),
            depth: 0,
            unsorted: HashSet::new(),
            unused: Vec::new()
        }
    }

    pub fn get_single(&self, location: NodeLocation) -> Option<&T> {
        if let Some(&Node::Leaf(ref node)) = self.get_node(location) {
            Some( node )
        } else {
            None
        }
    }

    pub fn clone_subtree(&self, location: NodeLocation) -> Option<SparseOctree<T>> {
        // If location is marked as unsorted
        if self.unsorted.contains(&location) { return None };
        // TODO: Check if location has an ancestor leaf before returning empty?
        if !self.map.contains_key(&location) { return None } // TODO: Return empty octant?  

        let index = self.map[&location];
        let (count, depth) = self.count_from_index(index);

        let storage = self.storage[index..index+count].to_vec();
        let mut lookup = HashMap::new();
        gen_lookup(NodeLocation::new_root(), &storage, &mut lookup);

        Some(SparseOctree {
            storage: storage,
            map: lookup,
            depth: depth,
            unsorted: HashSet::new(), // Subtree has to be sorted already
            unused: Vec::new()        // If the subtree is sorted, there are no unused nodes
        })
    }

    pub fn get_slice(&self, location: NodeLocation) -> Option<&[Node<T>]> {
        // If location is marked as unsorted
        if self.unsorted.contains(&location) { return None };
        // TODO: Check if location has an ancestor leaf before returning empty?
        if !self.map.contains_key(&location) { return Some(&[]) }  

        let index = self.map[&location];
        let (count, _depth) = self.count_from_index(index);
        println!("depth: {}", _depth); // TODO: Make sure depth calculation is correct
        Some(&self.storage[index..index+count])
    }

    pub fn insert_subtree(&mut self, subtree: SparseOctree<T>, location: NodeLocation) {
        if location.depth() + subtree.depth > MAX_DEPTH {
            // Error, octree would end up too deep!
        }

        // Check if location has leaf ancestor
        //      If has, mark old ancestor as unused and create new from there?

        // Check whether octant exists at location
        if let Some(&index) = self.map.get(&location) {
            // If it exists, mark as unused and clear lookup entries!
            let (length, _depth) = self.count_from_index(index);

            // Remove old lookups
            remove_lookup(location, &self.storage[index..index+length], &mut self.map);

            if length == subtree.storage.len() {
                // Overwrite in storage and generate new lookups

                // Generate new lookups
                gen_lookup(location, &self.storage[index..index+length], &mut self.map);
            } else {
                // Insert into back of array

                // Mark old nodes as unused

                // Generate new lookups
                gen_lookup(location, &self.storage[index..index+length], &mut self.map);
            }
            
        } 
        
        // insert new octant into back of storage
        // ...
        // recurse through new structure, adding lookup entries
        // ...
        // mark ancestors as unsorted
        self.set_ancestors_unsorted(location);
    }

    fn clear_location(&mut self, location: NodeLocation) {
        // TODO: Make nodes as unused
        self.clear_lookup(location);
    }

    fn clear_lookup(&mut self, location: NodeLocation) {
        // TODO: This is O(n), but it should be possible to make it more efficient by iterating
        // through the old removed indices and removing their lookup entries!
        self.map.retain(|&iter_location, _|{
            if iter_location >= location || iter_location < location.next() { 
                false
            } else {
                true
            }
        });
    }

    pub fn set(&mut self, location: NodeLocation, t: T) -> Result<(), ()>
    {
        self.set_node(location, Node::Leaf(t))
    }

    pub fn sort(&mut self) {
        use std;
        // TODO: Improve performance without storing location inside every node
        let mut kv_vec = Vec::with_capacity(self.storage.len());

        // Swap out the old storage with an empty but allocated new one
        let mut old_storage = Vec::with_capacity(self.storage.len());

        // TODO: Don't insert unused nodes and clear self.storage afterwards
        std::mem::swap(&mut self.storage, &mut old_storage);

        // Clear the old map into a vec
        for (k, v) in self.map.drain() {
            kv_vec.push((k, v));
        }

        // Sort the vec
        kv_vec.sort_unstable_by_key(|kv| { kv.0 });

        // Insert the nodes from old_storage into the new, but in correct order
        for (_, v) in kv_vec.iter() {
            self.storage.push(old_storage[*v].clone()); // TODO: Change to unchecked? Get rid of clone?
        }
        
        // Insert new mappings
        for (i, (k, _)) in kv_vec.iter().enumerate() {
            self.map.insert(*k, i);
        }

        // Everything's back to sorted!
        self.unsorted.clear();
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

}

// Private
impl<T: Clone> SparseOctree<T> {
    fn detach(&mut self, location: NodeLocation) {
        if !self.unsorted.contains(&location) {
            let &index = self.map.get(&location).unwrap();
            // Iterate continuous nodes and remove all lookups
            for_each_continuous_node(&self.storage[index..], &mut |location, index| {
                
            })
        } else {
            // Iterate through lookups to remove all lookup entries
        }
    }


    fn ancestor(&self, location: NodeLocation) -> NodeLocation {
        // TODO: Find nearest existing ancestor node
        NodeLocation::new_root()
    }

    fn count_from_index(&self, index: usize) -> (usize, u32) {

        match &self.storage[index] {
            Node::Branch(ref f) => {
                let mut count = 1;

                // Count children and loop through them recursively, incrementing the index by the amount of nodes read
                let bit_count = f.count_ones() as usize;
                let mut highest_depth = 0;
                for _ in 0..bit_count {
                    let (inner_count, depth) = self.count_from_index(index+count);
                    count+=inner_count;
                    if depth > highest_depth { highest_depth = depth }
                }
                (count, highest_depth+1)
            },
            Node::Leaf(_) => (1, 0)
        }
    }

    fn set_node(&mut self, location: NodeLocation, node: Node<T>) -> Result<(), ()> {
        // Make sure ancestors are pointing towards this (fails if an ancestor is a leaf)
        self.update_ancestors(location)?;

        if location.depth() > self.depth { 
            self.depth = location.depth() 
        };

        if let Some(index) = self.map.insert(location, self.storage.len()) {
            // If location already had a node, replace it
            self.storage[index] = node;
        } else {
            // Else create room for a new node
            self.storage.push(node);

            // Assume that nodes added this way will make their ancestors unsorted
            self.set_ancestors_unsorted(location);
        }
        Ok(())
    }

    fn update_ancestors(&mut self, location: NodeLocation) -> Result<(), ()> {
        if let (Some(parent_location), child_id) = location.disown() {
            // If it has a parent
            if let Some(node) = self.get_node_mut(parent_location) {
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
                self.set_node(parent_location, Node::Branch(child_id.flag())) // TODO: Change to actual flag from ChildId
            }
        } else {
            // If no parent, stop recursing
            return Ok(()) 
        }
    }

    fn set_ancestors_unsorted(&mut self, mut location: NodeLocation) {
        // Recursively update parents to be unsorted
        while let Some(parent_location) = location.parent() {
            if self.unsorted.insert(parent_location) {
                // If parent did not exist in unsorted, keep recursing
                location = parent_location;
            } else {
                // We ran into an unsorted ancestor so we can stop recursing
                return;
            }
        }
    }

    pub fn get_node(&self, location: NodeLocation) -> Option<&Node<T>> {
        let index = self.map.get(&location);
        match index {
            // If map has an index for the code, a node exists
            Some(&x) => Some( self.storage.get(x).unwrap() ),  // Unwrap should be safe here as it should always exist
            None => None
        }
    }

    fn get_node_mut(&mut self, location: NodeLocation) -> Option<&mut Node<T>> {
        let index = self.map.get(&location);
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
pub fn lookup_removal_insertion() {
    use sparse_octree::ChildId;

    let mut octree = SparseOctree::<u64>::new();

    let root = NodeLocation::new_root().child(ChildId::BLF).unwrap();

    let location1 = root
        .child(ChildId::BLF).unwrap()
        .child(ChildId::BLF).unwrap();
    let location2 = root
        .child(ChildId::BLF).unwrap()
        .child(ChildId::BRF).unwrap()
        .child(ChildId::TRB).unwrap(); 

    octree.set(location1, 1).unwrap();
    octree.set(location2, 2).unwrap();

    let mut lookup = HashMap::new();

    octree.sort();

    // TODO: This only works if relevant slice is passed, not whole storage?
    gen_lookup(NodeLocation::new_root().child(ChildId::BLF).unwrap(), &octree.storage, &mut lookup);
    assert_eq!(lookup, octree.map);

    remove_lookup(NodeLocation::new_root(), &octree.storage, &mut lookup);
    assert_eq!(lookup, HashMap::new());
}