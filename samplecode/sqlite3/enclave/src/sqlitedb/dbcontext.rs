use sqlite3::{access, DatabaseConnection, PreparedStatement, SqliteResult, StatementUpdate};

use std::cell::RefCell;
use std::default::Default;
use std::rc::Rc;
use std::string::String;

pub struct DbContext {
    pub conn: Rc<RefCell<DatabaseConnection>>,
}

impl DbContext {
    pub fn new(db_file: &str) -> DbContext {
        let mut connection = DatabaseConnection::in_memory().unwrap();
        if !db_file.is_empty() {
            connection = DatabaseConnection::new(access::ByFilename {
                filename: db_file,
                flags: Default::default(),
            })
            .unwrap();
        }

        DbContext {
            conn: Rc::new(RefCell::new(connection)),
        }
    }
}
impl DbContext {
    //用于创建表操作
    pub fn exec(&self, sql: &str) {
        let result = self.conn.borrow_mut().exec(sql);
        match result {
            Ok(_) => {}
            Err(e) => {
                println!("failed to execute sql, error message: {}", e);
                println!("sql: {}", sql);
            }
        }
    }

    //执行插入、更新、删除等操作
    pub fn execute(&self, sql: &str) -> bool {
        let mut result = self.conn.borrow_mut().prepare(sql);
        match result {
            Ok(ref mut stmt) => {
                let changes = stmt.update(&[]);
                match changes {
                    Ok(_num) => true,
                    Err(e) => {
                        println!("insert execute update failed {}", e);
                        println!("sql: {}", sql);
                        false
                    }
                }
            }
            Err(e) => {
                println!("insert execute prepare failed {}", e);
                println!("sql: {}", sql);
                false
            }
        }
    }

    pub fn query(&self, sql: &str) -> SqliteResult<PreparedStatement> {
        let result = self.conn.borrow_mut().prepare(sql);
        match result {
            Ok(stmt) => Ok(stmt),
            Err(e) => {
                println!("failed to prepare query sql, error message: {}", e);
                println!("sql: {}", sql);
                Err(e)
            }
        }
    }
}
