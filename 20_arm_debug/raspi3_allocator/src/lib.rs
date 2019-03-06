// #![deny(missing_docs)]
// #![deny(warnings)]
#![no_std]
#![feature(alloc_error_handler)]

use core::alloc::GlobalAlloc;
use core::alloc::Layout;
use core::ptr;

struct BumpPointerAlloc {
    head: usize,
    end: usize,
}

unsafe impl Sync for BumpPointerAlloc {}

unsafe impl GlobalAlloc for BumpPointerAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut head = self.head;

        let align = layout.align();
        let res = head % align;
        let start = if res == 0 { head } else { head + align - res };
        if start + align > self.end {
            // a null pointer signal an Out Of Memory condition
            ptr::null_mut()
        } else {
            head = start + align;
            start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        // this allocator never deallocates memory
    }
}

#[global_allocator]
static HEAP: BumpPointerAlloc = BumpPointerAlloc {
    head: 0x0100_0000,
    end: 0x0200_0000,
};

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    loop {}
}
