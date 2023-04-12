use std::ffi::{c_char, c_uint, c_ulonglong, CStr};
use std::os::raw::c_uchar;
use std::ptr::null;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::OnceCell;

pub struct ReqwestHandle {
    _runtime: Runtime,
    items: slab::Slab<Arc<Result>>,
}

pub struct Result {
    status: AtomicU8,
    data: OnceCell<Vec<u8>>,
}


/// 新建运行时
/// thread_count 表示并发线程数量
#[no_mangle]
pub extern "C" fn reqwest_create(thread_count: u32) -> *mut ReqwestHandle {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(thread_count as usize)
        .enable_all()
        .build()
        .expect("tokio runtime fail");

    Box::into_raw(Box::new(ReqwestHandle {
        _runtime: runtime,
        items: Default::default(),
    }))
}

/// 释放运行时
#[no_mangle]
pub unsafe extern "C" fn reqwest_release(handler: *mut ReqwestHandle) {
    let handler = Box::from_raw(handler);
    drop(handler)
}

/// 开始get url
#[no_mangle]
pub unsafe extern "C" fn reqwest_url(
    handler: &mut ReqwestHandle,
    url: *const c_char,
) -> c_ulonglong {
    let url = CStr::from_ptr(url).to_str().unwrap();

    let res = Arc::new(Result {
        status: AtomicU8::new(0),
        data: Default::default(),
    });
    let key = handler.items.insert(res.clone());

    handler._runtime.spawn(async move {
        match reqwest::get(url).await {
            Ok(response) => match response.bytes().await {
                Ok(data) => {
                    res.status.store(1, Ordering::Release);
                    res.data.set(data.to_vec()).unwrap();
                }
                Err(err) => {
                    res.status.store(2, Ordering::Release);
                    res.data.set(err.to_string().into()).unwrap();
                }
            },
            Err(err) => {
                res.status.store(2, Ordering::Release);
                res.data.set(err.to_string().into()).unwrap();
            }
        }
    });

    key as c_ulonglong
}

/// 检查是否完成 0=没有完成 1=成功 2=错误
#[no_mangle]
pub extern "C" fn reqwest_check(handler: &mut ReqwestHandle, key: c_ulonglong) -> c_uchar {
    if let Some(result) = handler.items.get(key as usize) {
        result.status.load(Ordering::Acquire)
    } else {
        0
    }
}

/// 获取url 数据 check=1 表示数据 check=2表示错误信息
#[no_mangle]
pub unsafe extern "C" fn reqwest_get_data(
    handler: &mut ReqwestHandle,
    key: c_ulonglong,
    len: &mut c_uint,
) -> *const c_uchar {
    if let Some(result) = handler.items.get(key as usize) {
        if let Some(data) = result.data.get() {
            *len = data.len() as c_uint;
            data.as_ptr()
        } else {
            null()
        }
    } else {
        null()
    }
}

/// 完成url
#[no_mangle]
pub extern "C" fn reqwest_finish_url(handler: &mut ReqwestHandle,key:c_ulonglong){
     handler.items.try_remove(key as usize);
}