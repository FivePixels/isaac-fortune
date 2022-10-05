use rand::Rng;
use std::include_bytes;

fn main() {
    let bytes = include_bytes!("resources/fortunes");
    let fortunes_as_string = String::from_utf8_lossy(bytes);
    let fortunes: Vec<&str> = fortunes_as_string.split("\n").collect();
    let mut rng = rand::thread_rng();
    let rn = rng.gen_range(0..fortunes.len() - 1);
    println!("{}", &fortunes[rn]);
}
