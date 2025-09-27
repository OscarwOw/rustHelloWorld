mod bublesort;
fn main() -> () {
    let vec = vec![64, 34, 25, 12, 22, 11, 90];
    println!("notSorted: {:?}", vec);
    let sorted = bublesort::sort(vec);
    println!("sorted: {:?}", sorted);
}