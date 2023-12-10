use std::mem::transmute;
use retour::static_detour;
use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows::Win32::UI::WindowsAndMessaging::{SM_REMOTESESSION, SYSTEM_METRICS_INDEX};

use crate::dbg::debug_writeln;

static_detour! {
    static GetSystemMetricsHook: extern "system" fn(SYSTEM_METRICS_INDEX) -> i32;
}

fn new_get_system_metrics(n_index: SYSTEM_METRICS_INDEX) -> i32 {
    // is remote desktop? no ;)
    let result = if n_index == SM_REMOTESESSION {
        0
    } else {
        GetSystemMetricsHook.call(n_index)
    };

    debug_writeln(&format!("GetSystemMetrics({:#x}) -> {}\n", n_index.0, result));
    result
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe fn hook() -> Result<(), retour::Error> {
    let user32 = GetModuleHandleA(PCSTR("User32.dll\0".as_ptr())).unwrap();
    let original = GetProcAddress(user32, PCSTR("GetSystemMetrics\0".as_ptr()));
    GetSystemMetricsHook.initialize(
        transmute(original),
        new_get_system_metrics,
    )?;
    GetSystemMetricsHook.enable()?;

    Ok(())
}
