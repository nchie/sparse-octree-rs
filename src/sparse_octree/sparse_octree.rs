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
        match index {
            // If map has an index for the code, a node exists
            Some(&x) => Some( (code, self.storage.get_mut(x).unwrap()) ),  // Unwrap should be safe here as it should always exist
            None => None
        }
    }

    pub fn set_node(&mut self, code: LocationalCode, t: T) 
    {
        self.set(code, Node::Leaf(t));
    }

    fn set(&mut self, code: LocationalCode, node: Node<T>) -> Result<(), ()> {
        if let Some(parent_code) = code.parent() {
            // If it has a parent, 'activate' it
            self.activate(parent_code, code.into())?;
        };

        

        Ok(())
    }

    fn activate(&mut self, code: LocationalCode, child_code: ChildCode) -> Result<(), ()> {
        if let Some((_, node)) = self.get_mut(code) {
            match node {
                Node::Branch(f) => {
                    *f |= 1u8; // TODO: Change to actual flag from ChildCode
                    Ok(()) // Stop recursing

                },
                Node::Leaf(_) => {
                    // Parent was a leaf, return 
                    Err(())
                }
            }
        } else {
            self.set(code, Node::Branch(0u8)) // TODO: Change to actual flag from ChildCode
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub enum Node<T> {
    Branch(u8),
    Leaf(T)
}
