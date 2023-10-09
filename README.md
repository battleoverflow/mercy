<h1 align="center">
    <img src="https://raw.githubusercontent.com/azazelm3dj3d/mercy/main/assets/mercy_icon.png" width="40%" />
</h1>

📚 [Documentation](https://docs.rs/mercy/latest/mercy/)

![Workflow](https://github.com/azazelm3dj3d/mercy/actions/workflows/workflow.yml/badge.svg?branch=main)

Mercy is an open source Rust crate and CLI designed for building cybersecurity tools, assessment projects, and testing. The goal of the project is to make creating security tools in Rust more accessible and sustainable without complex algorithms.

## Usage
Since Mercy is a standard crate, it can easily be used in any project already initialized with Cargo. Simply add the following line to your `Cargo.toml` file:

```toml
mercy = "2.0.1"
```

Once the `Cargo.toml` file is updated, you can import the crate and use the provided methods by running `cargo run`. There are lots of different examples available below.

### Cryptographic Processes
Here's a quick example of decoding and encoding using the base64 protocol:

```rust
use mercy::{ decode, encode };

fn main() {
    // Encode string "azazelm3dj3d"
    encode("base64", "azazelm3dj3d");
    
    // Decode string "YXphemVsbGFicw=="
    decode("base64", "YXphemVsbGFicw==");
}
```

### Hexadecimal Dumping
Here's how to dump hexadecimal values in a single line using Mercy:

```rust
use mercy::hex;

fn main() {
    hex("hex_dump", "/Location/of/file");
}
```

### Malware/Malicious Detection
You can check if a domain (i.e. azazelm3dj3d.com) is currently classified as malicious using the InQuest API:

```rust
use mercy::malicious;

fn main() {
    malicious("status", "azazelm3dj3d.com");
}
```

### Miscellaneous Methods
Some extra methods have been included to assist with data collection. You can collect the internal IP address of the host system, defang a URL or IP address, run a WHOIS domain lookup, or dump host system information, specified by the user.

```rust
use mercy::extra;

fn main() {
    // Contains the internal ip address of the user's system
    // Second parameter MUST be an empty string to work
    extra("internal_ip", "");

    // This method is extensive, but the "all" parameter allows the user to dump everything
    extra("system_info", "all");

    // Defang an ip address or domain
    extra("defang", "azazelm3dj3d.com");

    // Run a WHOIS lookup on a domain
    extra("whois", "azazelm3dj3d.com");

    // Attempt to identify an unknown string
    extra("identify", "UCrlEbqe4ppk5dVIHzdxtC7g");

    // Attempt to crack an encrypted string
    extra("crack", "YXphemVsbTNkajNk");
}
```

You can also use the following parameters, replacing the "all" keyword under `system_info`:

- hostname
- cpu_cores
- cpu_speed
- os_release
- proc

There's also an experimental method, which means you'll receive everything through stdout without a `println!()`:

```rust
use mercy::experimental;

fn main() {
    // Shuffle a provided string to construct a domain name
    experimental("domain_gen", "example.com");
}
```

You can now also extract zip files within your script or via the CLI. This is another method that only prints to stdout:

```rust
use mercy::experimental;

fn main() {
    experimental("zip", "/Users/name/Downloads/archive.zip");
}
```

### More Info
If ever in doubt, feel free to run this special function to display more information about the crate.

```rust
use mercy::source;

fn main() {
    source();
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
```bash
mercy -m mal -p status -i "azazelm3dj3d.com"
```

Extract a zip file.
```bash
mercy -m zip_e -p zip -i "/Users/name/Downloads/archive.zip"
```

If you're stuck, you can use this option to learn every command at your disposal from [Mercy](https://github.com/azazelm3dj3d/mercy):
```bash
mercy -e
```