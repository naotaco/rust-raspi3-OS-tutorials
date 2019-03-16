// #![deny(missing_docs)]
// #![deny(warnings)]
#![no_std]
#![feature(alloc_error_handler)]

use core::alloc::GlobalAlloc;
use core::alloc::Layout;
use core::ptr;

struct Raspi3Alloc {
    head: u32,
    end: u32,
}

unsafe impl Sync for Raspi3Alloc {}

unsafe impl GlobalAlloc for Raspi3Alloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let allocated_size = core::ptr::read_volatile(self.head as *mut u32);
        let offset = 0x100; // for header
        let align = layout.align() as u32;
        let align_offset = match allocated_size % align {
            0 => 0,
            (m) => align - m,
        };
        let pointer = self.head + allocated_size + offset + align_offset; // to return

        // ensure to keep requested size.
        if pointer + layout.size() as u32 > self.end {
            return ptr::null_mut();
        }

        //        let align = layout.align() as u32;
        //        let res = self.head % align;
        // let start = match res {
        //     0 => self.head + *allocated_size,
        //     _ => self.head + align - res + *allocated_size,
        // };

        // store next value
        core::ptr::write_volatile(
            self.head as *mut u32,
            allocated_size + (layout.size() as u32) + align_offset,
        );

        pointer as *mut u8

        // if start + align > self.end {
        //     // a null pointer signal an Out Of Memory condition
        //     ptr::null_mut()
        // } else {
        //     //            head = start + align;
        //     ptr::write_bytes(start as *mut u8, 0, 4);
        //     start as *mut u8
        // }
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        // this allocator never deallocates memory
    }
}

#[global_allocator]
static HEAP: Raspi3Alloc = Raspi3Alloc {
    head: 0x0100_0000,
    end: 0x0200_0000,
};

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    loop {}
}
