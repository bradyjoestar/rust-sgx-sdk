use sqlitedb::dbcontext::DbContext;
use std::sync::Arc;

use sqlitedb::sqlops::lose;
use sqlitedb::teacher_asset::TeacherAsset;
use sqlitedb::teacherdao;

use sgx_types::*;

extern "C" {
    // OCALLS
    pub fn ocall_empty(
        ret_val: *mut sgx_status_t,
        inside_str: *const u8,
        inside_len: u32,
        p_result_str: *mut u8,
        maxlen: u32,
        p_result_len: *mut u32,
    ) -> sgx_status_t;
}

pub fn test_case_thirteen() {
    let rc_context = Arc::new(DbContext::new("test.sqlite"));

    let teacher_asset = TeacherAsset::new(&rc_context, true);

    let inputstr = "{\"id\":2010001,\"street\":\"Street1\",\"city\":\"City1\",\"age\":1,\"sendstatus\":\"not end\",\"clientid\":10000,\"datatype\":\"energy_teacher\",\"ops\":\"insert\",\"indexid\":1}";

    let mut rt: sgx_status_t = sgx_status_t::SGX_ERROR_UNEXPECTED;

    let mut result_len: u32 = 0;
    let p_result_len = &mut result_len as *mut u32;

    const RET_QUOTE_BUF_LEN: u32 = 2048;
    let mut return_json_buf: [u8; RET_QUOTE_BUF_LEN as usize] =
        [0; RET_QUOTE_BUF_LEN as usize];
    let p_result = return_json_buf.as_mut_ptr();
    let maxlen = RET_QUOTE_BUF_LEN;

    let result = unsafe {
        ocall_empty(
            &mut rt as *mut sgx_status_t,
            inputstr.as_ptr(),
            inputstr.len() as u32,
            p_result,
            maxlen,
            p_result_len,
        )
    };

    println!("result str length is:{}", result_len);

    if result != sgx_status_t::SGX_SUCCESS {
        panic!("not sgx success");
    }
}
