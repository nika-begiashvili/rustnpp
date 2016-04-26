
use helpers;
use std::path::Path;
use std::env;

fn cargoProject( currentFile : &String, cmd: String ) -> bool{
	let mut p = Some( Path::new(&currentFile) );
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

fn rustFile( currentFile :&String, run :bool ) -> bool{
	let p = Path::new(&currentFile);
	let fname = p.file_name().unwrap();
	let fstem = p.file_stem().unwrap();
	let par = p.parent().unwrap();
	let cwd = env::current_dir();
	
	let mut cmd :String = String::new();
	if run{
		cmd = format!(" & {}", fstem.to_str().unwrap() );
	}
	env::set_current_dir( par ).unwrap();
	helpers::launchCmdWindow( format!("rustc {} {}", fname.to_str().unwrap() , cmd) );
	env::set_current_dir( cwd.unwrap() ).unwrap();
	
	return true;
}



//  exposed functions

pub extern "C" fn runProgram(){
	let s = helpers::getCurrentNppFile();
	if !cargoProject( &s, String::from("run") ){
		rustFile(&s,true);
	}
}

pub extern "C" fn buildProgram(){
	let s = helpers::getCurrentNppFile();
	if !cargoProject( &s, String::from("build") ){
		rustFile(&s,false);
	}
}


