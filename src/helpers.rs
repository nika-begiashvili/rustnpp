
use std::process::Command;

pub fn launchCmdWindow( command: String ){
	Command::new("cmd").arg("/c").
			arg("start").arg("cmd").arg("/k").
			arg( format!("{} & pause ",command) ).
			spawn().unwrap();
}