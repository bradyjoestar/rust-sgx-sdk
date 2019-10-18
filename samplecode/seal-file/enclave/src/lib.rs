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

#![crate_name = "sealfileenclave"]
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
extern crate sgx_tseal;

use sgx_types::*;

use std::string::String;
use std::vec::Vec;

mod test_file;
use test_file::*;
mod fileutils;
mod ringutils;
use sgx_tseal::*;
use sgx_types::*;
use sgx_types::marker::*;

use sgx_rand::{Rng, StdRng};
use std::io::{BufRead, BufReader, Read, Write};

use std::untrusted::fs::remove_file;
use std::untrusted::fs::File;
use std::untrusted::fs::OpenOptions;

use std::sgxfs::{self, SgxFile};

fn to_sealed_log<T: Copy + ContiguousMemory>(sealed_data: &SgxSealedData<T>,
                                             sealed_log: * mut u8,
                                             sealed_log_size: u32)
                                             -> Option<* mut sgx_sealed_data_t> {
    unsafe {
        sealed_data.to_raw_sealed_data_t(sealed_log as * mut sgx_sealed_data_t, sealed_log_size)
    }
}

fn from_sealed_log<'a, T: Copy + ContiguousMemory>(sealed_log: * mut u8, sealed_log_size: u32) -> Option<SgxSealedData<'a, T>> {
    unsafe {
        SgxSealedData::<T>::from_raw_sealed_data_t(sealed_log as * mut sgx_sealed_data_t, sealed_log_size)
    }
}

#[derive(Copy, Clone, Default, Debug)]
struct RandData {
    key: u32,
    rand: [u8; 16],
}

unsafe impl ContiguousMemory for RandData {}

pub fn test_seal_write(){
    println!("wenbin test seal");

    let mut data = RandData::default();
    data.key = 0x1234;
    let mut rand = StdRng::new().unwrap();
    rand.fill_bytes(&mut data.rand);

    let aad: [u8; 0] = [0_u8; 0];
    let sealed_data = SgxSealedData::<RandData>::seal_data(&aad, &data).unwrap();

    let mut sealed_log_arr:[u8;2048] = [0;2048];
    let sealed_log = sealed_log_arr.as_mut_ptr();
    let sealed_log_size : u32 = 2048;
    let opt = to_sealed_log(&sealed_data, sealed_log, sealed_log_size);
    assert_eq!(opt.is_some(), true);

    {
        let result = sgxfs::remove("sgx_key");
        let opt = SgxFile::create("sgx_key");
        assert_eq!(opt.is_ok(), true);
        let mut file = opt.unwrap();

        let result = file.write(&sealed_log_arr);

        assert_eq!(result.is_ok(), true);
        let write_key_size = result.unwrap();
        println!("{}", write_key_size);
    }

    println!("{:?}",data);

}

pub fn test_seal_read(){
    let mut key: [u8;2048] = [0; 2048];
    let sealed_log_size : u32 = 2048;
    {
        let read_key_size;
        {
            let opt = SgxFile::open("sgx_key");
            assert_eq!(opt.is_ok(), true);
            let mut file = opt.unwrap();

            let result = file.read(&mut key);
            assert_eq!(result.is_ok(), true);
            read_key_size = result.unwrap();
            println!("{}", read_key_size);
        }
    }
    let key_log = key.as_mut_ptr();

    let sealed_data = from_sealed_log::<RandData>(key_log, sealed_log_size).unwrap();
    let unsealed_data = sealed_data.unseal_data().unwrap();
    let udata = unsealed_data.get_decrypt_txt();

    println!("{:?}",udata);
}

#[no_mangle]
pub extern "C" fn test_main_entrance() -> size_t {
    test_seal_write();
    test_seal_read();

    sgx_status_t::SGX_SUCCESS as usize
}

