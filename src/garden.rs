use crate::garden::vegetables::Aspargus;

pub mod vegetables;


pub fn run_garden() {
    let aspargus = Aspargus {};
    println!("I'm growing {:?}", aspargus);
}
