use std::collections::HashMap;

use sparse_octree::{LocationalCode, ChildCode};

pub struct SparseOctree<T> {
    storage: Vec<Node<T>>,
    map: HashMap<LocationalCode, usize>
}

impl<T> SparseOctree<T> {
    pub fn new() -> SparseOctree<T> {
        SparseOctree::<T>{
            storage: Vec::new(),
            map: HashMap::<LocationalCode, usize>::new()
        }
    }

    pub fn get_node(&self, code: LocationalCode) -> Option<(LocationalCode, &T)> {
        if let Some((code, &Node::Leaf(ref node))) = self.get(code) {
            Some((code, node))
        } else {
            None
        }
    }

    pub fn get(&self, code: LocationalCode) -> Option<(LocationalCode, &Node<T>)> {
        let index = self.map.get(&code);
        match index {
            // If map has an index for the code, a node exists
            Some(&x) => Some( (code, self.storage.get(x).unwrap()) ),  // Unwrap should be safe here as it should always exist
            None => None
        }
    }

    pub fn get_mut(&mut self, code: LocationalCode) -> Option<(LocationalCode, &mut Node<T>)> {
        let index = self.map.get(&code);
        println!("get code: {:?}", code);
        match index {
            // If map has an index for the code, a node exists
            Some(&x) => Some( (code, self.storage.get_mut(x).unwrap()) ),  // Unwrap should be safe here as it should always exist
            None => None
        }
    }

    pub fn set_node(&mut self, code: LocationalCode, t: T) -> Result<(), ()>
    {
        self.set(code, Node::Leaf(t))
    }

    fn set(&mut self, code: LocationalCode, node: Node<T>) -> Result<(), ()> {
        // Make sure ancestors are pointing towards this (fails if an ancestor is a leaf)
        self.update_ancestors(code)?;

        if let Some(index) = self.map.insert(code, self.storage.len()) {
            self.storage[index] = node;
        } else {
            self.storage.push(node);
        }
        Ok(())
    }

    fn update_ancestors(&mut self, code: LocationalCode) -> Result<(), ()> {
        if let (Some(parent_code), child_code) = code.disown() {
            // If it has a parent
            if let Some((_, node)) = self.get_mut(parent_code) {
                match *node {
                    Node::Branch(mut f) => {
                        f |= child_code.flag(); // TODO: Change to actual flag from ChildCode
                        Ok(()) // Stop recursing

                    },
                    Node::Leaf(_) => {
                        // Parent was a leaf, return Err since it's not valid for a leaf to have another leaf as ancestor
                        Err(())
                    }
                }
            } else {
                // If parent didn't exist, set it
                self.set(parent_code, Node::Branch(child_code.flag())) // TODO: Change to actual flag from ChildCode
            }


        } else {
            // If no parent, stop recursing
            return Ok(()) 
        }


    }
}


#[derive(Debug, Copy, Clone)]
pub enum Node<T> {
    Branch(u8),
    Leaf(T)
}



#[test]
fn set_and_get() {
    let mut octree = SparseOctree::<u64>::new();
    let code1 = LocationalCode::new_debug(0b1000111000111000);
    let code2 = LocationalCode::new_debug(0b1000111000111001111);
    octree.set_node(code1, 1).unwrap();
    octree.set_node(code2, 2).unwrap();

    if let Some((_, &node)) = octree.get_node(code1) {
        assert_eq!(node, 1);
    } else {
        assert!(false);
    }
    
}
