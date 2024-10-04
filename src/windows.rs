use anyhow::Result;
use std::ffi::OsStr;
use std::iter;
use std::os::windows::ffi::OsStrExt;
use winapi::ctypes::c_void;
use winapi::um::winuser::SystemParametersInfoW;
use winapi::um::winuser::SPIF_SENDCHANGE;
use winapi::um::winuser::SPIF_UPDATEINIFILE;
use winapi::um::winuser::SPI_SETDESKWALLPAPER;
use winreg::enums::*;
use winreg::RegKey;

// completely stolen from https://github.com/reujab/wallpaper.rs/blob/master/src/windows.rs
pub fn set_wallpaper(filepath: &str) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (desktop, _) = hkcu.create_subkey(r"Control Panel\Desktop")?;

    desktop.set_value("WallpaperStyle", &"10".to_string())?;

    unsafe {
        let path = OsStr::new(filepath)
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
            Ok(())
        }
    }
}
