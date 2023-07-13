# HeapMaster

HeapMaster is a dynamic memory allocation library written in Rust. It provides functionality for managing memory blocks and implements the buddy system algorithm for efficient memory allocation.

## Table of Contents
- [Introduction](#introduction)
- [Usage](#usage)
- [Functionality](#functionality)
- [Future Goals](#future-goals)
- [Contributing](#contributing)
- [License](#license)

## Introduction

HeapMaster is designed to efficiently allocate and manage dynamic memory using the buddy system algorithm. It allows users to allocate and release memory blocks of various sizes, optimizing memory utilization and minimizing fragmentation.

This repository contains the Rust implementation of HeapMaster, along with tests and examples to demonstrate its usage and functionality.



## Usage
    use mempool::MemoryPool;
    
    fn main() {

    // Create a new memory pool
    
    let mut memory_pool = MemoryPool::new();

    // Add a heap to the memory pool
    
    let heap_size = 4096; // Size of the heap in bytes
    let heap = vec![0u8; heap_size as usize]; // Allocate the heap memory
    memory_pool.add_heap(heap.as_ptr() as *mut u8, heap_size);

    // Allocate memory from the memory pool
    
    let block_size = 256; // Size of the block to allocate in bytes
    let block = memory_pool.hl_alloc(block_size);

    if !block.is_null() {
        // Memory allocation successful, use the block
        // ...

        // Release the allocated memory
        memory_pool.hl_release(block);
    }
}

## Functionality
The current version of HeapMaster provides the following functionality:

- Initialize a memory pool with one or more heaps.
- Allocate memory blocks of variable sizes.
- Release allocated memory blocks.
## Future Goals
The future goals for HeapMaster include:

- Thread safety: Implement thread-safe operations to ensure concurrent access to the memory pool.
- Fragmentation management: Improve fragmentation handling to minimize memory fragmentation.
- Error handling: Implement proper error handling and error reporting mechanisms.
Contributions are welcome to help achieve these goals.

## Contributing
Contributions to HeapMaster are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request. Make sure to follow the project's code of conduct.

## License
HeapMaster is licensed under the MIT License. See the LICENSE file for more details.



