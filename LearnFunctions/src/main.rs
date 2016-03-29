fn main(){
	let mut x = 17;
	println!("X = {}",x);
	x = my_func();
	println!("X = {}",x);
	x = calc(100,100);
	println!("X = {}",x);
	x = |x| =>{
		64
	}
	println!("X = {}",x);
}

fn my_func() -> i32{
	let x = 9;
	x
}

fn calc(x:i32,y:i32) -> i32{
	let result = x * y;
	result

}
