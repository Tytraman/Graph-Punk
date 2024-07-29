use std::ffi::CString;

pub struct PunkString;

impl PunkString {
    pub fn i8_array_to_string(arr: &[i8]) -> String {
        // Créer un CString nécessite un tableau qui n'a pas d'octet valant 0,
        // il faut donc filtrer la collection.
        let message: Vec<u8> = arr
            .iter()
            .take_while(|&c| *c != 0)
            .map(|&c| c as u8)
            .collect();

        let message = CString::new(message).unwrap();

        message.into_string().unwrap()
    }
}

#[macro_export]
macro_rules! punk_info {

    ($($args:tt)*) => {{
        println!("[Graph-Punk] [INFO] {}", format!($($args)*));
    }};
}

#[macro_export]
macro_rules! punk_error {
    ($($args:tt)*) => {
        {
            println!("[Graph-Punk] [ERROR] {}", format!($($args)*));
        }
    };
}

#[macro_export]
macro_rules! punk_warning {
    ($($args:tt)*) => {
        {
        println!("[Graph-Punk] [WARNING] {}", format!($($args)*));
        }
    };
}
