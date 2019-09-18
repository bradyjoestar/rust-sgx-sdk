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

#![crate_name = "unittestsampleenclave"]
#![crate_type = "staticlib"]
#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(target_env = "sgx")]
extern crate core;

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate rand;
extern crate sgx_rand;

extern crate crypto;
extern crate ring;

use sgx_types::*;

use std::string::String;
use std::vec::Vec;

mod test_file;
use test_file::*;
mod fileutils;
mod ringutils;

use sgx_rand::{Rng, StdRng};
use std::io::{BufRead, BufReader, Read, Write};

use std::untrusted::fs::remove_file;
use std::untrusted::fs::File;
use std::untrusted::fs::OpenOptions;

use std::sgxfs::{self, SgxFile};

#[no_mangle]
pub extern "C" fn test_main_entrance() -> size_t {
    // std::sgxfs
    //    test_sgxfs_write();
    //
    //    test_sgxfs_read();
    //    // std::fs
    //    test_fs_write();
    //
    //    test_fs_read();
    //
    //    test_fs_untrusted_fs_feature_enabled_write_file();
    //
    //    test_fs_untrusted_fs_feature_enabled_read_file();
    test_sgxfs_write();

    let (key, iv) = test_sgxfs_read();
    println!("The aes key is : {:?}", key);
    println!("The aes iv  is :  {:?}", iv);

    gelchain_file(key.clone(), iv.clone());

    sgx_status_t::SGX_SUCCESS as usize
}

pub fn test_sgxfs_write() {
    let mut key: [u8; 32] = [0; 32];
    let mut iv: [u8; 16] = [0; 16];
    let write_key_size;
    let write_iv_size;

    {
        let result = sgxfs::remove("sgx_key");
        let mut rand = StdRng::new().unwrap();
        rand.fill_bytes(&mut key);

        let opt = SgxFile::create("sgx_key");
        assert_eq!(opt.is_ok(), true);
        let mut file = opt.unwrap();

        let result = file.write(&key);

        assert_eq!(result.is_ok(), true);
        write_key_size = result.unwrap();
        println!("{}", write_key_size);
        println!("{:?}", key);
    }
    {
        let result = sgxfs::remove("sgx_iv");
        let mut rand = StdRng::new().unwrap();
        rand.fill_bytes(&mut iv);

        let opt = SgxFile::create("sgx_iv");
        assert_eq!(opt.is_ok(), true);
        let mut file = opt.unwrap();

        let result = file.write(&iv);

        assert_eq!(result.is_ok(), true);
        write_iv_size = result.unwrap();
        println!("{}", write_iv_size);
        println!("{:?}", iv);
    }
}

pub fn test_sgxfs_read() -> ([u8; 32], [u8; 16]) {
    let mut key: [u8; 32] = [0; 32];
    let mut iv: [u8; 16] = [0; 16];
    let read_key_size;
    let read_iv_size;
    {
        let opt = SgxFile::open("sgx_key");
        assert_eq!(opt.is_ok(), true);
        let mut file = opt.unwrap();

        let result = file.read(&mut key);
        assert_eq!(result.is_ok(), true);
        read_key_size = result.unwrap();
        println!("{}", read_key_size);
        println!("{:?}", key);
    }
    {
        let opt = SgxFile::open("sgx_iv");
        assert_eq!(opt.is_ok(), true);
        let mut file = opt.unwrap();

        let result = file.read(&mut iv);
        assert_eq!(result.is_ok(), true);
        read_iv_size = result.unwrap();
        println!("{}", read_iv_size);
        println!("{:?}", iv);
    }
    (key, iv)
}

fn gelchain_file(key: [u8; 32], iv: [u8; 16]) {
    // In a real program, the key and iv may be determined
    // using some other mechanism. If a password is to be used
    // as a key, an algorithm like PBKDF2, Bcrypt, or Scrypt (all
    // supported by Rust-Crypto!) would be a good choice to derive
    // a password. For the purposes of this example, the key and
    // iv are just random values.

    for i in 1..10 {
        let message = format!("Hello,world:{}", i);
        test_write_crypto_msg(message, key.clone(), iv.clone());
    }

    let file = File::open("foo.txt").unwrap();

    let mut fin = BufReader::new(file);

    for line in fin.lines() {
        let encrypted_message = line.unwrap();
        test_read_crypto_msg(encrypted_message, key.clone(), iv.clone());
    }


    println!("-----------------sign_and_verify begin-------------------");
    ringutils::cryptos::sign_and_verify();
    println!("-----------------sign_and_verify success-------------------");
}
