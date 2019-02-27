use super::MMIO_BASE;

// use core::ops;
use register::{mmio::ReadWrite, register_bitfields};

register_bitfields! {
    u32,

    /// function Select 2
    GPFSEL2 [
    // based on 6.2 Alternative Function Assignments in BCM2835 ARM Peripherals

        FSEL27 OFFSET(21) NUMBITS(3)[ // GPIO27
            Input = 0b000,
            Output = 0b001,
            Alt0 = 0b100,
            Alt1 = 0b101,
            Alt2 = 0b110,
            Alt3 = 0b111,
            Alt4 = 0b011, // JTAG ARM debug: ARM_TMS
            Alt5 = 0b010
        ],

        FSEL26 OFFSET(18) NUMBITS(3)[ // GPIO26
            Input = 0b000,
            Output = 0b001,
            Alt0 = 0b100,
            Alt1 = 0b101,
            Alt2 = 0b110,
            Alt3 = 0b111,
            Alt4 = 0b011, // JTAG ARM debug: ARM_TDI
            Alt5 = 0b010
        ],

        FSEL25 OFFSET(15) NUMBITS(3)[ // GPIO25
            Input = 0b000,
            Output = 0b001,
            Alt0 = 0b100,
            Alt1 = 0b101,
            Alt2 = 0b110,
            Alt3 = 0b111,
            Alt4 = 0b011, // JTAG ARM debug: ARM_TCK
            Alt5 = 0b010
        ],

        FSEL24 OFFSET(12) NUMBITS(3)[ // GPIO24
            Input = 0b000,
            Output = 0b001,
            Alt0 = 0b100,
            Alt1 = 0b101,
            Alt2 = 0b110,
            Alt3 = 0b111,
            Alt4 = 0b011, // JTAG ARM debug: ARM_TDO
            Alt5 = 0b010
        ],

        FSEL23 OFFSET(9) NUMBITS(3)[ // GPIO23
            Input = 0b000,
            Output = 0b001,
            Alt0 = 0b100,
            Alt1 = 0b101,
            Alt2 = 0b110,
            Alt3 = 0b111,
            Alt4 = 0b011, // JTAG ARM debug: ARM_RTCK
            Alt5 = 0b010
        ],

        FSEL22 OFFSET(6) NUMBITS(3)[ // GPIO22
            Input = 0b000,
            Output = 0b001,
            Alt0 = 0b100,
            Alt1 = 0b101,
            Alt2 = 0b110,
            Alt3 = 0b111,
            Alt4 = 0b011, // JTAG ARM debug: ARM_TRST
            Alt5 = 0b010
        ]

    ]
}

const GPFSEL2: *const ReadWrite<u32, GPFSEL2::Register> =
    (MMIO_BASE + 0x0020_0008) as *const ReadWrite<u32, GPFSEL2::Register>;

pub fn setup_debug() {
    unsafe {
        (*GPFSEL2).modify(
            GPFSEL2::FSEL27::Alt4
                + GPFSEL2::FSEL26::Alt4
                + GPFSEL2::FSEL25::Alt4
                + GPFSEL2::FSEL24::Alt4
                + GPFSEL2::FSEL23::Alt4
                + GPFSEL2::FSEL22::Alt4,
        );
    }
}