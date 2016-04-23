#![crate_type = "dylib"]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;
extern crate winapi;
extern crate user32;
extern crate core;

use winapi::minwindef;
use std::process::{Command};
use def::{to_wide_chars,function_item_text,TCHAR,NppData,ShortcutKey,FuncItem};
//use std::io::{BufWriter, Write};

mod def;

#[macro_use]
extern crate lazy_static;

static mut SHORT_KEY1: ShortcutKey = ShortcutKey{
	_isCtrl: false,
	_isAlt: false,
	_isShift: false,
	_key: 116,
};

lazy_static! {
    static ref PROG_NAME: Vec<u16> = to_wide_chars("Rust plugin");
	static ref FUNC_ITEMS: Vec<FuncItem> = vec![
		FuncItem{ 
			_itemName: function_item_text("Run"), 
			_pFunc: testFn, 
			_cmdID: 0, 
			_init2Check: false,
			_pShKey: unsafe{ std::mem::transmute( &mut SHORT_KEY1 as *mut ShortcutKey ) }
		}
	];
}

//  item functions
#[allow(unused_variables)]
pub extern "C" fn testFn(){
	let cmdProc = Command::new("cmd").
						arg("/c").
						arg("start").
						arg("cmd").
						arg("/c").
						arg("pause").
						//stdin(Stdio::piped()).
						spawn().unwrap();
	//let mut cmdStdin = cmdProc.stdin.unwrap();
	//let mut writer = BufWriter::new(&mut cmdStdin);
}
// ______________


#[no_mangle]
pub extern "C" fn isUnicode() -> bool{ true }

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn setInfo( notpadPlusData:NppData ) {
	
}

#[no_mangle]
pub extern "C" fn getName() -> * const TCHAR{
	PROG_NAME.as_ptr()
}

#[no_mangle]
pub extern "C" fn getFuncsArray( nbF: *mut i32) -> *const FuncItem{
	unsafe { *nbF = 1 };
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
	minwindef::TRUE
}
