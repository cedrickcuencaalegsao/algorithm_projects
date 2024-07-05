fn pick_up_postcard() {
  println!("Picked up a postcard.");
}

fn pick_up_pen() {
  println!("Picked up the pen.");
}

fn write_on_postcard(message: &str) {
  println!("Wrote on the postcard: {}", message);
}

fn write_address(address: &str) {
  println!("Wrote the address: {}", address);
}

fn go_to_post_office() {
  println!("Went to the post office.");
}

fn post_card() {
  println!("Posted the card.");
}

fn main() {
  pick_up_postcard();
  pick_up_pen();
  write_on_postcard("Greetings from Rust!");
  write_address("123 Main St, Rustville");
  go_to_post_office();
  post_card();
}
