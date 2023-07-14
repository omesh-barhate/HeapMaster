mod lib;

fn main() {
    // Create a new memory pool
    let mut pool = lib::MemoryPool::new();

    // Add a heap to the memory pool
    let heap_size = 4096;
    let heap: Vec<u8> = Vec::with_capacity(heap_size as usize);
    let heap_ptr = heap.as_ptr() as *mut u8;
    pool.add_heap(heap_ptr, heap_size);

    // Allocate a block
    let block_size = 128;
    let block = pool.hl_alloc(block_size);
    if block.is_null() {
        panic!("Failed to allocate block");
    }

    // Release the block
    pool.hl_release(block);

    // Resize the block
    let new_size = 64;
    let resized_block = pool.hl_resize(block, new_size);
    if resized_block.is_null() {
        panic!("Failed to resize block");
    }

    // Print the resized block address
    println!("Resized block address: {:?}", resized_block);
}

