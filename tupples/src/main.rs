fn main() {
  let my_tuple = ('k',45);
  println!("Our tupple contains the letter: {:?}",my_tuple.0);
  println!("Our tupple contains the number: {:?}",my_tuple.1);

  let (a,b) = my_tuple;
  println!("Fancy: {:?}",a);
  println!("Pancy:{:?}",b);
}
