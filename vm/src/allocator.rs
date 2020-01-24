use wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &::core::panic::PanicInfo) -> ! {
    unsafe {
        ::core::intrinsics::abort();
    }
}

// Need to provide an allocation error handler which just aborts
// the execution with trap.
#[cfg(not(test))]
#[alloc_error_handler]
#[no_mangle]
pub extern "C" fn oom(_: ::core::alloc::Layout) -> ! {
    unsafe {
        ::core::intrinsics::abort();
    }
}

// Needed for non-wasm targets.
#[cfg(not(test))]
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}
