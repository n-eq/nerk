pub(crate) mod gdt;
mod handlers;
pub(crate) mod idt;
pub(crate) mod pic8259;

pub fn init() {
    gdt::init();
    idt::init_idt();

    unsafe {
        pic8259::PICS.lock().initialize();
    }
}
