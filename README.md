<h1 align="center">
    <img src="https://raw.githubusercontent.com/azazelm3dj3d/mercy/main/assets/mercy_icon.png" width="40%" />
</h1>

📚 [Documentation](https://docs.rs/mercy/latest/mercy/)

![Mercy Status (Dev)](https://github.com/azazelm3dj3d/mercy/actions/workflows/dev.yml/badge.svg?branch=dev)
![Mercy Status (Main)](https://github.com/azazelm3dj3d/mercy/actions/workflows/main.yml/badge.svg?branch=main)

Mercy is an open-source Rust crate and CLI for building cybersecurity tools, assessment projects, and testing infrastructure. The goal is to create a sustainable project to make creating security tools in Rust a little easier.

## Usage
Since Mercy is a standard crate, it can easily be used in any project already initialized with Cargo. Simply add the following line to your `Cargo.toml` file:

```toml
mercy = "1.2.21"
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
    
    // Decode string "YXphemVsbGFicw=="
    mercy_decode("base64", "YXphemVsbGFicw==");
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
You can check if a domain (i.e. azazelm3dj3d.com) is currently classified as malicious using the InQuest API:

```rust
use mercy::mercy_malicious;

fn main() {
    mercy_malicious("status", "azazelm3dj3d.com");
}
```

### Miscellaneous Methods
Some extra methods have been included to assist with data collection. You can collect the internal IP address of the host system, defang a URL or IP address, run a WHOIS domain lookup, or dump host system information, specified by the user.

```rust
use mercy::mercy_extra;

fn main() {
    // Contains the internal ip address of the user's system
    // Second parameter MUST be an empty string to work
    mercy_extra("internal_ip", "");

    // This method is extensive, but the "all" parameter allows the user to dump everything
    mercy_extra("system_info", "all");

    // Defang an ip address or domain
    mercy_extra("defang", "azazelm3dj3d.com");

    // Run a WHOIS lookup on a domain
    mercy_extra("whois", "azazelm3dj3d.com");

    // Attempt to identify an unknown string
    mercy_extra("identify", "UCrlEbqe4ppk5dVIHzdxtC7g");
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

While Mercy is available as a crate, it also moonlights as a command line interface. This allows you to install the crate as normal via Cargo and use all available methods without building your own assessment tool(s).

You can find a few examples below on how to use the CLI.

You can run the CLI tool using the following syntax:
```bash
mercy -m <METHOD> -p <PROTOCOL> -i <STRING/FILE>
```

You can also run the help command if you need a refresher on the available arguments:
```bash
mercy -h
```

The available options are listed below:
```
-i, --input     Encoded/Plaintext string for decoding/encoding or location of the file for hex_dump

-m, --method    Chosen method for data manipulation (ex: decode)

-p, --protocol  Chosen protocol for data manipulation (ex: base64)

-e, --extended  View every available option within Mercy
```

### Examples

Here are some quick examples of use cases:

If you need to decode a string using the base64 protocol.
```bash
mercy -m decode -p base64 -i <EncodedString>
```

Print host system information, such as hostname, CPU cores, etc.
```bash
mercy -m sys -p system_info -i all
```

Take a plaintext string and encode it using an MD5 hash.
```bash
mercy -m hash -p md5 -i <PlaintextString>
```

Print the internal IP address of your host system.
```bash
mercy -m ip -p internal_ip
```

Quickly check if a domain is malicious.
```
mercy -m mal -p status -i "azazelm3dj3d.com"
```

If you're stuck, you can use this option to learn every command at your disposal from [Mercy](https://github.com/azazelm3dj3d/mercy):
```bash
mercy -e
```