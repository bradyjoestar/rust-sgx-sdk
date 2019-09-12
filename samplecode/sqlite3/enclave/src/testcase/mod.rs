use sqlitedb::dbcontext::DbContext;
use std::sync::Arc;

use crate::sqlitedb;

use core::borrow::BorrowMut;
use sqlite3::{DatabaseConnection, SqliteResult};
use sqlitedb::sqlops::lose;
use sqlitedb::teacher_asset::TeacherAsset;
use sqlitedb::teacherdao;

pub mod ocalldemo;

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

pub fn test_case_ten() {
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

pub fn test_case_eleven() {
    let rc_context = Arc::new(DbContext::new("test.sqlite"));

    let teacher_asset = TeacherAsset::new(&rc_context, true);

    teacher_asset.select_teacher_sum();
}

pub fn test_case_twelve() {
    let rc_context = Arc::new(DbContext::new("test.sqlite"));

    let teacher_asset = TeacherAsset::new(&rc_context, true);

    for i in 0..1000 {
        match teacher_asset.select_teachers(i) {
            Ok(y) => {
                println!("SELECT * FROM teacher");
                println!("Ok: {:?}", y);
            }
            Err(oops) => lose(format!("oops!: {:?}\n", oops).as_ref()),
        }
    }
}
