#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &[u8] = b"Hello Casey!! Hi Maple and Mango :D";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hi Maple and Mango, again!").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers include: {} and {}", 1, 2.71828).unwrap();

    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

