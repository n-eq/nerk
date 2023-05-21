use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        super::handlers::define_handlers(&mut idt);
        idt
    };
}

pub(crate) fn init_idt() {
    IDT.load();
}

#[allow(dead_code)]
#[inline]
pub fn int3() {
    unsafe {
        core::arch::asm!("int3");
    }
}
