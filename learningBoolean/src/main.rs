fn main(){
	let result = isNumberGreaterThan(30,32);
	println!("{}",result);
}


fn isNumberGreaterThan(x:i32,y:i32) -> bool{
	if x > y{
		true
	}else{
		false
	}
}
