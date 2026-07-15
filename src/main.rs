// MAIN RUNTIME
use std::{
    env, 
    ffi::c_void
};
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{CloseHandle, GENERIC_READ, GENERIC_WRITE, HANDLE},
        Storage::FileSystem::{CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING},
        System::IO::DeviceIoControl,
    },
};
//DATA
const INPUT_OUTPUT_CONTROL: u32 = 0x0022240C;
const DEVICE_SET: u32 = 0x53564544;
const MODE: u32 = 0x00120075;
//EXECUTION
fn convert(text: &str) -> Vec<u16> {
    text.encode_utf16().chain([0]).collect()
}
fn access() -> Result<HANDLE, windows::core::Error> {
    unsafe {
        CreateFileW(
            PCWSTR(convert(r"\\.\ATKACPI").as_ptr()),
            (GENERIC_READ | GENERIC_WRITE).0,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            None,
        )
    }
}
fn call(acpi: HANDLE, id: u32, data: &[u8]) -> Result<[u8;16], Box<dyn std::error::Error>> {
    let mut inbuf = Vec::with_capacity(8 + data.len());
    inbuf.extend(id.to_le_bytes());
    inbuf.extend((data.len() as u32).to_le_bytes());
    inbuf.extend(data);
    let mut response = [0;16];
    let mut bytes_returned = 0;
    unsafe {
        DeviceIoControl(acpi,
                        INPUT_OUTPUT_CONTROL,
                        Some(inbuf.as_ptr() as *const c_void),
                        inbuf.len() as u32,
                        Some(response.as_mut_ptr() as *mut c_void),
                        16,
                        Some(&mut bytes_returned),
                        None,
                    )?;
    }
    if bytes_returned < 4 {
        return Err(format!("Unexpected response size: {bytes_returned}").into());
    }
    else {
        Ok(response)
    }
}
fn main() -> Result<(), windows::core::Error> {
    let (mode, name) = match env::args().nth(1).as_deref() {
        Some("/balanced") => (0, "Balanced"),
        Some("/turbo") => (1, "Turbo"),
        Some("/silent") => (2, "Silent"),
        Some("/help") => {
            println!("Usage:");
            println!("/balanced  - Set Balanced mode");
            println!("/turbo     - Set Turbo mode");
            println!("/silent    - Set Silent mode");
            println!("/help      - List available commands that you can use");
            return Ok(());
        },
        None => {
            println!("You didn't provide any arguments, please type /help to know how to use it properly.");
            return Ok(());
        },
        _ => {
            println!("Unknown command. Check your spelling or type /help for correction.");
            return Ok(());
        },
    };
    let open = access()?;
    let mut args = [0;8];
    args[..4].copy_from_slice(&MODE.to_le_bytes());
    args[4..].copy_from_slice(&(mode as u32).to_le_bytes());
    match call(open, DEVICE_SET, &args) {
        Ok(result) => {
            let success = i32::from_le_bytes(result[..4].try_into().unwrap()) == 1;
            if success {
                println!("Successfully set {name} mode\nAPPROVED");
            }
            else {
                eprintln!("Failed to set {name} mode\nREJECTED");
            }
        }
        Err(message) => {
            eprintln!("DeviceIoControl FAILURE: {message}");
        }
    }
    unsafe {
        CloseHandle(open)?;
    }
    Ok(())
}