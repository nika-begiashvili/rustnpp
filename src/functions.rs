
use helpers;
use std::path::Path;
use std::env;

fn cargoProject( s : String, cmd: String ) -> bool{
	let mut p = Some( Path::new(&s) );
	while p.unwrap().parent() != None{
		
		if p.unwrap().file_name().unwrap() == "src"{
			let par = p.unwrap().parent().unwrap();
		    if par.join("Cargo.toml").is_file() {
				let cwd = env::current_dir();
				env::set_current_dir( par ).unwrap();
				helpers::launchCmdWindow( format!("cargo {}",cmd) );
				env::set_current_dir( cwd.unwrap() ).unwrap();
				return true;
			}
		}else{
		    p = p.unwrap().parent();
		}
	}
	false
}

//  exposed functions

pub extern "C" fn runProgram(){
	let s = helpers::getCurrentNppFile();
	cargoProject( s, String::from("run") );
}

pub extern "C" fn buildProgram(){
	let s = helpers::getCurrentNppFile();
	cargoProject( s, String::from("build") );
}
