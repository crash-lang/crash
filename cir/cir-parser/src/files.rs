use std::fs::File;
use std::io::Read;

/// Compare files and check if they're equal
pub fn diff_files(file_1: &mut File, file_2: &mut File) -> bool {
    let buff1: &mut [u8] = &mut [0; 1024];
    let buff2: &mut [u8] = &mut [0; 1024];

    loop {
        match file_1.read(buff1) {
            Err(_) => return false,
            Ok(f1_read_len) => match file_2.read(buff2) {
                Err(_) => return false,
                Ok(f2_read_len) => {
                    if f1_read_len == 0 {
                        return true;
                    }
                    if &buff1[0..f1_read_len] != &buff2[0..f2_read_len] || f1_read_len != f2_read_len {
                        return false;
                    }
                }
            }
        }
    }
}