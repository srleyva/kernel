#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;

// This function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::printfatal!("{}", info);
    blog_os::hlt_loop();
}

// Function called on test
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info);
    blog_os::hlt_loop();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    blog_os::init();

    use x86_64::registers::control::Cr3;

    // let (level_4_page_table, _) = Cr3::read();
    // println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    println!("Hello World{}", "!");

    // Trigger a panic
    //panic!("This failed");

    // Trigger Breakpoint
    // x86_64::instructions::interrupts::int3();

    //trigger a page fault
    // let ptr = 0x2031cc as *mut u32;
    // unsafe { let x = *ptr; }
    // unsafe { *ptr = 42; }


    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

    /* Trigger StackOverflow

    This result in a tripple fault 
    as the guard page (an unmapped page) is 
    set at the bottom of the stack to make catching
    over overflow possible 
    (as writing to this cause a page fault becuase its unmapped)
    the double fault handler is pushed onto the call stack
    however the memory is still unmapped so this results in another
    page fault resulting in a tripple fault. 
    See GDT module for the solution to this issue
    */
    // fn stack_overflow() {
    //     stack_overflow();
    // }

    //stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    blog_os::hlt_loop();
}