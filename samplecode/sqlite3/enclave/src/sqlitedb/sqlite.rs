use sgx_types::*;
use std::io::{self, BufReader, Read, Write};
use std::prelude::v1::*;
use std::slice;
use std::vec::Vec;

use sqlite3::{DatabaseConnection, SqliteResult};
use sqlitedb::opening::base_test;
use sqlitedb::sqlops;

pub fn start_db(existed: uint8_t, num: i32) -> SqliteResult<DatabaseConnection> {
    // A sample &'static string
    let rust_raw_string = "This is a in-Enclave ";
    // An array
    println!("{}", &rust_raw_string);

    let mut conns;
    match sqlops::get_database_conn() {
        Ok(x) => {
            conns = x;
            println!("sqlite opening test:");
            base_test(&mut conns, existed, num);
            Ok(conns)
        }
        _ => {
            panic!("connect database failed");
        }
    }
}
