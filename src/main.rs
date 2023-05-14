//! # Mercy CLI
//!
//! Mercy is an open source Rust crate and CLI designed for building cybersecurity tools, assessment projects, and immediate testing. The goal of the project is to make creating security tools in Rust more accessible and sustainable.
//! 

/*
    Project: Mercy (https://github.com/azazelm3dj3d/mercy)
    Author(s): azazelm3dj3d (https://github.com/azazelm3dj3d)
    License: BSD 2-Clause
*/

use mercy;
use clap::Parser;

use prettytable::{
    Cell,
    Row,
    Table
};

#[derive(Parser, Debug)]
#[command(name = "Mercy")]
#[command(version)] // Reads from Cargo.toml
#[command(about = "Mercy is an open-source Rust crate and CLI for building cybersecurity tools, assessment tools, and testing infastructure.", long_about = None)]
struct Args {

    /// Encoded/Plaintext string for decoding/encoding (ex: IaMStr1Ng) + location of the file for hex_dump
    #[arg(short, long)]
    #[clap(default_value = "")]
    input: String,

    /// Chosen method for data manipulation (ex: decode)
    #[arg(short, long)]
    #[clap(default_value = "")]
    method: String,

    /// Chosen protocol for data manipulation (ex: base64)
    #[arg(short, long)]
    #[clap(default_value = "")]
    protocol: String,

    /// View every available option within the Mercy
    #[arg(short, long)]
    extended: bool
}

// Creates a pretty output for the CLI
fn pretty_output(method: &str, protocol: &str, left_col: &str, right_col: &str) {
    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new(left_col),
        Cell::new(right_col)
    ]));

    table.add_row(Row::new(vec![
        Cell::new(method),
        Cell::new(protocol)
    ]));

    table.printstd();
}

fn main() {
    let args = Args::parse();

    // Extended help section for new users
    if args.extended {
        println!("\n=== Mercy CLI ===");
        pretty_output("encode\ndecode\nhash\nhex\nsys\nip\nmal\nd\nwho\nid\nc\nlang\nemail\ndg\nzip_e", "base64, rot13\nbase64, rot13\nmd5, sha2_256\nhex_dump\nsystem_info\ninternal_ip\nstatus\ndefang\nwhois\nidentify\ncrack\ndetect_lang\nparse_email\ndomain_gen\nzip", "Method(s)", "Protocol(s)");

        println!("\n=== Mercy CLI Extended ===");
        pretty_output("system_info", "hostname\ncpu_cores\ncpu_speed\nos_release\nproc\nall", "Protocol(s)", "Input(s)");

        // Example scenarios
        println!("\n=== Examples ===");
        println!("Print general information for the host system");
        println!("mercy -m sys -p system_info -i all\n");

        println!("Decode an encoded string using base64");
        println!("mercy -m decode -p base64 -i bWVyY3kgaXMgcmVhbGx5IGNvb2w=\n");

        println!("Check if a domain is malicious or not");
        println!("mercy -m mal -p status -i 'example.com'\n");

        println!("Identify an unknown string");
        println!("mercy -m id -p identify -i 'UCrlEbqe4ppk5dVIHzdxtC7g'\n");

        println!("Shuffle a provided string to construct a domain name");
        println!("mercy -m dg -p domain_gen -i 'example.com'\n");

        println!("Extract a zip file");
        println!("mercy -m zip_e -p zip -i '/Users/name/Downloads/archive.zip'\n");

        println!("Attempt to detect the language in a string (beta)");
        println!("mercy -m lang -p detect_lang -i 'mercy is a rust crate'\n");

        println!("Parse email content directly from the command line");
        println!("mercy -m email -p parse_email -i file.eml\n");
    } else {
        match args.method.as_str() {

            // Available arguments from the Mercy crate
            "decode" => println!("{}",mercy::decode(&args.protocol, &args.input)),
            "encode" => println!("{}", mercy::encode(&args.protocol, &args.input)),
            "hash"   => println!("{}", mercy::hash(&args.protocol, &args.input)),
            "hex"    => println!("{}", mercy::hex(&args.protocol, &args.input)),
            "sys"    => println!("{}", mercy::extra(&args.protocol, &args.input)),
            "ip"     => println!("{}", mercy::extra(&args.protocol, &args.input)),
            "d"      => println!("{}", mercy::extra(&args.protocol, &args.input)),
            "who"    => println!("{}", mercy::extra(&args.protocol, &args.input)),
            "id"     => println!("{}", mercy::extra(&args.protocol, &args.input)),
            "c"      => println!("{}", mercy::extra(&args.protocol, &args.input)),
            "lang"   => println!("{}", mercy::extra(&args.protocol, &args.input)),
            "email"  => println!("{}", mercy::extra(&args.protocol, &args.input)),
            "dg"     => mercy::experimental(&args.protocol, &args.input),
            "zip_e"  => mercy::experimental(&args.protocol, &args.input),
            "mal"    => println!("{}", mercy::malicious(&args.protocol, &args.input)),
            _        => println!("Unable to parse provided arguments")
        }
    }
}
