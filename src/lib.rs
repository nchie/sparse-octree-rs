#![feature(nll)]
#![feature(inclusive_range_syntax)]

mod sparse_octree;
mod tests;

pub use sparse_octree::SparseOctree;
pub use sparse_octree::NodeLocation;