use std::env;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use winapi::ctypes::c_void;
use winapi::um::winuser::SystemParametersInfoW;
use winapi::um::winuser::SPIF_UPDATEINIFILE;
use winapi::um::winuser::SPI_SETDESKWALLPAPER;
use winapi::um::winuser::SPIF_SENDCHANGE;

use live_wallpapers::*;

fn main() {
    let class_name = wide_null("Window Class Name");
    let window_name = wide_null("Window Name");
    let (window_class, h_instance) = WNDCLASSW::create(&class_name);
    let window_handle = create_window_handle(&window_class, &class_name, &window_name, h_instance);
    let window = create_window(window_handle);


    loop {

    }

    let path = get_folder_path();
    let delay = get_delay();
    let path_str = path.to_str().unwrap().trim();

    loop {
        for i in 1..14 {
            let img_path = format!("{}\\{}.jpeg", path_str, i);
            set_wallpaper_img(img_path.as_str());
            thread::sleep(Duration::from_millis(delay));
        }
    }
}

fn get_folder_path() -> PathBuf {
    println!("Input images folder: ");
    let input = std::io::stdin();
    let mut images_folder = String::new();
    input.read_line(&mut images_folder);
    let dir = env::current_dir();
    if dir.is_err() {
        todo!();
    }

    let mut binding = dir.unwrap();
    binding.push(images_folder);

    println!("{}", binding.to_str().unwrap());
    binding.clone()
}

fn get_delay() -> u64 {
    println!("Input delay: ");
    let input = std::io::stdin();
    let mut delay_input = String::new();
    input.read_line(&mut delay_input);
    match delay_input.as_str().trim().parse() {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Can not parse delay string: {}\n({})", delay_input, e);
            350
        },
    }
}

fn set_wallpaper_img(path: &str) {
    let path = OsStr::new(path)
            .encode_wide()
            .chain(iter::once(0))
            .collect::<Vec<u16>>();
    unsafe {
        let successful = SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            path.as_ptr() as *mut c_void,
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        ) == 1;

        if !successful {
            println!("{}", std::io::Error::last_os_error().to_string());
        }
    }
}