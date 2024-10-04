use anyhomw::Result;
use std::ffi::OsStr;
use std::io;
use std::iter;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use winapi::ctypes::c_void;
use winapi::um::winuser::SystemParametersInfoW;
use winapi::um::winuser::SPIF_SENDCHANGE;
use winapi::um::winuser::SPIF_UPDATEINIFILE;
use winapi::um::winuser::SPI_GETDESKWALLPAPER;
use winapi::um::winuser::SPI_SETDESKWALLPAPER;
use winreg::enums::*;
use winreg::RegKey;

pub fn set_wallpaper(filepath: &str) -> Result<()> {
    unsafe {
        let path = OsStr::new(path)
            .encode_wide()
            // append null byte
            .chain(iter::once(0))
            .collect::<Vec<u16>>();
        let successful = SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            path.as_ptr() as *mut c_void,
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        ) == 1;

        if successful {
            Ok(())
        } else {
            println!("{}", filepath);
        }
    }
}
