#![recursion_limit = "2048"]
mod dbg;
mod hook;

use std::os::raw::c_void;

use windows::Win32::Foundation::*;

use dbg::debug_writeln;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(module: HMODULE, call_reason: u32, _reserved: *mut c_void) -> BOOL {
    match call_reason {
        windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH => {
            unsafe {
                windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls(module).unwrap();
                // windows::Win32::System::Console::AllocConsole();
            };
            // std::thread::sleep(std::time::Duration::from_millis(30000));
            std::thread::spawn(|| unsafe {
                let hooked = hook::hook();

                match hooked {
                    Ok(()) => 0u32,
                    Err(e) => {
                        debug_writeln(&format!("Error occurred when injecting: {}", e));
                        1
                    }
                }
            });
        }
        _ => (),
    }

    true.into()
}
