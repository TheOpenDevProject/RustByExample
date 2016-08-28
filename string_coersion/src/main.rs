fn main() {
      let my_string = "Wassup Epic Mic Condenser".to_string();
      takes_slice(&my_string);
}

fn takes_slice(slice: &str) {
    println!("Our Converted Slice: {}",slice);
}
