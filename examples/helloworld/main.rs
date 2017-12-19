/*
 * Simple 'Hello World' example calling Red from Rust.
 */
	 
extern crate red;

use std::ffi::CString;
use red::*;

fn main() {
	unsafe {
	let text = r#"
			Red []			
			view/options [title "Red and Rust"
				 text "Hello World from Rust!"
			][]
		"#;
		redOpen();
		redDo(CString::new(text).unwrap().as_ptr());
		redClose();
	}
}