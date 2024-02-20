extern crate libc;
extern crate winapi;

use def::{function_item_text,ShortcutKey,FuncItem,NppData};
use std;
use functions;

pub static mut NPP_DATA:Option<NppData> = None;

static mut SHORT_KEY_F10: ShortcutKey = ShortcutKey{
	_isCtrl: false,
	_isAlt: false,
	_isShift: false,
	_key: 121,
};

static mut SHORT_KEY_CTRL_F10: ShortcutKey = ShortcutKey{
	_isCtrl: true,
	_isAlt: false,
	_isShift: false,
	_key: 121,
};

pub fn FuncItem_Run() -> FuncItem{
	FuncItem{ 
		_itemName: function_item_text("Run"), 
		_pFunc: functions::runProgram, 
		_cmdID: 0, 
		_init2Check: false,
		_pShKey: unsafe{ std::mem::transmute( &mut SHORT_KEY_F10 as *mut ShortcutKey ) }
	}
}

pub fn FuncItem_Build() -> FuncItem{
	FuncItem{ 
		_itemName: function_item_text("Build"), 
		_pFunc: functions::buildProgram, 
		_cmdID: 1, 
		_init2Check: false,
		_pShKey: unsafe{ std::mem::transmute( &mut SHORT_KEY_CTRL_F10 as *mut ShortcutKey ) }
	}
}

pub fn getNppHandle() -> &'static mut NppData {
    unsafe { match NPP_DATA {
            Some(ref mut x) => &mut *x,
            None => panic!(),
    } }
}


