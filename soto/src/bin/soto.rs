extern crate soto;

fn main() {
    match soto::build("./") {
        Ok(_) => {},
        Err(e) => println!("Soto error: {}", e),
    }
}
