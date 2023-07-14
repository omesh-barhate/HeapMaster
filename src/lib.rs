use std::mem;

const MIN_HEAP_SIZE: u32 = 1024;

#[derive(Debug)]
struct Block {
    size: u32,
    used: bool,
    next: Option<*mut Block>,
}

pub struct Heap {
    start: *mut u8,
    end: *mut u8,
    first_block: Option<*mut Block>,
}

pub struct MemoryPool {
    heaps: Vec<Heap>,
}

impl MemoryPool {
    pub fn new() -> Self {
        MemoryPool { heaps: Vec::new() }
    }

    pub fn add_heap(&mut self, heap: *mut u8, heap_size: u32) {
        let new_heap = Heap {
            start: heap,
            end: unsafe { heap.add(heap_size as usize) },
            first_block: Some(heap as *mut Block),
        };
        self.heaps.push(new_heap);
    }

    pub fn hl_alloc(&mut self, block_size: u32) -> *mut u8 {
        let required_block_size = calculate_block_size(block_size);
        for heap in &mut self.heaps {
            let mut curr_block = heap.first_block;

            while let Some(block) = curr_block {
                let block_ref = unsafe { &mut *block };

                if !block_ref.used && block_ref.size >= required_block_size {
                    if block_ref.size > required_block_size {
                        unsafe {
                            split_block(heap, block, required_block_size);
                        }
                    }
                    block_ref.used = true;
                    return unsafe { (block as *mut u8).add(mem::size_of::<Block>()) };
                }
                curr_block = block_ref.next;
            }
        }

        std::ptr::null_mut()
    }

    pub fn hl_release(&mut self, block: *mut u8) {
        for heap in &mut self.heaps {
            let released_block = unsafe { (block as *mut Block).offset(-1) };
            let heap_start = heap.start as *mut Block;
            let heap_end = heap.end as *mut Block;

            if (heap_start <= released_block) && (released_block < heap_end) {
                let block_ref = unsafe { &mut *released_block };
                assert!(
                    block_ref.used,
                    "Attempting to release an unused block"
                );
                block_ref.used = false;
                unsafe {
                    coalesce_blocks(heap);
                }
                return;
            }
        }
    }

    pub fn hl_resize(&mut self, block: *mut u8, new_size: u32) -> *mut u8 {
        for heap in &mut self.heaps {
            let resize_block = unsafe { (block as *mut Block).offset(-1) };
            let heap_start = heap.start as *mut Block;
            let heap_end = heap.end as *mut Block;

            if (heap_start <= resize_block) && (resize_block < heap_end) {
                let block_ref = unsafe { &mut *resize_block };
                assert!(
                    block_ref.used,
                    "Attempting to resize an unused block"
                );

                let current_block_size = block_ref.size;
                let new_block_size = calculate_block_size(new_size);

                if new_block_size == current_block_size {
                    return block;
                } else if new_block_size < current_block_size {
                    unsafe {
                        shrink_block(heap, resize_block, new_block_size);
                    }
                    return block;
                } else {
                    let next_block = block_ref.next;
                    if let Some(next) = next_block {
                        let next_block_ref = unsafe { &mut *next };
                        if !next_block_ref.used && (current_block_size + next_block_ref.size) >= new_block_size {
                            if (current_block_size + next_block_ref.size) > new_block_size {
                                unsafe {
                                    split_block(heap, resize_block, new_block_size);
                                }
                            }
                            unsafe {
                                coalesce_blocks(heap);
                            }
                            return block;
                        }
                    }
                }

                let new_block = self.hl_alloc(new_size);
                if !new_block.is_null() {
                    let current_block_size = calculate_block_size(block_ref.size);
                    let copy_size = if new_size < current_block_size {
                        new_size
                    } else {
                        current_block_size
                    };
                    unsafe {
                        std::ptr::copy_nonoverlapping(block, new_block, copy_size as usize);
                    }
                    self.hl_release(block);
                    return new_block;
                }

                return std::ptr::null_mut();
            }
        }

        std::ptr::null_mut()
    }
}

fn calculate_block_size(requested_size: u32) -> u32 {
    let mut size = MIN_HEAP_SIZE;
    while size < requested_size {
        size *= 2;
    }
    size
}

unsafe fn split_block(heap: &mut Heap, block: *mut Block, required_size: u32) {
    let block_ref = &mut *block;
    let block_size = block_ref.size;
    let remaining_size = block_size - required_size;

    if remaining_size >= mem::size_of::<Block>() as u32 {
        let new_block = (block as *mut u8).add(required_size as usize) as *mut Block;
        let new_block_ref = &mut *new_block;
        new_block_ref.size = remaining_size;
        new_block_ref.used = false;
        new_block_ref.next = block_ref.next;
        block_ref.next = Some(new_block);
    }

    block_ref.size = required_size;
}

unsafe fn coalesce_blocks(heap: &mut Heap) {
    let mut curr_block = heap.first_block;

    while let Some(block) = curr_block {
        let block_ref = &mut *block;
        if let Some(next_block) = block_ref.next {
            let next_block_ref = &mut *next_block;
            if !block_ref.used && !next_block_ref.used {
                block_ref.size += next_block_ref.size;
                block_ref.next = next_block_ref.next;
            }
        }
        curr_block = block_ref.next;
    }
}

unsafe fn shrink_block(heap: &mut Heap, block: *mut Block, new_size: u32) {
    let block_ref = &mut *block;
    let block_size = block_ref.size;
    let remaining_size = block_size - new_size;

    if remaining_size >= mem::size_of::<Block>() as u32 {
        let new_block = (block as *mut u8).add(new_size as usize) as *mut Block;
        let new_block_ref = &mut *new_block;
        new_block_ref.size = remaining_size;
        new_block_ref.used = false;
        new_block_ref.next = block_ref.next;
        block_ref.size = new_size;
        block_ref.next = Some(new_block);
    }
}

