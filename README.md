# Rcrack

## Available algorithms


| Name        | Algorithm  | Crates.io |
|-------------|------------|-----------|
| `md5`       | MD5        | [![crates.io](https://img.shields.io/crates/v/md5.svg)](https://crates.io/crates/md5)      |
| `sha1`      | SHA-1      | [![crates.io](https://img.shields.io/crates/v/sha1.svg)](https://crates.io/crates/sha-1)   |
| `sha256`    | SHA-2 256  | [![crates.io](https://img.shields.io/crates/v/sha256.svg)](https://crates.io/crates/sha2)  |
| `sha512`    | SHA-2 512  | [![crates.io](https://img.shields.io/crates/v/sha256.svg)](https://crates.io/crates/sha2)  |
|`ripemd320`  | RIPEMD320  | [![crates.io](https://img.shields.io/crates/v/ripemd320.svg)](https://crates.io/crates/ripemd320)|


## Build From Source

### Prerequisites

* rust

### Building

Clone the repository and use cargo to generate a release build.
```sh
# git clone https://github.com/TheHolyTachanka/RCrack
# cd RCrack/
# cargo build --release
# mv target/release/rcrack
```

## Usage
```sh
# rcrack -d <WORDLIST-FILE-PATH> -h <HASH-TO-CRACK>
```

## License
This project is licensed under the terms of the MIT license. 
Check the [LICENSE](LICENSE.md) file out for license rights and limitations.

🍰
