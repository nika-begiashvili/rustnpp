
use std;
use std::process::Command;
use def::{from_wide_ptr,TCHAR};
use plugindata::getNppHandle;
use winapi::winuser;
use winapi::minwindef;
use user32::SendMessageW;

const NPPM_GETFULLCURRENTPATH :minwindef::UINT = winuser::WM_USER + 3001;

pub fn launchCmdWindow( command: String ){
	Command::new("cmd").arg("/c").
			arg("start").arg("cmd").arg("/c").
			arg( format!("{} & pause ",command) ).
			spawn().unwrap();
}

#[allow(unused_mut)]
pub fn getCurrentNppFile() -> String{
	let mut fPath: Vec<TCHAR>  = vec![0;255];
	
	unsafe{
		SendMessageW( 
			getNppHandle()._nppHandle,
			NPPM_GETFULLCURRENTPATH, 0, 
			std::mem::transmute( fPath.as_ptr() )
		);
	}
	
	from_wide_ptr( fPath.as_ptr() )
}