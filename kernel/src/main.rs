#![no_std] // no enlazar con la biblioteca estandar de Rust
#![no_main] // deshabilitar todos los entry points estandar
use core::panic::PanicInfo;
extern crate bootloader;

static HELLO: &[u8] = b"Hola papum";

#[unsafe(no_mangle)] // no modificar el nombre de esta funcion
pub extern "C" fn _start() -> ! {
    // esta función es el punto de entrada, ya que el linker busca una función
    // llamada `_start` por defecto

    let vga_buffer: *mut u8 = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

/// Esta función se llama cuando ocurre un panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
