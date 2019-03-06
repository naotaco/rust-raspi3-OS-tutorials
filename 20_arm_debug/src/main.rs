/*
 * MIT License
 *
 * Copyright (c) 2018 Andre Richter <andre.o.richter@gmail.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#![no_std]
#![no_main]
#![feature(asm)]

const MMIO_BASE: u32 = 0x3F00_0000;

mod arm_debug;
mod gpio;
mod mbox;
mod uart;

use core::sync::atomic::{compiler_fence, Ordering};

fn kernel_entry() -> ! {
    let mut mbox = mbox::Mbox::new();
    let uart = uart::Uart::new();

    // set up serial console
    match uart.init(&mut mbox) {
        Ok(_) => uart.puts("\n[0] UART is live!\n"),
        Err(_) => loop {
            unsafe { asm!("wfe" :::: "volatile") }; // If UART fails, abort early
        },
    }

    uart.puts("[1] Press a key to continue booting... ");
    uart.getc();
    uart.puts("Greetings fellow Rustacean!\n");

    // get the board's unique serial number with a mailbox call
    mbox.buffer[0] = 8 * 4; // length of the message
    mbox.buffer[1] = mbox::REQUEST; // this is a request message
    mbox.buffer[2] = mbox::tag::GETSERIAL; // get serial number command
    mbox.buffer[3] = 8; // buffer size
    mbox.buffer[4] = 8;
    mbox.buffer[5] = 0; // clear output buffer
    mbox.buffer[6] = 0;
    mbox.buffer[7] = mbox::tag::LAST;

    // Insert a compiler fence that ensures that all stores to the
    // mbox buffer are finished before the GPU is signaled (which is
    // done by a store operation as well).
    compiler_fence(Ordering::Release);

    // send the message to the GPU and receive answer
    let serial_avail = match mbox.call(mbox::channel::PROP) {
        Err(_) => false,
        Ok(()) => true,
    };

    let main_id = get_part_id();

    if serial_avail {
        uart.puts("[i] My serial number is: 0x");
        uart.hex(mbox.buffer[6]);
        uart.hex(mbox.buffer[5]);
        uart.puts("\n");
        uart.puts("My part id is: ");
        uart.hex(main_id);
        uart.puts("\n");
    } else {
        uart.puts("[i] Unable to query serial!\n");
    }

    arm_debug::setup_debug();

    let mut counter = 0;
    // echo everything back
    loop {
        counter += 1;
        uart.hex(counter);
    }
}

fn get_part_id() -> u32 {
    // MRC p15, 0, <Rt>, c0, c0, 0 ; Read MIDR into Rt <- aarch32
    // MRS <Xt>, MIDR_EL1 // => 410FD034
    let mut v: u32;
    unsafe {
        asm!("MRS x5, MIDR_EL1"
        :"={x5}"(v)
        );
    }
    v
}

raspi3_boot::entry!(kernel_entry);
