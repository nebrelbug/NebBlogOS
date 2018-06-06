#![feature(lang_items)]
#![no_std]
#![feature(unique)]
#![feature(const_fn)]
#![feature(ptr_internals)]
//All external dependencies
extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate volatile;
use vga_buffer::*;
#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    vga_buffer::clear_screen();
    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!(
            "    start: 0x{:x}, length: 0x{:x}",
            area.base_addr, area.length
        );
    }
    let elf_sections_tag = boot_info
        .elf_sections_tag()
        .expect("Elf-sections tag required");

    println!("kernel sections:");
    for section in elf_sections_tag.sections() {
        println!(
            "    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
            section.addr, section.size, section.flags
        );
    }
    let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();
    let kernel_end = elf_sections_tag
        .sections()
        .map(|s| s.addr + s.size)
        .max()
        .unwrap();

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("Kernel start: {}, Kernel end: {}", kernel_start, kernel_end);
    println!(
        "Multiboot start: {}, Multiboot end: {}",
        multiboot_start, multiboot_end
    );
    setcolor!(Color::White, Color::Black);
    println!("Hello, {}", "it's me");
    setcolor!(Color::Green, Color::White);
    println!("Hello, {}", "it's me");
    setposition!(0, 0, ' ');

    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nNeblogOS is panicking in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop {}
}
