#![allow(clippy::needless_return)]
#![allow(unused_parens)] 

pub fn sort(mut input: Vec<i32>) -> Vec<i32>{

    let len = input.len();
    if(len == 1){
        return input;
    }
    if(len == 2){
        if(input[0]>input[1]){
            input.swap(0, 1);
        }
        return input;
    }
    
    let pivot_index: usize = median_of_three(&mut input, 0, len - 1);

    let pivot = input[pivot_index];

    input.swap(pivot_index, len-1);
    let mut left =0;
    let mut right = len-2;

    
    while( left < right ){
        if( input[left] <= pivot ){
            left = left + 1;
            continue;
        }
        if( input[right] > pivot){
            right = right - 1;
            continue;
        }
        input.swap(left, right);
    }
    input.swap(left, len-1);

    if(left>0){
        let tmp = sort(input[0..left].to_vec());
        input[0..left].copy_from_slice(&tmp);
    }
    if(right<len-1){
        let tmp = sort(input[right+1..len].to_vec());
        input[right+1..len].copy_from_slice(&tmp);
    }

    return input;
}

fn median_of_three(arr: &mut Vec<i32>, left: usize, right: usize) -> usize {
    if(right == 0){
        return 0;
    }
    let mid = (left + right) / 2;

    if arr[left] > arr[mid] {
        arr.swap(left, mid);
    }
    if arr[left] > arr[right] {
        arr.swap(left, right);
    }
    if arr[mid] > arr[right] {
        arr.swap(mid, right);
    }

    return mid;
}