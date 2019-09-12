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

extern crate dirs;
extern crate sgx_types;
extern crate sgx_urts;
use sgx_types::*;
use sgx_urts::SgxEnclave;

use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path;
use std::slice;
use std::str;
use std::str::FromStr;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate sqlite3;

mod buz;
mod beans;
mod sqlitedb;
use buz::buzlogic::buzfn;

static ENCLAVE_FILE: &'static str = "enclave.signed.so";
static ENCLAVE_TOKEN: &'static str = "enclave.token";

extern "C" {
    fn start_db(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        existed: uint8_t,
        testtype: uint8_t,
    ) -> sgx_status_t;
}

#[no_mangle]
pub extern "C" fn ocall_empty(
    inside_str: *const u8,
    inside_len: u32,
    p_result_str: *mut u8,
    maxlen: u32,
    p_result_len: *mut u32,
) -> sgx_status_t {
    println!("ocall_empty");

    let str_slice = unsafe { slice::from_raw_parts(inside_str, inside_len as usize) };
    let jsonstr = str::from_utf8(str_slice).unwrap();

    let resultstr = buzfn(jsonstr).as_bytes();

    let result_slice = unsafe { slice::from_raw_parts_mut(p_result_str, maxlen as usize) };

    let mut j = 0;
    for x in resultstr {
        result_slice[j] = *x;
        j = j + 1;
    }

    unsafe {
        *p_result_len = j as u32;
    };
    println!("inside_len is :{}", inside_len);
    sgx_status_t::SGX_SUCCESS
}

fn init_enclave() -> SgxResult<SgxEnclave> {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // Step 1: try to retrieve the launch token saved by last transaction
    //         if there is no token, then create a new one.
    //
    // try to get the token saved in $HOME */
    let mut home_dir = path::PathBuf::new();
    let use_token = match dirs::home_dir() {
        Some(path) => {
            println!("[+] Home dir is {}", path.display());
            home_dir = path;
            true
        }
        None => {
            println!("[-] Cannot get home dir");
            false
        }
    };

    let token_file: path::PathBuf = home_dir.join(ENCLAVE_TOKEN);;
    if use_token == true {
        match fs::File::open(&token_file) {
            Err(_) => {
                println!(
                    "[-] Open token file {} error! Will create one.",
                    token_file.as_path().to_str().unwrap()
                );
            }
            Ok(mut f) => {
                println!("[+] Open token file success! ");
                match f.read(&mut launch_token) {
                    Ok(1024) => {
                        println!("[+] Token file valid!");
                    }
                    _ => println!("[+] Token file invalid, will create new token file"),
                }
            }
        }
    }

    // Step 2: call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {
        secs_attr: sgx_attributes_t { flags: 0, xfrm: 0 },
        misc_select: 0,
    };
    let enclave = try!(SgxEnclave::create(
        ENCLAVE_FILE,
        debug,
        &mut launch_token,
        &mut launch_token_updated,
        &mut misc_attr
    ));

    // Step 3: save the launch token if it is updated
    if use_token == true && launch_token_updated != 0 {
        // reopen the file with write capablity
        match fs::File::create(&token_file) {
            Ok(mut f) => match f.write_all(&launch_token) {
                Ok(()) => println!("[+] Saved updated launch token!"),
                Err(_) => println!("[-] Failed to save updated launch token!"),
            },
            Err(_) => {
                println!("[-] Failed to save updated enclave token, but doesn't matter");
            }
        }
    }

    Ok(enclave)
}

fn main() {
    let mut args: Vec<_> = env::args().collect();
    //default max_conn is 30
    let mut test_type: u8 = 1;
    args.remove(0);
    while !args.is_empty() {
        match args.remove(0).as_ref() {
            "--test1" => {
                test_type = 1;
            }
            "--test2" => {
                test_type = 2;
            }
            "--test3" => {
                test_type = 3;
            }
            "--test4" => {
                test_type = 4;
            }
            "--test5" => {
                test_type = 5;
            }
            "--test6" => {
                test_type = 6;
            }
            "--test7" => {
                test_type = 7;
            }
            "--test8" => {
                test_type = 8;
            }
            "--test9" => {
                test_type = 9;
            }
            "--test10" => {
                test_type = 10;
            }
            "--test11" => {
                test_type = 11;
            }
            "--test12" => {
                test_type = 12;
            }
            _ => {
                panic!("Only --test* is accepted");
            }
        }
    }

    let dbfile = "test.db";
    let mut existed = 0;
    match fs::File::open(dbfile) {
        Err(_) => {
            existed = 0;
            println!("dbfile not existed");
        }
        _ => {
            existed = 1;
            println!("dbfile existed");
        }
    }

    let enclave = match init_enclave() {
        Ok(r) => {
            println!("[+] Init Enclave Successful {}!", r.geteid());
            r
        }
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        }
    };

    let mut retval = sgx_status_t::SGX_SUCCESS;

    let result = unsafe { start_db(enclave.geteid(), &mut retval, existed, test_type) };

    match result {
        sgx_status_t::SGX_SUCCESS => {}
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }

    println!("[+] say_something success...");

    enclave.destroy();
}
