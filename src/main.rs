use std::env;
use std::fs::read;
use goblin::{Object, error::Error};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
        print_help();
        return;
    }

    if args.len() != 2 {
        println!("Usage: {} <binary_path>", args[0]);
        return;
    }

    let binary_path = &args[1];

    let binary_bytes = match read(binary_path) {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("Error reading binary file: {}", err);
            return;
        }
    };

    // Attempt to parse the binary file
    match Object::parse(&binary_bytes) {
        Ok(binary) => {
            // Print basic information about the binary
            match binary {
                Object::Elf(elf) => {
                    println!("Binary format: ELF");
                    println!("Architecture: {}", get_architecture_name_elf(elf.header.e_machine));
                    // Detect language based on ELF sections, symbols, etc.
                    detect_language(&binary_bytes);
                }
                Object::PE(pe) => {
                    println!("Binary format: PE");
                    println!("Architecture: {}", get_architecture_name_pe(pe.header.coff_header.machine));
                    // Detect language based on PE sections, symbols, etc.
                    detect_language(&binary_bytes);
                }
                _ => {
                    println!("Unknown binary format");
                }
            }
        }
        Err(err) => {
            match err {
                Error::Malformed(msg) => eprintln!("Error parsing binary: {}", msg),
                _ => eprintln!("Unexpected error: {:?}", err),
            }
        }
    }
}

fn get_architecture_name_elf(machine: u16) -> &'static str {
    match machine {
        0x03 => "x86",
        0x3E => "x86-64",
        // Could add more mappings in the future
        _ => "Unknown",
    }
}

fn get_architecture_name_pe(machine: u16) -> &'static str {
    match machine {
        0x014c => "x86",
        0x0200 => "IA64",
        0x8664 => "x86-64",
        // Could add more mappings in the future
        _ => "Unknown",
    }
}

fn detect_language(binary_bytes: &[u8]) {
    // This might be wrong sometimes

    // Check for common Rust patterns
    let contains_rust_pattern = binary_bytes.windows(4).any(|w| 
        w == b"fn " || w == b"use " || w == b"struct" || w == b"enum" || w == b"impl" ||
        w == b"'a" || w == b"trait" || w == b"match" || w == b"if " || w == b"loop");

    // Check for common C/C++ patterns
    let contains_c_cpp_pattern = binary_bytes.windows(2).any(|w| w == b"if" || w == b"for" || w == b"int");

    if contains_rust_pattern {
        println!("Programming Language: Rust");
    } else if contains_c_cpp_pattern {
        println!("Programming Language: C++");
    } else {
        println!("Programming Language: Unknown (Detection not conclusive)");
    }
}

fn print_help() {
    println!("Usage: bytez <binary_path>");
    println!("Bytez analyzes binary files to determine their format, architecture, and programming language.");
}