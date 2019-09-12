use sqlitedb::dbcontext::DbContext;
use std::sync::Arc;

use sqlitedb::sqlops::lose;
use sqlitedb::teacher_asset::TeacherAsset;
use sqlitedb::teacherdao;

pub fn test_case_thirteen() {
    let rc_context = Arc::new(DbContext::new("test.sqlite"));

    let teacher_asset = TeacherAsset::new(&rc_context, true);

    for i in 0..100 {
        match teacher_asset.select_teachers(i) {
            Ok(y) => {
                println!("SELECT * FROM teacher");
                println!("Ok: {:?}", y);
            }
            Err(oops) => lose(format!("oops!: {:?}\n", oops).as_ref()),
        }
    }
}
