#![allow(non_snake_case)]
// #![feature(adt_const_params)]
mod crystal_ball;
mod binary_search;
mod bubble_sort;
fn main() {
    const SIZE: usize = 10;

    let arr = crystal_ball::crystalball::init_arr::<SIZE>();
    // This iterates by reference:
    // for item in arr.iter().enumerate() {
    //     let (i, x): (usize, &i32) = item;
    //     println!("arr[{i}] = {x}");
    // }

    // // This iterates by value:
    // for item in arr.into_iter().enumerate() {
    //     let (i, x): (usize, i32) = item;
    //     println!("arr[{i}] = {x}");
    // }

    // Example condition: Even numbers are not allowed
    // let condition = |x: i32| x % 2 == 0;

    // let result = crystal_ball::crystalball::process_array(&arr, condition);
   
    // let low = 2;
    // let high = 5;
    // println!("{:?}", low + (high -low ) / 2 ) ;
    // match result {
    //     Ok(()) => println!("Condition met for all elements"),
    //     Err(err) => println!("Error: {}", err),
    // }


    let condition = |x: i32| x >= 30;

    // error because condition will be false for all
    let result = crystal_ball::crystalball::check_crystal_ball::<SIZE>(&arr, condition);
    println!("{:?}", result);

}
