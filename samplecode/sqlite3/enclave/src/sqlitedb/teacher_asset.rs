use beans::teacher::Teacher;
use sqlite3::{PreparedStatement, QueryFold, ResultRowAccess, SqliteResult};
use sqlitedb::dbcontext::DbContext;
use std::prelude::v1::*;
use std::string::String;
use std::sync::Arc;

#[derive(Clone)]
pub struct TeacherAsset {
    pub db_context: Arc<DbContext>,
}

#[allow(unused_variables)]
impl TeacherAsset {
    pub fn new(context: &Arc<DbContext>, existed: bool) -> TeacherAsset {
        let teacher_asset = TeacherAsset {
            db_context: context.clone(),
        };
        if !existed {
            teacher_asset.init_table();
        }
        teacher_asset
    }

    fn init_table(&self) {
        let sql = "CREATE TABLE teacher (
                 id              SERIAL PRIMARY KEY,
                 street          VARCHAR NOT NULL,
                 city            VARCHAR NOT NULL,
                 sendstatus      VARCHAR NOT NULL,
                 datatype        VARCHAR NOT NULL,
                 ops             VARCHAR NOT NULL,
                 age             integer,
                 clientid        integer,
                 indexid         integer
               )";
        self.db_context.exec(sql);
    }

    pub fn set_teacher_asset(&self, num: i32) -> String {
        for (_i, j) in (num..num + 1).enumerate() {
            println!("{}", j);
            let teacher = Teacher {
                id: j,
                street: "streett".to_string(),
                city: "cityt".to_string(),
                sendstatus: "sendstatust".to_string(),
                datatype: "datatypet".to_string(),
                ops: "insert".to_string(),
                age: j,
                clientid: 10000,
                indexid: j,
            };

            let sql = format!(
                "INSERT INTO teacher (id, street,city,sendstatus,datatype,ops,age,clientid,indexid)
                           VALUES ({}, '{}', '{}','{}', '{}', '{}',{}, {},{})",
                &teacher.id,
                &teacher.street,
                &teacher.city,
                &teacher.sendstatus,
                &teacher.datatype,
                &teacher.ops,
                &teacher.age,
                &teacher.clientid,
                &teacher.indexid,
            );

            if !self.db_context.execute(sql.as_str()) {}
            println!("insert {} data", j);
        }
        String::from("success")
    }

    pub fn select_teacher_list(&self) -> SqliteResult<Vec<Teacher>> {
        //    select teacher
        let mut stmt: PreparedStatement = self.db_context.query("SELECT * FROM teacher")?;

        let snoc = |x, mut xs: Vec<_>| {
            xs.push(x);
            xs
        };

        let mut ppl;
        ppl = stmt
            .query_fold(&[], vec![], |row, ppl| {
                Ok(snoc(
                    Teacher {
                        id: row.get(0),
                        street: row.get(1),
                        city: row.get(2),
                        sendstatus: row.get(3),
                        datatype: row.get(4),
                        ops: row.get(5),
                        age: row.get(6),
                        clientid: row.get(7),
                        indexid: row.get(8),
                    },
                    ppl,
                ))
            })
            .unwrap();
        Ok(ppl)
    }

    pub fn select_teacher_sum(&self) {
        //select teacher sum(clientid)

        println!("SELECT sum(clientid) FROM teacher");
        let mut stmt2 = self
            .db_context
            .conn
            .borrow_mut()
            .prepare("SELECT sum(clientid) FROM teacher")
            .unwrap();
        let mut results = stmt2.execute();
        match results.step() {
            Ok(Some(ref mut row1)) => {
                let id = row1.column_int(0);
                println!("clientid sum is {}", id);
            }
            Err(oops) => panic!(oops),
            Ok(None) => panic!("where did our row go?"),
        }
    }

    pub fn select_teachers(&self, num: i64) -> SqliteResult<Vec<Teacher>> {
        let sql = format!("SELECT * FROM teacher where id = {}", num);
        //    select teacher
        let mut stmt: PreparedStatement = self.db_context.query(&sql)?;

        let snoc = |x, mut xs: Vec<_>| {
            xs.push(x);
            xs
        };

        let mut ppl;
        ppl = stmt
            .query_fold(&[], vec![], |row, ppl| {
                Ok(snoc(
                    Teacher {
                        id: row.get(0),
                        street: row.get(1),
                        city: row.get(2),
                        sendstatus: row.get(3),
                        datatype: row.get(4),
                        ops: row.get(5),
                        age: row.get(6),
                        clientid: row.get(7),
                        indexid: row.get(8),
                    },
                    ppl,
                ))
            })
            .unwrap();
        Ok(ppl)
    }
}
