use std::env;
use std::fs::File;
use std::io::Read;

const GLOBAL_BUFFER_LENGTH: usize = 16;

fn get_file(path_to_file: String) -> File {
    match File::open(path_to_file) {
        Ok(f) => File::from(f),
        Err(e) => {
            panic!("{}", e);
        }
    }
}

pub fn get_hex_rep(byte_array: &mut [u8]) -> String {
    let build_str_vec: Vec<String> = byte_array.chunks(2)
        .map(|c| {
            if c.len() == 2 {
                format!("{:02x}{:02x}", c[0], c[1])
            } else {
                format!("{:02x}", c[0])
            }
        }).collect();

        build_str_vec.join(" ")
}

pub fn get_ascii_rep(byte_array: &mut [u8]) -> String {
    let build_str_vec: Vec<String> = byte_array.iter().map(|num| {
        if *num >= 32 && *num <= 126 {
            (*num as char).to_string()
        } else {
            '.'.to_string()
        }
    }).collect();

    build_str_vec.join("")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments");
    }

    let mut file_to_read = get_file(String::from(&args[1]));

    let mut buffer = [0; GLOBAL_BUFFER_LENGTH];
    let mut offset: usize = 0;

    loop {
        let bytes_read = file_to_read.read(&mut buffer);
        match bytes_read {
            Ok(number) => {
                if number == 0 {
                    break;
                } else {
                    println!("{:08x}: {:40} {:10}",
                        offset,
                        get_hex_rep(&mut buffer[0..number]),
                        get_ascii_rep(&mut buffer[0..number])
                    );
                    offset += GLOBAL_BUFFER_LENGTH;
                }
            },
            Err(why) => {
                eprintln!("rusthexdump: {}", why);
                break;
            }
        }
    }
}
