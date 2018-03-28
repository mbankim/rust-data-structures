extern crate rust_data_structures;
use rust_data_structures::trees::treap;

fn main () {
    let mut tre = treap::Treap::new();
    for i in 1..100 {
        tre.insert(i);
    }
    let depths = tre.get_depths();
    println!("depths: {:?}", depths);
    println!("leaf nodes: {}", depths.len());
}
