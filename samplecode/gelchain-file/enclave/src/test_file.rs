// Copyright (C) 2017-2019 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use fileutils::cryptos;
use sgx_rand::{Rng, StdRng};
use sgx_types::SGX_FILE;
use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::sgxfs::{self, SgxFile};
use std::string::*;
use std::untrusted::fs::remove_file;
use std::untrusted::fs::File;
use std::untrusted::fs::OpenOptions as ops;
use std::vec::Vec;

pub fn test_sgxfs_write() {
    let mut write_data: [u8; 16] = [0; 16];
    let write_size;

    {
        let result = sgxfs::remove("sgx_file");
        let mut rand = StdRng::new().unwrap();
        rand.fill_bytes(&mut write_data);

        let opt = SgxFile::create("sgx_file");
        assert_eq!(opt.is_ok(), true);
        let mut file = opt.unwrap();

        let result = file.write(&write_data);

        assert_eq!(result.is_ok(), true);
        write_size = result.unwrap();
        println!("{}", write_size);
        println!("{:?}", write_data);
    }
}

pub fn test_sgxfs_read() {
    let mut read_data: [u8; 16] = [0; 16];
    let read_size;

    {
        let opt = SgxFile::open("sgx_file");
        assert_eq!(opt.is_ok(), true);
        let mut file = opt.unwrap();

        let result = file.read(&mut read_data);
        assert_eq!(result.is_ok(), true);
        read_size = result.unwrap();
        println!("{}", read_size);
        println!("{:?}", read_data);
    }
}

pub fn test_fs_write() {
    {
        let mut f = ops::new()
            .create(true)
            .append(true)
            .open("foo.txt")
            .unwrap();

        let result = f.write_all(b"Hello, world!");
        assert!(result.is_ok());
    }
}

pub fn test_fs_read() {
    {
        let f = File::open("foo.txt");
        assert!(f.is_ok());

        let mut s = String::new();
        let result = f.unwrap().read_to_string(&mut s);
        assert!(result.is_ok());
        println!("{}", s);
    }
}

pub fn test_fs_untrusted_fs_feature_enabled_write_file() {
    {
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open("foo_unfs.txt")
            .unwrap();

        println!("test_fs_untrusted_fs_feature_enabled_write_file");

        for i in 1..182362 {
            let result = f.write_all(b"Hello, world!This is a simple file write test\n");
            assert!(result.is_ok());
        }
        f.write_all(b"\n");
    }
}

pub fn test_fs_untrusted_fs_feature_enabled_read_file() {
    {
        println!("test_fs_untrusted_fs_feature_enabled_read_file");

        let f = fs::File::open("foo_unfs.txt");
        assert!(f.is_ok());

        let mut s = String::new();
        let result = f.unwrap().read_to_string(&mut s);
        assert!(result.is_ok());
        println!("the length of s is {}", s.len());
    }
}

pub fn test_write_crypto_msg(message: String, key: [u8; 32], iv: [u8; 16]) {
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open("foo.txt")
        .unwrap();

    let encrypted_data = cryptos::encrypt(message.as_bytes(), &key, &iv)
        .ok()
        .unwrap();

    let mut encrypted_string: Vec<String> = Vec::new();

    for i in encrypted_data.iter() {
        let a = format!("{}", i);
        encrypted_string.push(a);
    }

    let mut output = String::new();
    let mut looptime = 0;
    for unit in encrypted_string.iter() {
        output.push_str(unit);
        looptime = looptime + 1;
        if looptime != encrypted_string.len() {
            output.push_str(",")
        }
    }
    output.push('\n');

    if !could_write("foo.txt", output.as_ref()) {
        panic!("exceed the maximum size");
    } else {

    }

    let result = f.write_all(output.as_bytes());
}

pub fn test_read_crypto_msg(message: String, key: [u8; 32], iv: [u8; 16]) {
    let f = fs::File::open("foo.txt");
    assert!(f.is_ok());

    let v: Vec<&str> = message.split(',').collect();
    let mut encrypte_buffer: Vec<u8> = vec![];
    for unit in v.iter() {
        encrypte_buffer.push(unit.parse().unwrap())
    }

    let decrypted_data = cryptos::decrypt(&encrypte_buffer[..], &key, &iv)
        .ok()
        .unwrap();

    let s = match String::from_utf8(decrypted_data) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("the decrypt data is : {}", s);
}

pub fn could_write(path: &str, write_string: &str) -> bool {
    let f = fs::File::open(path);
    let mut s = String::new();
    let result = f.unwrap().read_to_string(&mut s);

    //sgx could read 1MB data
    if (s.len() + write_string.len() >= 8388608) {
        false
    } else {
        true
    }
}
