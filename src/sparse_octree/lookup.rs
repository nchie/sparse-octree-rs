use super::{ NodeLocation, Node };
use std::collections::HashMap;

fn recurse_nodes<T, F>(index: usize, location: NodeLocation, nodes: &[Node<T>], func: &mut F)
        where F: FnMut(NodeLocation, usize) {
    match nodes[0] {
        Node::Branch(f) => {
            func(location, index);
            let mut count = 0;
            for i in 0..8 {
                if 1<<i & f > 0 {
                    count += 1;
                    if nodes.len() > 1 { 
                        lookup_recurse(index+count, location.child(i.into()).unwrap(), &nodes[1..], func);
                    }
                }
            }
        },
        Node::Leaf(_) => {
            func(location, index);
        }
    }
}

pub fn for_each_continuous_node<T, F>(nodes: &[Node<T>], func: &mut F) where F: FnMut(NodeLocation, usize) {
    recurse_nodes(0, NodeLocation::new_root(), nodes, func);
}



fn lookup_recurse<T, F>(index: usize, location: NodeLocation, nodes: &[Node<T>], func: &mut F)
        where F: FnMut(NodeLocation, usize) {
    match nodes[index] {
        Node::Branch(f) => {
            func(location, index);
            let mut count = 0;
            for i in 0..8 {
                if 1<<i & f > 0 {
                    count += 1;
                    lookup_recurse(index+count, location.child(i.into()).unwrap(), nodes, func);
                }
            }
        },
        Node::Leaf(_) => {
            func(location, index);
        }
    }
}

pub fn gen_lookup<T>(location: NodeLocation, nodes: &[Node<T>], lookup: &mut HashMap<NodeLocation, usize>) {
    lookup_recurse(0, location, nodes, &mut |location, index|{ lookup.insert(location, index); });
}

pub fn remove_lookup<T>(location: NodeLocation, nodes: &[Node<T>], lookup: &mut HashMap<NodeLocation, usize>) {
    lookup_recurse(0, location, nodes, &mut |location, _|{ lookup.remove(&location); });
}




fn gen_lookup_recurse<T>(index: usize, location: NodeLocation, nodes: &[Node<T>], lookup: &mut HashMap<NodeLocation, usize>) {
    match nodes[index] {
        Node::Branch(f) => {
            lookup.insert(location, index);
            let mut count = 0;
            for i in 0..8 {
                if 1<<i & f > 0 {
                    count += 1;
                    gen_lookup_recurse(index+count, location.child(i.into()).unwrap(), nodes, lookup);
                }
            }
        },
        Node::Leaf(_) => {
            lookup.insert(location, index);
        }
    }
}