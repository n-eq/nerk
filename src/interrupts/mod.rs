pub(crate) mod gdt;
mod handlers;
pub(crate) mod idt;

pub fn init() {
    gdt::init();
    idt::init_idt();
}
