use rand::prelude::*;

pub fn run_benchmark(size: u32) -> Vec<i32> {
    let mut vec = random_vector(size);

    quick_sort(&mut vec);
    check_sorted(&vec);
    return vec;
}

pub fn quick_sort(slice: &mut [i32]) {
    if !slice.is_empty() {
        let partition_index = partition(slice);
        let len = slice.len();

        quick_sort(&mut slice[0..partition_index]);
        quick_sort(&mut slice[partition_index + 1..len]);
        check_sorted(slice);
    }
}

fn partition(slice: &mut [i32]) -> usize {
    let len = slice.len();
    let pivot = slice[len - 1];
    let mut i = 0;
    let mut j = 0;

    while j < len - 1 {
        if slice[j] <= pivot {
            slice.swap(i, j);
            i += 1;
        }
        j += 1;
    }

    slice.swap(i, len - 1);

    i
}

pub fn check_sorted(slice: &[i32]) -> bool {
    for i in 1..slice.len() {
        if !(slice[i - 1] <= slice[i]) {
            return false;
        }
    }
    return true;
}

pub fn random_vector(size: u32) -> Vec<i32> {
    let mut slice: Vec<i32> = Vec::new();
    for _ in 0..size {
        slice.push(rand::random::<i32>());
    }
    slice
}

#[cfg(test)]
mod tests {
    // use std::{collections::HashSet, error::Error};
    // use std::io;

    use super::*;

    #[test]
    fn test_init_arr() -> Result<(), &'static str> {
        let mut slice: Vec<i32> = Vec::new();
        slice = vec![1, 2, 3];

        assert_eq!(check_sorted(&slice), true);

        Ok(())
    }

    #[test]
    fn test_push_q() -> Result<(), &'static str> {
        let mut slice: Vec<i32> = Vec::new();
        slice = vec![1, 2, 3];
        quick_sort(&mut slice);

        assert_eq!(check_sorted(&slice), true);
        Ok(())
    }

    #[test]
    fn test_sort() -> Result<(), &'static str> {
        let mut slice: Vec<i32> = Vec::new();
        slice = vec![3, 2, 3, -1212];
        quick_sort(&mut slice);

        assert_eq!(check_sorted(&slice), true);
        Ok(())
    }

    #[test]
    fn test_sort_100() -> Result<(), &'static str> {
        let mut vec = run_benchmark(100);
        println!("{:?}", vec);
        Ok(())
    }

    #[test]
    fn test_sort_1000000() -> Result<(), &'static str> {
        run_benchmark(1000000);
        Ok(())
    }
}
