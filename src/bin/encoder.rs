use base64::engine::general_purpose;
use base64::Engine;
use flate2::write::DeflateEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    // Path to the file to compress and encode
    let file_path: Vec<_> = env::args().collect();

    // Open the file
    let file = File::open(file_path[1].clone())?;

    // Create a buffered reader on the file
    let reader = BufReader::new(file);

    // Initialize a Vec<u8> to store the compressed data
    let mut compressed_data = Vec::new();

    // Initialize the DeflateEncoder with the highest level of compression
    let mut encoder = DeflateEncoder::new(&mut compressed_data, Compression::best());

    // Read the data from the reader and write it to the encoder
    for byte in reader.bytes() {
        let byte = byte?;
        encoder.write_all(&[byte])?;
    }

    // Finalize the encoding to ensure all data is written
    encoder.finish()?;

    // Encode the compressed data to base64
    let encoded_data = general_purpose::STANDARD.encode(&compressed_data);

    // Print the base64 encoded, compressed data
    println!("{encoded_data}");

    Ok(())
}
