#![no_std]
#![no_main]
#![feature(panic_info_message)] // at the top of the file

use core::panic::PanicInfo;
use blog_os::{serial_println,serial_print, QemuExitCode, exit_qemu};

const MESSAGE: &str = "Example panic message from panic_handler test";
const PANIC_LINE: u32 = 14; // adjust this when moving the `panic!` call

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("panic_handler... ");
    panic!("{}", MESSAGE);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    check_message(info);
    check_location(info);


    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

use core::fmt::Write;

fn check_message(info: &PanicInfo) {
    let message = info.message().unwrap_or_else(|| fail("no message"));
    let mut compare_message = CompareMessage { expected: MESSAGE };
    write!(&mut compare_message, "{}", message)
        .unwrap_or_else(|_| fail("write failed"));
    if !compare_message.expected.is_empty() {
        fail("message shorter than expected");
    }
}

fn check_location(info: &PanicInfo) {
    let location = info.location().unwrap_or_else(|| fail("no location"));
    if location.file() != file!() {
        fail("filename is wrong");
    }

    if location.line() != PANIC_LINE {
        fail("panic line is wrong");
    }
}

use core::fmt;

/// Compares a `fmt::Arguments` instance with the `MESSAGE` string
///
/// To use this type, write the `fmt::Arguments` instance to it using the
/// `write` macro. If the message component matches `MESSAGE`, the `expected`
/// field is the empty string.
struct CompareMessage {
    expected: &'static str,
}

impl fmt::Write for CompareMessage {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.expected.starts_with(s) {
            self.expected = &self.expected[s.len()..];        
        } else {
            fail("message not equal to expected message");
        }
        Ok(())
    }
}

fn fail(error: &str) -> ! {
    serial_println!("[failed]");
    serial_println!("error {}", error);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}