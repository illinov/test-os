#[used] // Decir al compilador que esta función es usada, aunque no se llame desde Rust
#[unsafe(link_section = ".multiboot_header")] // Crear la seccion .multiboot_header
static MULTIBOOT_HEADER: [u32; 6] = {
    let magic: u32 = 0xe85250d6;
    let arch: u32 = 0;
    let length: u32 = 24;
    let checksum: u32 = (0x100000000u64 - (magic + arch + length) as u64) as u32;
    [magic, arch, length, checksum, 0, 8]
};