#![allow(non_snake_case)]
// #![feature(adt_const_params)]
mod binary_search;
mod bubble_sort;
mod crystal_ball;
mod linked_lst;
use crate::linked_lst::linkedlst::*;

mod vec_queue;
use crate::vec_queue::vecqueue::*;

mod stack_queue;
use crate::stack_queue::stack::*;

mod quicksort;
use crate::quicksort::quicksort::*;

#[allow(dead_code)]
fn main() {
    const SIZE: usize = 10;

    let _arr = crystal_ball::crystalball::init_arr::<SIZE>();
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

    // let condition = |x: i32| x >= 30;

    // // error because condition will be false for all
    // let result = crystal_ball::crystalball::check_crystal_ball::<SIZE>(&arr, condition);

    // println!("{:?}", result);

    ///////// LINKED LIST QUEUE START //////
    // let mut Q: Queue<NodeValue> = Queue::new();

    // let val: Option<&Box<Node<NodeValue>>> = Q.peek();
    // println!("val at init: \n{:?}\n", val);

    // Q.enqueue(NodeValue::I32(1));
    // let val = Q.peek();
    // println!("val after enQ: \n{:?}\n", val);

    // println!("Val of Q: \n{:?}\n", Q);

    // Q.enqueue(NodeValue::I32(2));
    // let val = Q.peek();
    // println!("val after enQ: \n{:?}\n", val);

    // println!("Val of Q: \n{:?}\n", Q);

    // let val = Q.dequeue();
    // println!("Val of deQ: \n{:?}\n", val);

    // let val = Q.peek();
    // println!("Val after deQ: \n{:?}\n", val);

    // println!("Val of Q: \n{:?}\n", Q);

    ///////// LINKED LIST QUEUE END //////
    // let vec_queue: VQueue<i32> = VQueue::new();
    // const SIZE: usize = 10;
    let condition = |x: i32| x >= 8;
    println!("{}", condition(1));

    let mut vec = run_benchmark(1000000);
    println!("{:?}", vec);
}
