
_This project is still in a very, very unfinished stage, use it on your own risk!_

# sparse-octree-rs
A generic sparse hashed octree data structure written in Rust.

## Sparse Hashed Linear Octree
A sparse hashed octree is a linear octree which uses a hashmap as a lookup table reach nodes in O(1), 
while still benefitting from the compact storage of a linear octree. Inspired by [this](https://geidav.wordpress.com/2014/08/18/advanced-octrees-2-node-representations/) blog post. 
