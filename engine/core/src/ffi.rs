use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;
use crate::runner::AgentEngine;

/// 创建核心引擎句柄 (返回指针供外部语言或主程序操纵)
#[no_mangle]
pub extern "C" fn agent_engine_create() -> *mut AgentEngine {
    match AgentEngine::new() {
        Ok(engine) => Box::into_raw(Box::new(engine)),
        Err(_) => std::ptr::null_mut(),
    }
}

/// 执行安全文件 Patch (返回 0 成功, 负数为错误代号)
#[no_mangle]
pub extern "C" fn agent_engine_patch(
    handle: *mut AgentEngine,
    file_path: *const c_char,
    old_block: *const c_char,
    new_block: *const c_char,
) -> i32 {
    if handle.is_null() || file_path.is_null() || old_block.is_null() || new_block.is_null() {
        return -1;
    }

    let engine = unsafe { &mut *handle };
    let c_path = unsafe { CStr::from_ptr(file_path) }.to_string_lossy();
    let c_old = unsafe { CStr::from_ptr(old_block) }.to_string_lossy();
    let c_new = unsafe { CStr::from_ptr(new_block) }.to_string_lossy();

    match engine.safe_patch_file(Path::new(c_path.as_ref()), &c_old, &c_new) {
        Ok(_) => 0,
        Err(_) => -2,
    }
}

/// 执行 Rust 语法干跑自检 (返回 null 为通过，非 null 为错误信息字符串)
#[no_mangle]
pub extern "C" fn agent_engine_verify_syntax(
    handle: *mut AgentEngine,
    code: *const c_char,
) -> *mut c_char {
    if handle.is_null() || code.is_null() {
        return std::ptr::null_mut();
    }

    let engine = unsafe { &mut *handle };
    let code_str = unsafe { CStr::from_ptr(code) }.to_string_lossy();

    match engine.verifier.check_syntax(&code_str) {
        Ok(()) => std::ptr::null_mut(),
        Err(err_msg) => {
            if let Ok(c_str) = CString::new(err_msg) {
                c_str.into_raw()
            } else {
                std::ptr::null_mut()
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn agent_engine_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        };
    }
}

#[no_mangle]
pub extern "C" fn agent_engine_destroy(handle: *mut AgentEngine) {
    if !handle.is_null() {
        unsafe {
            let _ = Box::from_raw(handle);
        };
    }
}
