use crate::println;
use x86_64::structures::idt::InterruptStackFrame;

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        super::pic8259::PICS
            .lock()
            .notify_end_of_interrupt(super::pic8259::InterruptIndex::Timer as u8);
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{stack_frame:#?}");
}

extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: DIVIDE BY ZERO\n{stack_frame:#?}");
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    println!("DOUBLE FAULT {error_code:?}\n{stack_frame:#?}");
    loop {}
}

pub(crate) fn define_handlers(idt: &mut x86_64::structures::idt::InterruptDescriptorTable) {
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.divide_error.set_handler_fn(divide_by_zero_handler);
    unsafe {
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(super::gdt::DOUBLE_FAULT_IST_INDEX);
    }
    idt[super::pic8259::InterruptIndex::Timer as usize].set_handler_fn(timer_interrupt_handler);
    idt[super::pic8259::InterruptIndex::Keyboard as usize]
        .set_handler_fn(crate::keyboard::keyboard_interrupt_handler);
}
