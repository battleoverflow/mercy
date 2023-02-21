<h1 align="center">
    <img src="assets/mercy_icon.png" width="40%" />
</h1>

📚 [Documentation](https://docs.rs/mercy/latest/mercy/)

Mercy is a public Rust crate created to assist with building cybersecurity frameworks (offensive and defensive) and assessment tools. The goal is to create a sustainable crate to make creating security tools in Rust a little easier.

## Usage
Since Mercy is a standard crate, it can easily be utilized in any project already initialized with Cargo.

Add the following line to your `Cargo.toml` file:
```toml
mercy = "1.2.19"
```

Once the `Cargo.toml` file is updated, you can import the crate and use the provided methods by running `cargo run`. There are lots of different examples available below.

### Cryptographic Processes
Here's a quick example of decoding and encoding using the base64 protocol:
```rust
use mercy::{
    mercy_decode,
    mercy_encode
};

fn main() {
    // Encode string "azazelm3dj3d"
    mercy_encode("base64", "azazelm3dj3d");
    
    // Decode string "YXphemVsbTNkajNk"
    mercy_decode("base64", "YXphemVsbTNkajNk");
}
```

### Hexadecimal Dumping
Here's how to dump hexadecimal values in a single line using Mercy:
```rust
use mercy::mercy_hex;

fn main() {
    mercy_hex("hex_dump", "/Location/of/file");
}
```

### Malware/Malicious Detection
You can check if a domain (i.e. google.com) is currently classified as malicious using the InQuest API:
```rust
use mercy::mercy_malicious;

fn main() {
    mercy_malicious("status", "example.com");
}
```

### Miscellaneous Methods
Some extra methods have been included to assist with data collection. You can currently collect the internal IP address of the host system, defang a URL or IP address, run a WHOIS domain lookup, or dump host system information, specified by the user.
```rust
use mercy::mercy_extra;

fn main() {
    // Contains the internal ip address of the user's system
    mercy_extra("internal_ip", ""); // Second parameter MUST be an empty string to work

    // This method is extensive, but the "all" parameter allows the user to dump everything we have set in Mercy
    mercy_extra("system_info", "all");

    // Method now available which allows you to defang a url or ip address
    mercy_extra("defang", "example.com");

    // Run a WHOIS lookup for a domain
    mercy_extra("whois", "example.com");
}
```
You can also use the following parameters, replacing the "all" keyword under `system_info`:

- hostname
- cpu_cores
- cpu_speed
- os_release
- proc

### More Info
If ever in doubt, feel free to run this special function to display more information about the crate.
```rust
use mercy::source;

fn main() {
    mercy_source();
}
```

## CLI

Once the executable has been downloaded, you can run the CLI tool using the following syntax:
```bash
./mercy-cli -m <METHOD> -p <PROTOCOL> -i <STRING/FILE>
```

You can also run the help command if you need a refresher on the available arguments:
```bash
./mercy-cli -h
```

The available options are listed below:
```
-i, --input     Encoded/Plaintext string for decoding/encoding (ex: IaMStr1Ng) + location of the file for hex_dump

-m, --method    Chosen method for data manipulation (ex: decode)

-p, --protocol  Chosen protocol for data manipulation (ex: base64)

-e, --extended  View every available option within the Mercy CLI
```

### Examples

Here are some quick examples of use cases:

If you need to decode a string using the base64 protocol.
```bash
./mercy -m decode -p base64 -i <EncodedString>
```

Print host system information, such as hostname, cpu cores, etc.
```bash
./mercy -m sys -p system_info -i all
```

Take a plaintext string and encode it using MD5.
```bash
./mercy -m hash -p md5 -i <PlaintextString>
```

Print the internal IP address of your host system.
```bash
./mercy -m ip -p internal_ip
```

Quickly check if a domain is malicious.
```
./mercy -m mal -p status -i "example.com"
```

If you're stuck, you can use this option to learn every command at your disposal from [Mercy](https://github.com/azazelm3dj3d/mercy):
```bash
./mercy -e
```