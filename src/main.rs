use std::fs;
use std::path::PathBuf;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the public key file
    #[arg(short, long)]
    pub_key_file_path: PathBuf,
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn read_and_encode_pub_key(pub_key_file_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let pub_key_bytes = fs::read(pub_key_file_path)?; // Read the file as bytes
    let cut_pub_key_bytes = &pub_key_bytes[4..]; // Cut the first 4 bytes

    let pub_key = STANDARD.encode(cut_pub_key_bytes);
    Ok(pub_key)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let pub_key = read_and_encode_pub_key(&args.pub_key_file_path)?;

    if args.verbose {
        // Output additional information
        println!("Public Key File Path: {:?}", args.pub_key_file_path);
    }

    println!("{}", pub_key);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_read_and_encode_pub_key() {
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();

        let fake_key_data = vec![
            0x00, 0x01, 0x02, 0x03,  // Garbage bytes
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88  // Real public key data
        ];
        temp_file.write_all(&fake_key_data).unwrap();

        let pub_key_path = temp_file.path().to_path_buf();
        let result = read_and_encode_pub_key(&pub_key_path).unwrap();

        let expected_base64_key = STANDARD.encode(&fake_key_data[4..]);
        assert_eq!(result, expected_base64_key);
    }
}