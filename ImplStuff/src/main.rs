trait MyInterface {
	fn say_something(&self);
}

impl MyInterface for Hello{
	fn say_something(&self) {
		println!("Hello World");
	}
}


fn main(){
	let x = Hello;
	x.say_something();
}

