<h1 align="center">
    <img src="assets/mercy_banner_v1.png" />
    <br />
    Mercy
</h1>

<h3 align="center">
    📚 <a href="https://docs.rs/mercy/latest/mercy/">Documentation</a>
</h3>
<br>

Mercy is a public Rust crate created to assist with building cybersecurity frameworks, assessment tools, and numerous other projects. We hope to create a sustainable crate to make creating security tools in Rust a little easier.

## Usage
Since Mercy is a standard crate, it can easily be utilized in any project already initialized with Cargo.

Add the following line to your `Cargo.toml` file:
```toml
mercy = "1.1.13"
```

Once the `Cargo.toml` file has been updated, you can import the crate and use the provided methods by running `cargo run`.

### Cryptographic Processes
Here's a quick example for decoding and encoding using the base64 protocol:
```rust
use mercy::{
    mercy_decode,
    mercy_encode
};

fn main() {
    // Encode string "Hifumi1337"
    mercy_encode("base64", "Hifumi1337");
    
    // Decode string "SGlmdW1pMTMzNw=="
    mercy_decode("base64", "SGlmdW1pMTMzNw==");
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

### Miscellaneous Methods
Some extra methods have been included to assist with local data collection. You can currently collect the internal ip address of your device or dump certain information, specified by the user.
```rust
use mercy::mercy_extra;

fn main() {
    // Contains the internal ip address of the user's system
    mercy_extra("internal_ip", ""); // Second parameter MUST be an empty string to work

    // This method is extensive, but the "all" parameter allows the user to dump everything we have set in Mercy
    mercy_extra("system_info", "all");
}
```
You can also use the following paremeters, replacing the "all" keyword:

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