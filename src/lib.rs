#![feature(unique)]
#![feature(lang_items)]
#![no_std]
#![feature(const_fn)]
#![feature(const_unique_new)]

extern crate volatile;
extern crate rlibc;
extern crate spin;
extern crate multiboot2;

#[macro_use]
mod vga_buffer;
mod memory;

#[macro_use]
extern crate bitflags;
extern crate x86_64;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize)
{

    use memory::FrameAllocator;
 
    vga_buffer::clear_screen();
    println!("Hello World{}", "!");
    println!("{}", { println!("inner"); "outer" });

    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag()
       .expect("Memory map tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas()
    {
       println!("    start: 0x{:x}, length: 0x{:x}",
       area.base_addr, area.length);
    }

   let elf_sections_tag = boot_info.elf_sections_tag()
       .expect("Elf-sections tag required");

   println!("kernel sections:");
   for section in elf_sections_tag.sections()
   {
    println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
        section.addr, section.size, section.flags);
   } 
  
   let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
    .min().unwrap();
   let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
    .max().unwrap();

    println!("kernel start: 0x{}", kernel_start);
    println!("kernel end: 0x{}", kernel_end);

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("multiboot start: 0x{}", multiboot_start);
    println!("multiboot end: 0x{}", multiboot_end); 

    let mut frame_allocator = memory::AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize, multiboot_start,
        multiboot_end, memory_map_tag.memory_areas());
	memory::test_paging(&mut frame_allocator);

    for i in 0..
    {
        if let None = frame_allocator.allocate_frame()
	{
            println!("allocated {} frames", i);
            break;
        }
   }

   loop{}
   
    /*
    let y = 4;
    let x = fix(y);
    println!("Fibboniacci of {} is {}", y, x);
    */
    
}

pub fn fix(n:u32) -> u32
{
	if n == 0
	{
		0
	}
	else
	{
	  let(res, _) = fib(n);
	  res
	}
}

pub fn fib(n:u32) -> (u32, u32)
{
	if n == 1
	{
		(1,0)
	}
	else
	{
		println!("fib({})", n);
		let (f1, f2) = fib(n-1);
		(f1 + f2, f1)
	}
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str,
    line: u32) -> !
{
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop{}
}

