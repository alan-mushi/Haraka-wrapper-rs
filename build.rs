use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
	let out_dir = env::var("OUT_DIR").unwrap();
	Command::new("gcc").args(&["-O0", "-ffunction-sections", "-fdata-sections", "-g", "-m64", "-fPIC", "-I", "src", "-c", "-msse", "-maes", "-msse4", "src/c/haraka.c", "-o"])
		.arg(&format!("{}/haraka.o", out_dir))
		.status().unwrap();

	Command::new("ar").args(&["crus", "libharaka.a", "haraka.o"])
		.current_dir(&Path::new(&out_dir))
		.status().unwrap();

	println!("cargo:rustc-link-search=native={}", out_dir);
	println!("cargo:rustc-link-lib=static=haraka");
	println!("cargo:root={}", out_dir);
}
