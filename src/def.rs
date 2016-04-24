
extern crate winapi;
extern crate libc;

use winapi::windef;
use winapi::minwindef;
use std;

pub type TCHAR = u16;

#[repr(C)]
#[derive(Clone)]
pub struct NppData{
	pub _nppHandle: windef::HWND,
	pub _scintillaMainHandle: windef::HWND,
	pub _scintillaSecondHandle: windef::HWND,
}

#[repr(C)]
pub struct ShortcutKey {
	pub _isCtrl: bool,
	pub _isAlt: bool,
	pub _isShift: bool,
	pub _key: minwindef::UCHAR,
}

#[repr(C)]
pub struct FuncItem {
	pub _itemName: [TCHAR; 64],
	pub _pFunc: extern "C" fn(),
	pub _cmdID: i32,
	pub _init2Check: bool,
	pub _pShKey: usize,
}

pub fn to_wide_chars(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>()
}

pub fn from_wide_ptr(ptr: *const u16) -> String {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    unsafe {
        assert!(!ptr.is_null());
        let len = (0..std::isize::MAX).position(|i| *ptr.offset(i) == 0).unwrap();
        let slice = std::slice::from_raw_parts(ptr, len);
        OsString::from_wide(slice).to_string_lossy().into_owned()
    }
}

pub fn function_item_text( s: &str ) -> [TCHAR;64]{
	let mut arr: [TCHAR;64] = [0;64];
	let vecStr = to_wide_chars( &s );
	for (i,ch) in vecStr.iter().enumerate() {
		arr[i] = *ch;
	}
	
	arr
}