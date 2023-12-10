use windows::core::{HSTRING, PCWSTR};
use windows::Win32::System::Diagnostics::Debug::OutputDebugStringW;

pub fn debug_writeln(message: &str) {
    unsafe {
        OutputDebugStringW(PCWSTR(HSTRING::from(message.to_owned() + "\n").as_ptr()));
    }
}