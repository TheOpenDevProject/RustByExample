#[derive(Debug)] //Derive from the std fmt::Debug trait for our struct called structure
struct m_struct(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(m_struct);

fn main(){
	println!("Printing out structures {:?}",m_struct(3));
}
