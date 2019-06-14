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
    loop {}
}

// Function called on test
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    blog_os::init();
    println!("Hello World{}", "!");

    // Trigger a panic
    //panic!("This failed");

    // Trigger Breakpoint
    // x86_64::instructions::interrupts::int3();

    //trigger a page fault
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
    */
    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}