use mercy;

/*
    Decoding, encoding, and hashing
*/

#[test]
fn decodes_values() {
    assert_eq!(mercy::decode("base64", "YXphemVsbTNkajNk"), "azazelm3dj3d");
    assert_eq!(mercy::decode("rot13", "nmnmryz3qw3q"), "azazelm3dj3d");
}

#[test]
fn encodes_values() {
    assert_eq!(mercy::encode("base64", "azazelm3dj3d"), "YXphemVsbTNkajNk");
}

#[test]
fn hashes_values() {
    assert_eq!(mercy::hash("sha256", "azazelm3dj3d"), "88172dfbf2e8c6aa00bdbd9611ffc72b3910be1fac0ef2c43196502022df8cfa");
    assert_eq!(mercy::hash("md5", "azazelm3dj3d"), "4f85ebc7ef03bf7de13dd609f7b6a637");
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
    assert_eq!(mercy::malicious("status", "https://azazelm3dj3d.com"), "No classification available");
}

/*
    Extra methods
*/

#[test]
fn extra_and_experimental_tests() {
    assert_eq!(mercy::extra("defang", "https://azazelm3dj3d.com"), "https://azazelm3dj3d[.]com");
}