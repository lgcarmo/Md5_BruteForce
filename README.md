# MD5 Password Cracker

This simple Rust program is designed to crack MD5 hashed passwords by comparing the hash of each line in a specified file with a target MD5 hash.

## Install Rust

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
```bash
$ git clone https://github.com/lgcarmo/Md5_BruteForce.git
$ cd Md5_BruteForce

$ cargo b
```

## Usage

```bash
$ ./md5_bruteForce> <target_md5> <nome_do_arquivo>
```
