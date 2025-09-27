#![allow(clippy::needless_return)]
#![allow(unused_parens)] 

pub fn sort(mut input: Vec<i32>) -> Vec<i32>  {
    for i in 0..input.len() {
        for j in 1..input.len()-i{
            let current_number = *input.get(j).unwrap();
            let previous_number = *input.get(j-1).unwrap(); 
            if( current_number < previous_number ) {
                input.swap(j, j-1);
            }
        }
    }
    return input;
}