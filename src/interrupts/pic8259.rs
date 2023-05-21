use pic8259::ChainedPics;
use spin;

// CPU exceptions are mapped to 0..31, we chose an immediately available range starting from 32.
const PIC_1_OFFSET: u8 = 32;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
#[allow(unused)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard, /* PIC_1_OFFSET + 1 */
    Unused1,
    Unused2,
    Unused3,
    FloppyDisk,
    Unused4,
}

pub(crate) static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });
