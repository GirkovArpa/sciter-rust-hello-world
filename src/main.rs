// uncomment the line below to hide the console window
// #![windows_subsystem="windows"]

#[macro_use]
extern crate sciter;

use sciter::Value;
use std::env;

struct EventHandler;

impl EventHandler {
	fn hello(&self, input: String) -> Value {
		println!("Recieved the string: {}", input);
		Value::from(format!("Hello {}!", input))
	}
}

impl sciter::EventHandler for EventHandler {
	dispatch_script_call! (
		fn hello(String);
	);
}

fn main() {
	let handler = EventHandler {};
	let mut frame = sciter::Window::new();
	frame.event_handler(handler);
	let dir = env::current_dir().unwrap().as_path().display().to_string();
	// so that index.htm can load files such as 
	// <style src="index.css"></style>
	// <script type="text/tiscript" src="index.tis"></script>
	// we must give it the full absolute filename of the main file
	let filename = format!("{}\\{}", dir, "index.htm");
	println!("Full filename with path of index.htm: {}", filename);
	frame.load_file(&filename);
	frame.run_app();
}
