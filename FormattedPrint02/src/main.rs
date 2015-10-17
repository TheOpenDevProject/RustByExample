fn main(){
	//Standard string replace output
	println!("{} days of RustLang",365);
	
	//C# Like string outputting based on an ordinal position
	println!("{0} is a cat, {1} is a dawg","Josh","Mutt");

	//Rust has a cool feature of being able to parse named arguments zomg
	println!("{MyArgument} is better than {YourArg}",MyArgument="Baker",YourArg="Razer");
	
	//Special fmt
	println!("{} of {:b} people are weird",1,2);

}
