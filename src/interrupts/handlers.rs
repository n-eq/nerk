use crate::println;

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: x86_64::structures::idt::InterruptStackFrame,
) {
    println!("EXCEPTION: BREAKPOINT\n{stack_frame:#?}");
}

extern "x86-interrupt" fn divide_by_zero_handler(
    stack_frame: x86_64::structures::idt::InterruptStackFrame,
) {
    println!("EXCEPTION: DIVIDE BY ZERO\n{stack_frame:#?}");
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: x86_64::structures::idt::InterruptStackFrame,
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
}
