mod automate;
mod real_float;
use rand::random;
use std::cmp::Reverse;
use std::collections::binary_heap::BinaryHeap;
use typed_floats::NonNaNFinite;
type RevRealFloat = Reverse<NonNaNFinite<f64>>;

fn main() {
    let array: [RevRealFloat; 10] = core::array::from_fn(|_| {
        Reverse(NonNaNFinite::<f64>::new(random::<f64>() * 100.0f64).unwrap())
    });
    let string_with_sep = "some nice\n\rtext";
    let string_without_sep = string_with_sep.replace('\r', "");
    for line in string_without_sep.lines() {}

    let mut heap = BinaryHeap::from(array);
    for _ in 0..10 {
        println!("{} ", heap.pop().unwrap().0);
    }
}
