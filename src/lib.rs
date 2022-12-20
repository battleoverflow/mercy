/*
    Owner: Catherine Framework (https://github.com/CatherineFramework)
    Project: Mercy
    License: BSD 2-Clause
*/

use std::{
    path::Path,
    fs::{self, File},
    io::Read,
    net::UdpSocket
};

use base64;
use md5;
use sha2::{Sha256, Digest};
use hexdump::hexdump;

use sys_info::{
    hostname,
    cpu_num,
    cpu_speed,
    os_release,
    proc_total
};

pub fn mercy_source() -> String {
    const VERSION: &str = "1.1.13";
    const AUTHOR: &str = "Catherine Framework (https://github.com/CatherineFramework)";
    return format!("Author: {}\nVersion: {}\nDocumentation: https://docs.rs/crate/mercy/latest", AUTHOR, VERSION);
}

/*
    Public decoding methods provided by Mercy
*/
pub fn mercy_decode(mercy_call: &str, mercy_string: &str) -> String {
    match mercy_call {
        "base64" => base64_decode(mercy_string.to_string()),
        "rot13" => rot13_decode(mercy_string.to_string()),
         _ => unknown_msg("Unable to decode message")
    }
}

/*
    Public encoding methods provided by Mercy
*/
pub fn mercy_encode(mercy_call: &str, mercy_string: &str) -> String {
    match mercy_call {
        "base64" => base64_encode(mercy_string.to_string()),
         _ => unknown_msg("Unable to encode message")
    }
}

/*
    Public hash methods provided by Mercy
*/
pub fn mercy_hash(mercy_call: &str, mercy_string: &str) -> String {
    match mercy_call {
        "sha2_256" => sha2_256_hash(mercy_string.to_string()),
        "md5" => md5_hash(mercy_string.to_string()),
        _ => unknown_msg("Unable to hash message")
    }
}

/*
    Public hexadecimal methods provided by Mercy
*/
pub fn mercy_hex(mercy_call: &str, mercy_file: &str) -> String {
    match mercy_call {
        "hex_dump" => collect_file_hex(mercy_file),
        _ => unknown_msg("Unable to provide hexadecimal dump for file specified")
    }
}

/*
    Public extra methods provided by Mercy
*/
pub fn mercy_extra(mercy_call: &str, mercy_choose: &str) -> String {
    match mercy_call {
        "internal_ip" => internal_ip(),
        "system_info" => system_info(mercy_choose),
        _ => unknown_msg("Unable to provide the information you requested")
    }
}

/*
    Decoding methods
*/

// Base64 decode
fn base64_decode(encoded_msg: String) -> String {
    // Converts into bytes
    let bytes = base64::decode(encoded_msg.to_string()).expect("Unable to decode provided string");
    
    // Converts into a more readable format
    let final_out = String::from_utf8_lossy(&bytes);

    return final_out.to_string();
}

// rot13 decode
fn rot13_decode(encoded_msg: String) -> String {
    let mut result_str = String::from("");
    
    // Iterates over encoded_msg
    for x in encoded_msg.chars() {
        let charcode = x as u32;
        
        if x.is_lowercase() {
            // Checks if character in string is lowercase
            let check_text = 'a' as u32;
            let rot_final = ((charcode - check_text + 13) % 26) + check_text;
            result_str.push(char::from_u32(rot_final).unwrap());
        } else if x.is_uppercase() {
            // Checks if character in string is uppercse
            let check_text = 'A' as u32;
            let rot_final = ((charcode - check_text + 13) % 26) + check_text;
            result_str.push(char::from_u32(rot_final).unwrap());
        } else {
            result_str.push(x);
        }
    }
    
    return result_str.to_string();
}

/*
    Encoding methods
*/

// Base64 encode
fn base64_encode(plaintext_msg: String) -> String {
    // Converts into bytes
    let encoded_msg = base64::encode(plaintext_msg.as_bytes());
    return encoded_msg.to_string();
}

/*
    Hashing methods
*/

// SHA256 hash
fn sha2_256_hash(plaintext_msg: String) -> String {
    let mut run_hash = Sha256::new();
    run_hash.update(plaintext_msg.as_bytes());

    let hash = run_hash.finalize();
    return format!("{:x}", hash);
}

// MD5 hash
fn md5_hash(plaintext_msg: String) -> String {
    let hash = md5::compute(plaintext_msg.as_bytes());
    return format!("{:x}", hash);
}

/*
    Hexadecimal manipulation
*/

// Converts file/bytes to a readable vector
fn byte_to_vec(filename: &str) -> Vec<u8> {
    let mut file = File::open(&filename).expect("Unable to locate file");
    let file_metadata = fs::metadata(&filename).expect("Unable to read file metadata");
    let mut buffer = vec![0; file_metadata.len() as usize];

    // Writes buffer data to the hex file 
    for i in 0..buffer.len() {
        file.read(&mut buffer).expect("Buffer overflow detected. Stopping operation...");

        if i == buffer.len() {
            println!("Buffer limit exceeded");
            break;
        }
    }

    buffer
}

fn collect_file_hex(convert_file: &str) -> String {
    // convert_file requires an absolute path to work 100% of the time
    if Path::new(convert_file).exists() {
        // Dumps hex data to stdout
        return format!("{:#?}", hexdump(&byte_to_vec(convert_file)));
    } else {
        return format!("Unable to locate the file specified");
    }
}

/*
    Miscellaneous
*/

// Quick method for collecting the internal ip address of the local system
fn internal_ip() -> String {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Unable to bind UDP socket");
    socket.connect("8.8.8.8:80").expect("Unable to connect to address");
    let addr = socket.local_addr().expect("Unable to return the socket address");
    return addr.ip().to_string();
}

// System information based on matching parameter
fn system_info(data: &str) -> String {

    let all_system_info = format!("\nHostname: {}\nNumber of CPU cores: {}\nCPU Fan Speed: {} MHz\nOperating System Release Version: {}\nNumber of Processes: {}\n", hostname().unwrap(), cpu_num().unwrap(), cpu_speed().unwrap(), os_release().unwrap(), proc_total().unwrap());

    match data {
        "hostname" => return format!("Hostname: {}", hostname().unwrap()),
        "cpu_cores" => return format!("Number of CPU cores: {}", cpu_num().unwrap()),
        "cpu_speed" => return format!("CPU Fan Speed: {} MHz", cpu_speed().unwrap()),
        "os_release" => return format!("Operating System Release Version: {}", os_release().unwrap()),
        "proc" => return format!("Number of Processes: {}", proc_total().unwrap()),
        "all" => return format!("{}", all_system_info),
        _ => return format!("Unable to gather system information")
    }
}

fn unknown_msg(custom_msg: &str) -> String {
    return format!("{}", custom_msg);
}