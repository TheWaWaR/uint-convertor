use clap::{Arg, App, AppSettings};
use num_bigint::BigUint;
use numext_fixed_uint::U256;

fn main() {
    let all_types = [
        "u32",
        "u64",
        "u128",
        "u160",
        "u224",
        "u256",
        "u384",
        "u512",
        "u520",
        "u1024",
        "u2048",
        "u4096",
    ];
    let endians = ["big", "little"];
    let matches = App::new("uint-convertor")
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("from")
                .long("from")
                .takes_value(true)
                .required(true)
                .help("From number")
        )
        .arg(
            Arg::with_name("from-radix")
                .long("from-radix")
                .takes_value(true)
                .possible_values(&["2", "8", "10", "16"])
                .help("From radix")
        )
        .arg(
            Arg::with_name("from-type")
                .long("from-type")
                .takes_value(true)
                .possible_values(&all_types)
                .help("From uint type")
        )
        .arg(
            Arg::with_name("from-endian")
                .long("from-endian")
                .takes_value(true)
                .possible_values(&endians)
                .default_value("big")
                .help("From endian")
        )
        .arg(
            Arg::with_name("to-type")
                .long("to-type")
                .takes_value(true)
                .possible_values(&all_types)
                .default_value("u256")
                .help("To uint type")
        )
        .arg(
            Arg::with_name("to-endian")
                .long("to-endian")
                .takes_value(true)
                .possible_values(&endians)
                .default_value("big")
                .help("To endian")
        )
        .get_matches();
    let from_str = matches.value_of("from").unwrap();
    let from_radix_str = matches
        .value_of("from-radix")
        .unwrap_or_else(|| {
            if from_str.len() >= 2 {
                match &from_str[0..2] {
                    "0x" | "0X" => "16",
                    "0o" | "0O" => "8",
                    "0b" | "0B" => "2",
                    _ => "10",
                }
            } else {
                "10"
            }
        });
    // let _from_type = matches.value_of("from-type").unwrap();
    let from_endian = matches.value_of("from-endian").unwrap();
    let to_type = matches.value_of("to-type").unwrap();
    let to_endian = matches.value_of("to-endian").unwrap();

    let from_radix: u32 = from_radix_str.parse().unwrap();
    let number = if from_endian == "big" {
        BigUint::parse_bytes(from_str.as_bytes(), from_radix).unwrap()
    } else {
        // TODO: from_radix_le, from_radix_be
        unimplemented!()
    };
    let target_bytes: Vec<u8> = match to_endian {
        "big" => {
            let bytes = number.to_bytes_be();
            match to_type {
                "u256" => {
                    let mut base = U256::default().to_be_bytes();
                    base[(32-bytes.len())..].copy_from_slice(&bytes[..]);
                    base.to_vec()
                }
                _ => unimplemented!()
            }
        },
        "little" => number.to_bytes_le(),
        _ => unreachable!()
    };
    println!("0x{}", hex::encode(&target_bytes));
}
