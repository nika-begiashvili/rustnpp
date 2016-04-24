extern crate libc;
extern crate winapi;
extern crate user32;

use winapi::minwindef;
use winapi::winuser;
use user32::SendMessageW;

use def::{function_item_text,ShortcutKey,FuncItem,NppData,TCHAR,from_wide_ptr};
use std;
use helpers;

pub static mut NPP_DATA:Option<NppData> = None;

const NPPM_GETFULLCURRENTPATH :minwindef::UINT = winuser::WM_USER + 3001;

static mut SHORT_KEY_F5: ShortcutKey = ShortcutKey{
	_isCtrl: false,
	_isAlt: false,
	_isShift: false,
	_key: 116,
};

pub fn FuncItem_Run() -> FuncItem{
	FuncItem{ 
		_itemName: function_item_text("Run"), 
		_pFunc: testFn, 
		_cmdID: 0, 
		_init2Check: false,
		_pShKey: unsafe{ std::mem::transmute( &mut SHORT_KEY_F5 as *mut ShortcutKey ) }
	}
}

pub fn getNppHandle() -> &'static mut NppData {
    unsafe { match NPP_DATA {
            Some(ref mut x) => &mut *x,
            None => panic!(),
    } }
}

//  exposed functions
#[allow(unused_variables)]
#[allow(unused_mut)]
pub extern "C" fn testFn(){
	let mut fPath: Vec<TCHAR>  = vec![0;255];
	
	unsafe{
		SendMessageW( 
			getNppHandle()._nppHandle,
			NPPM_GETFULLCURRENTPATH, 0, 
			std::mem::transmute( fPath.as_ptr() )
		);
	}
	
	let s: String = from_wide_ptr( fPath.as_ptr() );
	helpers::launchCmdWindow( format!("echo {}",s) );
}
