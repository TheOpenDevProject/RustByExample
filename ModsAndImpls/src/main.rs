//This statement imports the module. Rustc will say hello i see a module named sample_mod -> sample_mod -> mod.rs
mod sample_module;
use sample_module::SampleImpl;

fn main(){
	let myObject = SampleImpl::new("Matt".to_string());
	myObject.hello_world();
}
