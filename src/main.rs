#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(philopp_ostest::test_runner)]
#![reexport_test_harness_main = "test_main"]

use philopp_ostest::println;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    
    philopp_ostest::init();
    // x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();
   
    println!("no crashes here!");
    philopp_ostest::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    philopp_ostest::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    philopp_ostest::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}






