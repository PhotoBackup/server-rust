Join our online chat at [![Gitter](https://badges.gitter.im/gitterHQ/gitter.svg)](https://gitter.im/PhotoBackup)

#  The Rust PhotoBackup server implementation 

This is an experimental version of a PhotoBackup server implementation, made with Rust. It is not recommended to use it on a daiy basis, as it is far from complete and stable. Its main purpose is educational, for its own developer only.


## Installation
Then run the installer, which asks for the directory to save your pictures to and the server password:

    photobackup init <username>

This step creates a `.photobackup` file in the user's home directory,
containing:

* `BindAddress`, the IP address (default is `127.0.0.1`) ;
* `MediaRoot`, the directory where the pictures are written in ;
* `Password`, the SHA-512 hashed password ;
* `PasswordBcrypt`, a Bcrypt-ed version of your SHA-512 hashed password ;
* `Port`, the port (default is `8420`).

## Usage

Launch the server with:

    photobackup run <username>

By default, it runs on host `127.0.0.1`, port `8420`.

## Production

Don't put it in production!