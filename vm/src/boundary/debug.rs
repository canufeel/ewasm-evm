#[no_mangle]
extern "C" {
    fn logDebug(pc: i32);
}

pub fn log_debug_local(pc: i32) {
    unsafe { logDebug(pc); }
}
