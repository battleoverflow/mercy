use mercy;

/*
    Decoding, encoding, and hashing
*/

#[test]
fn decodes_values() {
    assert_eq!(mercy::decode("base64", "YmF0dGxlb3ZlcmZsb3c="), "battleoverflow");
    assert_eq!(mercy::decode("rot13", "onggyrbiresybj"), "battleoverflow");
}

#[test]
fn encodes_values() {
    assert_eq!(mercy::encode("base64", "battleoverflow"), "YmF0dGxlb3ZlcmZsb3c=");
}

#[test]
fn hashes_values() {
    assert_eq!(mercy::hash("sha256", "battleoverflow"), "504543ca9f14824a934f7a6e76fac178cbf6652d2589edf664baf6479295a637");
    assert_eq!(mercy::hash("md5", "battleoverflow"), "4ce35faf1f8881ef6bfcedbd07e82fd2");
}

/*
    Hexadecimal dumping
*/

#[test]
fn dumps_hex_values() {
    let res = format!("()").to_string();
    assert_eq!(mercy::hex("hex_dump", "tests/assets/test.hex"), res.as_str());
}

/*
    Malicious status test
*/

#[test]
fn malicious_status_test() {
    assert_eq!(mercy::malicious("status", "https://example.com"), "No classification available");
}

/*
    Extra methods
*/

#[test]
fn extra_and_experimental_tests() {
    assert_eq!(mercy::extra("defang", "https://example.com"), "https://example[.]com");
}
