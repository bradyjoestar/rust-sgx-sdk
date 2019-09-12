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

#[no_mangle]
pub extern "C" fn start_db(existed: uint8_t, testtype: uint8_t) -> sgx_status_t {
    //call start_db;
    println!("start_db");
    println!("testtype is {}", testtype);

    if testtype == 1 {
        test_case_one();
    } else if testtype == 2 {
        test_case_two();
    } else if testtype == 3 {
        test_case_three();
    } else if testtype == 4 {
        test_case_four();
    } else if testtype == 5 {
        test_case_five();
    } else if testtype == 6 {
        test_case_six();
    } else if testtype == 7 {
        test_case_seven();
    } else if testtype == 8 {
        test_case_eight();
    } else if testtype == 9 {
        test_case_nine();
    } else if testtype == 10{
        test_case_ten();
    } else if testtype == 11{
        test_case_eleven();
    } else if  testtype == 12{
        test_case_twelve()
    }

    sgx_status_t::SGX_SUCCESS
}

pub fn test_case_one() {
    let rc_context = Arc::new(DbContext::new("test.sqlite"));

    let teacher_asset = TeacherAsset::new(&rc_context, false);

    for i in 1..200 {
        teacher_asset.set_teacher_asset(i);
    }

    //query
    match teacher_asset.select_teacher_list() {
        Ok(y) => {
            println!("SELECT * FROM teacher");
            println!("Ok: {:?}", y);
        }
        Err(oops) => lose(format!("oops!: {:?}\n", oops).as_ref()),
    }
}

pub fn test_case_two() {
    let mut conn;
    match sqlitedb::sqlite::start_db(0, 0) {
        Ok(x) => conn = x,
        _ => panic!("create database failed"),
    }

    teacherdao::base_teacher_ops(&mut conn, &true, 1000);
}

pub fn test_case_three() {
    let rc_context = Arc::new(DbContext::new("test.sqlite"));

    let teacher_asset = TeacherAsset::new(&rc_context, false);

    for i in 1..200 {
        teacher_asset.set_teacher_asset(i);

        match teacher_asset.select_teacher_list() {
            Ok(y) => {
                println!("SELECT * FROM teacher");
                println!("Ok: {:?}", y);
            }
            Err(oops) => lose(format!("oops!: {:?}\n", oops).as_ref()),
        }
    }
}

pub fn test_case_four() {
    let rc_context = Arc::new(DbContext::new("test.sqlite"));

    let teacher_asset = TeacherAsset::new(&rc_context, false);

    for i in 1..200 {
        teacher_asset.set_teacher_asset(i);
    }

    match teacher_asset.select_teacher_list() {
        Ok(y) => {
            println!("SELECT * FROM teacher");
            println!("Ok: {:?}", y);
        }
        Err(oops) => lose(format!("oops!: {:?}\n", oops).as_ref()),
    }
}

pub fn test_case_five() {
    let rc_context = Arc::new(DbContext::new("test.sqlite"));

    let teacher_asset = TeacherAsset::new(&rc_context, false);

    for i in 1..2000 {
        teacher_asset.set_teacher_asset(i);

        match teacher_asset.select_teacher_list() {
            Ok(y) => {
                println!("SELECT * FROM teacher");
                println!("Ok: {:?}", y);
            }
            Err(oops) => lose(format!("oops!: {:?}\n", oops).as_ref()),
        }
    }

    //1247 success

    //id: 1253 }]
    //1254
    //insert execute update failed step [disk I/O error] (code: 10)
    //sql: INSERT INTO teacher (id, street,city,sendstatus,datatype,ops,age,clientid,indexid)
    //                           VALUES (1254, 'streett', 'cityt','sendstatust', 'datatypet', 'insert',1254, 10000,1254)
    //insert 1254 data
    //thread panicked at 'called `Result::unwrap()` on an `Err` value: SqliteError { kind: SQLITE_IOERR, desc: "step", detail: Some("disk I/O error") }', src/libcore/result.rs:999:5
    //Illegal instruction (core dumped)
}

pub fn test_case_six() {
    let rc_context = Arc::new(DbContext::new("test.sqlite"));

    let teacher_asset = TeacherAsset::new(&rc_context, false);

    for i in 1..1200 {
        teacher_asset.set_teacher_asset(i);

        teacher_asset.select_teacher_sum();
    }
    //1247 success

    //insert execute update failed step [disk I/O error] (code: 10)
    //sql: INSERT INTO teacher (id, street,city,sendstatus,datatype,ops,age,clientid,indexid)
    //                           VALUES (1247, 'streett', 'cityt','sendstatust', 'datatypet', 'insert',1247, 10000,1247)
    //insert 1247 data
    //SELECT sum(clientid) FROM teacher
    //clientid sum is 12470000

    //insert 1253 data
    //SELECT sum(clientid) FROM teacher
    //clientid sum is 12530000
    //1254
    //insert execute update failed step [disk I/O error] (code: 10)
    //sql: INSERT INTO teacher (id, street,city,sendstatus,datatype,ops,age,clientid,indexid)
    //                           VALUES (1254, 'streett', 'cityt','sendstatust', 'datatypet', 'insert',1254, 10000,1254)
    //insert 1254 data
    //SELECT sum(clientid) FROM teacher
    //thread panicked at 'Box<Any>', src/sqlitedb/teacher_asset.rs:117:26
    //Illegal instruction (core dumped)
}

pub fn test_case_seven() {
    let rc_context = Arc::new(DbContext::new("test.sqlite"));

    let teacher_asset = TeacherAsset::new(&rc_context, true);

    teacher_asset.select_teacher_sum();

    println!("try to insert again");

    for i in 1300..3500 {
        teacher_asset.set_teacher_asset(i);
        teacher_asset.select_teacher_sum();
    }
}

pub fn test_case_eight() {
    let rc_context = Arc::new(DbContext::new("test.sqlite"));

    let teacher_asset = TeacherAsset::new(&rc_context, true);

    teacher_asset.select_teacher_sum();

    println!("try to insert again");

    for i in 4000..4800 {
        teacher_asset.set_teacher_asset(i);
        teacher_asset.select_teacher_sum();
    }
}

pub fn test_case_nine() {
    let rc_context = Arc::new(DbContext::new("test.sqlite"));

    let teacher_asset = TeacherAsset::new(&rc_context, true);

    teacher_asset.select_teacher_sum();

    println!("try to insert again");

    for i in 1200..1300 {
        teacher_asset.set_teacher_asset(i);
        teacher_asset.select_teacher_sum();
    }
}

pub fn test_case_ten(){
    let rc_context = Arc::new(DbContext::new("test.sqlite"));

    let teacher_asset = TeacherAsset::new(&rc_context, false);

    teacher_asset.select_teacher_sum();

    for i in 140..150 {
        teacher_asset.set_teacher_asset(i);
        teacher_asset.select_teacher_sum();
    }

    match teacher_asset.select_teacher_list() {
        Ok(y) => {
            println!("SELECT * FROM teacher");
            println!("Ok: {:?}", y);
        }
        Err(oops) => lose(format!("oops!: {:?}\n", oops).as_ref()),
    }
}


pub fn test_case_eleven(){
    let rc_context = Arc::new(DbContext::new("test.sqlite"));

    let teacher_asset = TeacherAsset::new(&rc_context, true);

    teacher_asset.select_teacher_sum();
}

pub fn test_case_twelve(){
    let rc_context = Arc::new(DbContext::new("test.sqlite"));

    let teacher_asset = TeacherAsset::new(&rc_context, true);

    for i in 0..1000{
        match teacher_asset.select_teachers(i) {
            Ok(y) => {
                println!("SELECT * FROM teacher");
                println!("Ok: {:?}", y);
            }
            Err(oops) => lose(format!("oops!: {:?}\n", oops).as_ref()),
        }
    }
}