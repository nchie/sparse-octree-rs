// Check whether octant exists at location
if let Some(&index) = self.map.get(&location) {
    // If exists, check if new size is different
    let (current_len, _depth) = self.count_from_index(index);
    if current_len > octant.data.len() { 
        // If bigger...
        // insert into back of storage
        // ...
        // mark old nodes as unused
        //..
        // remove old lookup entries
        self.clear_lookup(location);

        // ...
        // recurse through new structure, adding lookup entries 
        // ...
        // mark ancestors as unsorted
        self.set_ancestors_unsorted(location);

    }
    else if current_len < octant.data.len() {
        // If smaller...
        // overwrite old location
        // ...
        // mark unused nodes, remove old lookup entries
        // ...
        // mark ancestors as unsorted
        self.set_ancestors_unsorted(location);

    }
    else { 
        // If same size...
        // overwrite old location,
        // remove old lookup entries
        // recurse through new structure, adding lookup entries 
    }

} else {
    // If didn't exist...
    // insert into back of storage
    // ...
    // recurse through new structure, adding lookup entries
    // ...
    // mark ancestors as unsorted
    self.set_ancestors_unsorted(location);
}


// Check whether octant exists at location
//      If exists, check if new size is different
//          If bigger, insert into new location, mark old nodes as unused and mark ancestors as unsorted
//          If smaller, insert into old location, mark unused nodes and mark ancestors as unsorted 
//          If unchanged, insert into old location
//      If not exists, insert into back of storage and mark ancestors as unsorted