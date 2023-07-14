use base64::engine::general_purpose;
use base64::Engine;
use flate2::read::DeflateDecoder;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    // Path to the file to decompress and decode
    let file_path: Vec<_> = env::args().collect();

    if file_path.len() <= 1 ||  file_path.len() > 3 {
        println!("decoder <encoded_file> <desired_file>");
        return Ok(());
    }

    // Open the file
    let file = File::open(file_path[1].clone())?;
    let out_file = File::create(file_path[2].clone());

    // Create a buffered reader on the file
    let mut reader = BufReader::new(file);

    // Read the entire file into a string
    let mut encoded_data = String::new();
    reader.read_to_string(&mut encoded_data)?;

    // Trim any trailing newlines or whitespace
    let encoded_data = encoded_data.trim();

    // Decode the base64 data
    let decoded_data = general_purpose::STANDARD
        .decode(encoded_data)
        .expect("Failed to decode base64 data");

    // Create a DeflateDecoder with the decoded data
    let mut decoder = DeflateDecoder::new(&decoded_data[..]);

    // Initialize a Vec<u8> to store the decompressed data
    let mut decompressed_data = Vec::new();

    // Read the data from the decoder into the Vec<u8>
    decoder.read_to_end(&mut decompressed_data)?;

    // Write decompressed data to a file taken from arg[2]
    out_file.unwrap().write_all(&decompressed_data)?;

    Ok(())
}
