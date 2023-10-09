//! # Mercy
//!
//! Mercy is an open source Rust crate and CLI designed for building cybersecurity tools, assessment projects, and testing.
//!
//! | Function                | More Info                               |
//! | ----------------------- | --------------------------------------- |
//! | `source`          | Learn more about the crate              |
//! | `decode`          | Supports: base64, rot13                 |
//! | `encode`          | Supports: base64                        |
//! | `hash`            | Supports: sha256, md5                   |
//! | `hex`             | Dump hexadecimal values of a file       |
//! | `malicious`       | Malware detection or malicious intent   |
//! | `extra`           | Information about various data points   |
//! | `experimental`    | Experimental functions for data control |
//! 

/*
    Project: Mercy (https://github.com/azazelm3dj3d/mercy)
    Author(s): azazelm3dj3d (https://github.com/azazelm3dj3d)
    License: BSD 2-Clause
*/

use std::{
    path::Path,
    str::from_utf8,
    fs::{
        self,
        File
    },
    io::{
        self,
        Read,
        Write
    },
    net::{
        UdpSocket,
        TcpStream
    }
};

use serde_json::Value;

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

use lemmeknow::Identifier;

use ares::{
    perform_cracking,
    config::Config
};

use whatlang;
use mailparse::*;

/// Learn more about the crate
pub fn source() -> String {
    const VERSION: &str = "2.0.1";
    const AUTHOR: &str = "azazelm3dj3d (https://github.com/azazelm3dj3d)";
    return format!("Author: {}\nVersion: {}\nDocumentation: https://docs.rs/crate/mercy/latest", AUTHOR, VERSION);
}

/* Public decoding methods provided by Mercy */

/// Supports: base64, rot13
pub fn decode(mercy_call: &str, mercy_string: &str) -> String {
    match mercy_call {
        "base64" => base64_decode(mercy_string.to_string()),
        "rot13" => rot13_decode(mercy_string.to_string()),
         _ => unknown_msg("Unable to decode message")
    }
}

/* Public encoding methods provided by Mercy */

/// Supports: base64
pub fn encode(mercy_call: &str, mercy_string: &str) -> String {
    match mercy_call {
        "base64" => base64_encode(mercy_string.to_string()),
         _ => unknown_msg("Unable to encode message")
    }
}

/* Public hashing methods provided by Mercy */

/// Supports: sha256, md5
pub fn hash(mercy_call: &str, mercy_string: &str) -> String {
    match mercy_call {
        "sha256" => sha256_hash(mercy_string.to_string()),
        "md5" => md5_hash(mercy_string.to_string()),
        _ => unknown_msg("Unable to hash message")
    }
}

/* Public hexadecimal methods provided by Mercy */

/// Dump hexadecimal values of a file
/// 
/// `hex_dump` - Dumps hexadecimal data of a file
pub fn hex(mercy_call: &str, mercy_file: &str) -> String {
    match mercy_call {
        "hex_dump" => collect_file_hex(mercy_file),
        _ => unknown_msg("Unable to provide hexadecimal dump for file specified")
    }
}

/* Public malware and malicious detection */

/// Malware detection or malicious intent
/// 
/// `status` - Returns a status of 'malicious', 'unknown', or 'suspicious' from the InQuest API
pub fn malicious(mercy_call: &str, mercy_domain: &str) -> String {
    match mercy_call {
        "status" => malicious_domain_status(mercy_domain),
        _ => unknown_msg("Unable to classify domain")
    }
}

/* Public extra methods provided by Mercy */

/// Information about various data points
/// ### Methods
/// `internal_ip` - Returns the host (internal) ip address of the system
/// 
/// `system_info` - Returns numerous data points associated with the host system
/// 
/// `defang` - Returns a defanged url and/or ip address
/// 
/// `whois` - Returns WHOIS lookup information
/// 
/// `identify` - Attempt to identify an unknown string
/// 
/// `crack` - Attempt to crack an encrypted string
/// 
/// `detect_lang` - Attempt to detect the language in a string (beta)
pub fn extra(mercy_call: &str, mercy_choose: &str) -> String {
    match mercy_call {
        "internal_ip" => internal_ip(),
        "system_info" => system_info(mercy_choose),
        "defang" => defang(mercy_choose),
        "whois" => whois_lookup(mercy_choose),
        "identify" => identify_str(mercy_choose),
        "crack" => crack_str(mercy_choose),
        "detect_lang" => language_detection(mercy_choose),
        "parse_email" => parse_email_content(mercy_choose),
        _ => unknown_msg("Unable to provide the information you requested")
    }
}

/* Experimental methods that do not require a `prinln!()`. Instead, you just call the method within the code for stdout. */
// Example: experimental("zip", "/Users/name/Downloads/archive.zip");

/// Experimental functions that only accept stdout
/// ### Methods
/// `domain_gen` - Shuffle a provided string to construct a domain name (requires a "." followed by an extension)
/// 
/// `zip` - Extract a zip file
pub fn experimental(mercy_call: &str, mercy_choose: &str) {
    match mercy_call {
        "domain_gen" => domain_gen(mercy_choose),
        "zip" => zip_extract(mercy_choose),
        _ => println!("Unable to provide the information you requested")
    }
}

/* Decoding methods */

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

/* Encoding methods */

// Base64 encode
fn base64_encode(plaintext_msg: String) -> String {
    // Converts into bytes
    let encoded_msg = base64::encode(plaintext_msg.as_bytes());
    return encoded_msg.to_string();
}

/* Hashing methods */

// SHA256 hash
fn sha256_hash(plaintext_msg: String) -> String {
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

/* Extraction methods */

// Zip file extraction
fn zip_extract(filename: &str) {

    // Locate zip file
    let collect_file = File::open(Path::new(&*filename)).expect("Unable to locate file");

    // Begin zip file read
    let mut zip_archive = zip::ZipArchive::new(collect_file).expect("Failed to generate zip archive");

    // Iterate over zip file data
    for file in 0..zip_archive.len() {
        let mut file_idx = zip_archive.by_index(file).expect("Unable to build zip index");
        
        let out_path = match file_idx.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue
        };
        
        {
            // Checks the status of comments
            let comment = file_idx.comment();

            if !comment.is_empty() {
                println!("Comment located for file: {file}");
                println!("Comment: \n{comment}");
            }
        }

        // Validates if the extracted data is a file or directory
        if (*file_idx.name()).ends_with('/') {
            println!("[{}] Directory path: \"{}\"", file, out_path.display());

            // Creates internal zip directories
            fs::create_dir_all(&out_path).expect("Unable to create zip directories");
        } else {
            println!("[{}] File path: \"{}\" ({} bytes)", file, out_path.display(), file_idx.size());

            if let Some(zip_path) = out_path.parent() {
                if !zip_path.exists() {
                    fs::create_dir_all(zip_path).expect("Unable to create directories");
                }
            }

            // Builds the output path
            let mut out = File::create(&out_path).expect("Unable to create out_path");
            io::copy(&mut file_idx, &mut out).expect("Unable to copy contents");
        }

        // Handles permissions for Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file_idx.unix_mode() {
                fs::set_permissions(&out_path, fs::Permissions::from_mode(mode)).expect("Failure to update permissions");
            }
        }
    }
}

/* Hexadecimal manipulation */

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

/* Miscellaneous */

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

// Basic defang for URLs and IP addresses (or any string with a '.')
fn defang(ip_or_url: &str) -> String {
    return ip_or_url.replace(".", "[.]")
}

// WHOIS lookup for domain information
fn whois_lookup(url: &str) -> String {
    let whois_server = "whois.verisign-grs.com";
    let whois_port = 43;

    let mut stream = TcpStream::connect((whois_server, whois_port)).unwrap();
    stream.write_all(format!("{}\r\n", url).as_bytes()).unwrap();

    let mut whois_response = Vec::new();
    stream.read_to_end(&mut whois_response).unwrap();

    let res_to_str = from_utf8(&whois_response).unwrap();
    return res_to_str.to_string();
}

// Attempt to identify an unknown string
fn identify_str(data: &str) -> String {
    return Identifier::to_json(&Identifier::default().identify(data));
}

// Attempt to crack an encrypted string
fn crack_str(data: &str) -> String {
    let mut config = Config::default();
    // TODO: Allow user to specify timeout
    // Attempts to crack the string within 10 seconds
    config.timeout = 10;
    config.human_checker_on = false;
    
    let result = perform_cracking(data, config);

    if !result.is_none() {
        match result {
            Some(result) => return format!("{:?}", result.text),
            _ => return "Unable to crack string".to_string()
        }
    } else {
        return "Result is None".to_string()
    }
}

// Domain generation
fn domain_gen(url: &str) {

    let common_exts = [".com", ".io", ".co", ".ai", ".moe", ".org", ".edu", ".net", ".biz", ".ru", ".uk", ".au", ".de", ".in"];

    for i in 0..url.len() {
        let char_val = url.as_bytes()[i];

        for bit_switch in 0..8 {
            // Shuffles the character position
            let shuffle: u8 = char_val ^ 1 << bit_switch;

            if shuffle.is_ascii_alphanumeric()
            || shuffle as char == '-'
            && shuffle.to_ascii_lowercase()
            != char_val.to_ascii_lowercase() {
                
                let mut payload = url.as_bytes()[..i].to_vec();
                payload.push(shuffle);

                // Appends onto the Vec<u8> for parsing
                payload.append(&mut url.as_bytes()[i + 1..].to_vec());

                if let Ok(d) = String::from_utf8(payload) {

                    // Iterates over the preset extensions
                    for e in common_exts.iter() {

                        // Only returns if one of the extensions is present
                        if d.ends_with(e) {
                            println!("{}", d);
                        }
                    }

                    // Handles cases where an extension is not present
                    if !d.contains(".") {
                        // Apparently, this way it doesn't return something like examplecom (when providing "example.com")
                        if d.contains(".") {
                            println!("{}", d);
                        }
                    }
                }
            }
        }
    }
}

// Attempt to identify an unknown language (beta)
fn language_detection(input: &str) -> String {
    if let Some(info) = whatlang::detect_lang(&input) {
        return info.to_string();
    } else {
        return "Unable to detect language".to_string();
    }
}

// Quickly parse email content and return generic email values
fn parse_email_content(content: &str) -> String {
    let vec_binding = convert_file_to_vec(content);
    let parse_content: ParsedMail = parse_mail(&vec_binding).unwrap();

    let parsed_content: String = format!("Subject: {}\nFrom: {}\nTo: {}\nReturn Path: {}\nContent-Type: {}\nDate: {}",
    parse_content.headers.get_first_value("Subject").unwrap_or("N/A".to_string()).as_str(),
    parse_content.headers.get_first_value("From").unwrap_or("N/A".to_string()).as_str(),
    parse_content.headers.get_first_value("To").unwrap_or("N/A".to_string()).as_str(),
    parse_content.headers.get_first_value("Return-Path").unwrap_or("N/A".to_string()).as_str(),
    parse_content.headers.get_first_value("Content-Type").unwrap_or("N/A".to_string()).as_str(),
    parse_content.headers.get_first_value("Date").unwrap_or("N/A".to_string()).as_str());

    return parsed_content;
}

fn unknown_msg(custom_msg: &str) -> String {
    return format!("{}", custom_msg);
}

/* Malicious Detection */

// Handles the actual JSON response from the url request
#[tokio::main]
async fn malicious_domain_status(domain: &str) -> String {
    url_request(domain).await.unwrap();

    // Saves a local JSON file for parsing
    let json_file: &str = "/tmp/mercy_domain_review.json";
    
    let json_parse = {
        // Load the JSON file and convert to an easier to read format
        let json_convert = std::fs::read_to_string(&json_file).expect("Unable to locate file");
        serde_json::from_str::<Value>(&json_convert).unwrap()
    };

    // Deletes temporary JSON file
    fs::remove_file("/tmp/mercy_domain_review.json").unwrap();

    if &json_parse["data"][0]["classification"] == "MALICIOUS" {
        return "Malicious".to_string();
    } else if &json_parse["data"][0]["classification"] == "UNKNOWN" {
        return "Unknown".to_string();
    } else if &json_parse["data"][0]["classification"] == "SUSPICIOUS" {
        return "Suspicious".to_string();
    } else {
        return "No classification available".to_string();
    }
}

// Makes an async url request to the InQuest API for domain IOC info
async fn url_request(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    // Creates temp file for JSON data
    let mut file = File::create("/tmp/mercy_domain_review.json").expect("Failed to create file");

    // Constructs API request via InQuest
    let form_url = format!("https://labs.inquest.net/api/dfi/search/ioc/domain?keyword={}", url);

    // Data from API request
    let body = client.get(form_url).send()
        .await?
        .text()
        .await?;

    // Writes JSON data to the temp file
    file.write_all(body.as_bytes()).expect("Failed to write to file");

    Ok(body)
}

// Helper function to convert a file's raw contents to a Vec<u8> to read as bytes
fn convert_file_to_vec(file: &str) -> Vec<u8> {
    let mut f = File::open(&file).expect("Unable to locate file");
    let file_metadata = fs::metadata(&file).expect("Unable to read file metadata for buffer");
    let mut buffer = vec![0; file_metadata.len() as usize];
    f.read(&mut buffer).expect("Buffer overflow");

    return buffer
}