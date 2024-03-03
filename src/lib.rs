#![crate_type = "cdylib"]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;
extern crate winapi;
extern crate core;

use winapi::shared::minwindef;
use def::{to_wide_chars,TCHAR,NppData,FuncItem};

mod def;
mod plugindata;
mod helpers;
mod functions;

#[macro_use]
extern crate lazy_static;

lazy_static! {
	static ref PROG_NAME: Vec<u16> = to_wide_chars("Rust plugin");
	static ref FUNC_ITEMS: Vec<FuncItem> = vec![
		plugindata::FuncItem_Run(),
		plugindata::FuncItem_Build(),
	];
}

#[no_mangle]
pub extern "C" fn isUnicode() -> bool{ true }

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn setInfo( notpadPlusData : NppData ) {
	unsafe{ plugindata::NPP_DATA = Some(notpadPlusData); }
}

#[no_mangle]
pub extern "C" fn getName() -> * const TCHAR{
	PROG_NAME.as_ptr()
}

#[no_mangle]
pub extern "C" fn getFuncsArray( nbF: *mut i32) -> *const FuncItem{
	let items = FUNC_ITEMS.len();
	let itemsize =  items as i32;
	unsafe { *nbF = itemsize };
	FUNC_ITEMS.as_ptr()
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn beNotified(notifyCode: *mut libc::c_void){
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn messageProc( 
	Message: minwindef::UINT, 
	wParam: minwindef::WPARAM, 
	lParam: minwindef::LPARAM) -> minwindef::LRESULT{
/*
	if (Message == WM_MOVE)
	{
		::MessageBox(NULL, "move", "", MB_OK);
	}
*/
	minwindef::TRUE as minwindef::LRESULT
}
