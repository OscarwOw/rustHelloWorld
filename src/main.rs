mod bublesort;
mod quicksort;
fn main() -> () {
    // generate 300 numbers in 0..=1000
    let mut vec: Vec<i32> = Vec::with_capacity(300);
    let mut state: u64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64;
    for _ in 0..300 {
        // LCG step
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let val = ((state >> 33) % 1001) as i32; // 0..=1000
        vec.push(val);
    }

    println!("notSorted: {:?}", vec);
    // let sorted = bublesort::sort(vec);
    
    println!("sorting...\n\n\n");
    let sorted = quicksort::sort(vec);
    println!("complete...\n\n\n");
    println!("sorted: {:?}", sorted);
}