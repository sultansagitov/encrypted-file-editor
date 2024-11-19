# Encrypted File Editor

A simple bash script that allows you to securely edit encrypted files. The script supports file creation, decryption, editing using your preferred editor, and re-encryption with the same password.

## Features
- Encrypt and decrypt files using OpenSSL with **AES-256-CBC** and **PBKDF2** for strong password-based encryption.
- Supports editing files with `$EDITOR` or falls back to `nano` if `$EDITOR` is not set.
- Automatically handles temporary files securely, ensuring sensitive data is not left on disk.

## Prerequisites
- **Bash**: The script is designed to run on Unix-like systems.
- **OpenSSL**: Ensure OpenSSL is installed on your system. Use `openssl version` to verify.
- **Text Editor**: Any text editor (e.g., `nano`, `vim`, `nvim`, `emacs`). The script defaults to `nano` if no editor is specified in the `$EDITOR` environment variable.

## Installation
1. Clone the repository:
```bash
git clone https://github.com/yourusername/encrypted-file-editor.git
cd encrypted-file-editor
```

2. Make the script executable:
```bash
chmod +x edit_encrypted_file.sh
```

3. Optionally, move it to a directory in your `PATH` for global access:
```bash
sudo mv edit_encrypted_file.sh /usr/local/bin/encrypt-edit
```

## Usage
The script takes one argument, the name of the encrypted file. It decrypts the file, allows you to edit it, and re-encrypts it with the same password.

### Syntax
```bash
./encr <encrypted_file>
```

### Examples

#### Editing an Existing Encrypted File
```bash
./encr my_secret_file.enc
```
1. Prompts for the password to decrypt the file.
2. Opens the decrypted file in your text editor (e.g., `$EDITOR` or `nano`).
3. Re-encrypts the file after editing using the same password.

#### Creating a New Encrypted File
```bash
./encr new_file.enc
```
1. If the file doesn’t exist, it creates an empty file for editing.
2. Prompts for a password to encrypt the new file.
3. Saves and encrypts the file after editing.

## Environment Variables
- `$EDITOR`: Specifies the text editor to use. Defaults to `nano` if not set.

To set a default editor (e.g., `vim`):
```bash
export EDITOR="vim"
```

## Security
- Uses **AES-256-CBC** cipher with **PBKDF2** for strong encryption.
- Securely handles passwords without storing them in plaintext.
- Cleans up temporary files after use.

## Dependencies
- [OpenSSL](https://www.openssl.org): Used for encryption and decryption.

## Limitations
- The script uses a single password for both decryption and encryption. Make sure to remember the password, as there is no recovery mechanism.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request if you have suggestions or improvements.
