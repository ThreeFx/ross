#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::{asm,global_asm};

#[macro_use]
mod uart;

global_asm!(include_str!("boot.s"));
global_asm!(include_str!("interrupt.s"));

#[export_name = "start_kernel"]
pub fn start_kernel() -> ! {
    println!("Starting kernel...");
    let hartid = riscv::register::mhartid::read();
    println!("Got hartid: hartid={}", hartid);
    let mtime_ptr = 0x200_BFF8 as *const u64;
    let mtimecmp_ptr = 0x200_4000 as *mut u64;
    unsafe { riscv::interrupt::enable() }
    unsafe { riscv::register::mie::set_mtimer() }
    println!("enabled interrupts");
    let clint_ptr = 0x200_0000 as *mut u32;
    unsafe { clint_ptr.write_volatile(1); }
    println!("enabled clint");
    let mtime = unsafe { mtime_ptr.read_volatile() };
    unsafe { mtimecmp_ptr.write_volatile(mtime + 2_000_000) };
    println!("read time={}", mtime);
    println!("set timer to={}", mtime + 2_000_000);
    loop {
        unsafe { asm!("wfi") }
    }
}

#[repr(C)]
pub struct InterruptContext {
    pub regs: [u64; 32], // 32 * 8 = 2^8 = 256
    pub mcause: u64, // 8
    pub hartid: u64, // 8
}
// 272

#[panic_handler]
fn panic(p: &PanicInfo) -> ! {
    if let Some(location) = p.location() {
        println!(
            "panic occurred in file '{}' at line {}",
            location.file(),
            location.line(),
        );
    } else {
        println!("panic occurred but can't get location information...");
    }
    loop {
        unsafe { riscv::asm::wfi() }
    }
}

#[export_name = "handle_interrupt"]
pub extern "C" fn handle_interrupt(ic: &InterruptContext) {
    let synchronicity = match ic.mcause >> 63 {
        0 => "synchronous",
        1 => "asynchronous",
        _ => panic!("impossible"),
    };
    println!("handle_interrupt hartid={} mcause={} synchronicity={}", ic.hartid, ic.mcause, synchronicity);

    match ic.mcause & (1 << 63) - 1 {
        7 => handle_timer_interrupt(),
        _ => (),
    };
}

/// Handles a timer interrupt.
#[inline]
fn handle_timer_interrupt() {
    println!("handling timer interrupt");
    let mtime_ptr = 0x200_BFF8 as *const u64;
    let mtimecmp_ptr = 0x200_4000 as *mut u64;
    let mtime = unsafe { mtime_ptr.read_volatile() };
    unsafe { mtimecmp_ptr.write_volatile(mtime + 2_000_000) };
    println!("read time={}", mtime);
    println!("set new timer");
}
