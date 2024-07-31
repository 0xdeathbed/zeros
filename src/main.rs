#![no_std] // Disable std linking
#![no_main] // To disable normal entry point i.e `main`

use core::panic::PanicInfo;

#[no_mangle] // Disable name mangling to ensure `_start` remains function name
             // `_start` used because it's default entry point name for most systems
pub extern "C" fn _start() -> ! {
    // As this function is invoked by os or bootloader it should invoke exit syscall or shutdown machine instead of returning so marked as diverging by never type

    loop {}
}

#[panic_handler] // Defining function to invoke on panic which std automatically provides
fn panic(_info: &PanicInfo) -> ! {
    // As panic handler function should never return it is marked as diverging function by
    // returning never type `!`
    loop {}
}
