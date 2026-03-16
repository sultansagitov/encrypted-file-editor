use rpassword::read_password;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    // 1. Argument Validation
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <encrypted_file>", args[0]);
        std::process::exit(1);
    }

    let encrypted_file = &args[1];
    let cipher = "aes-256-cbc";
    let temp_file_path = "/tmp/rust_encr_temp.tmp";

    // 2. Logic for Password Retrieval and Decryption
    // We assign the result of the block to 'password' to avoid unused_assignment warnings
    let password = if Path::new(encrypted_file).exists() {
        print!("Enter password to decrypt the file: ");
        io::stdout().flush()?;
        let pwd = read_password().expect("Failed to read password");

        let mut child = Command::new("openssl")
            .args([
                "enc", "-d", 
                &format!("-{}", cipher), 
                "-base64", "-pbkdf2", 
                "-pass", "stdin",
                "-in", encrypted_file,
                "-out", temp_file_path,
            ])
            .stdin(Stdio::piped())
            .spawn()?;

        // Send password to openssl's stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(pwd.as_bytes())?;
            stdin.write_all(b"\n")?;
        }

        if !child.wait()?.success() {
            eprintln!("Failed to decrypt the file. Check your password or format.");
            let _ = fs::remove_file(temp_file_path);
            std::process::exit(1);
        }
        println!("File decrypted successfully. Opening for editing...");
        pwd
    } else {
        println!("File does not exist. Creating a new file for editing.");
        fs::File::create(temp_file_path)?;

        print!("Enter a password to use for encrypting the new file: ");
        io::stdout().flush()?;
        let pwd = read_password().expect("Failed to read password");
        pwd
    };

    // 3. Launch Editor (Uses $EDITOR or defaults to nano)
    let editor = env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    Command::new(editor).arg(temp_file_path).status()?;

    // 4. Re-encrypt the edited file
    let mut child = Command::new("openssl")
        .args([
            "enc", "-e", 
            &format!("-{}", cipher), 
            "-base64", "-pbkdf2", 
            "-pass", "stdin",
            "-in", temp_file_path,
            "-out", encrypted_file,
        ])
        .stdin(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(password.as_bytes())?;
        stdin.write_all(b"\n")?;
    }

    if child.wait()?.success() {
        println!("File successfully encrypted as {}.", encrypted_file);
    } else {
        eprintln!("Failed to encrypt the file.");
    }

    // 5. Cleanup
    if Path::new(temp_file_path).exists() {
        fs::remove_file(temp_file_path)?;
    }

    Ok(())
}
