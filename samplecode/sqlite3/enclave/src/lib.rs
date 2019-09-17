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

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]
#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

use sgx_types::*;
use std::io::{self, Write};
use std::slice;
use std::string::String;
use std::vec::Vec;

extern crate serde;
extern crate sqlite3;
#[macro_use]
extern crate serde_derive;

extern crate sgx_untrusted_time;

use sqlitedb::dbcontext::DbContext;
use std::sync::Arc;

use core::borrow::BorrowMut;
use sqlite3::{DatabaseConnection, SqliteResult};
use sqlitedb::sqlops::lose;
use sqlitedb::teacher_asset::TeacherAsset;
use sqlitedb::teacherdao;

mod beans;
mod sqlitedb;
mod testcase;

#[no_mangle]
pub extern "C" fn start_db(existed: uint8_t, testtype: uint8_t) -> sgx_status_t {
    //call start_db;
    println!("start_db");
    println!("testtype is {}", testtype);

    if testtype == 1 {
        testcase::test_case_one();
    } else if testtype == 2 {
        testcase::test_case_two();
    } else if testtype == 3 {
        testcase::test_case_three();
    } else if testtype == 4 {
        testcase::test_case_four();
    } else if testtype == 5 {
        testcase::test_case_five();
    } else if testtype == 6 {
        testcase::test_case_six();
    } else if testtype == 7 {
        testcase::test_case_seven();
    } else if testtype == 8 {
        testcase::test_case_eight();
    } else if testtype == 9 {
        testcase::test_case_nine();
    } else if testtype == 10 {
        testcase::test_case_ten();
    } else if testtype == 11 {
        testcase::test_case_eleven();
    } else if testtype == 12 {
        testcase::test_case_twelve()
    } else if testtype == 13 {
        testcase::ocalldemo::test_case_thirteen();
    }

    sgx_status_t::SGX_SUCCESS
}
