// use std::error::Error;
#[allow(dead_code)]
pub fn init_arr<const SIZE: usize>() -> [i32; SIZE] {
    let mut array: [i32; SIZE] = [0; SIZE]; // Initialize with default values (0)

    for i in 0..SIZE {
        array[i] = i as i32 * 2; // index * 2. [0,2,4,6....]
    }

    array
}

#[allow(dead_code)]
pub fn process_array<const SIZE: usize>(array: &mut [i32]) {
    for i in 0..SIZE {
        for j in 0..SIZE-i-1 {
            if array[j] > array[j+1] {
                array.swap(j, j+1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use std::{collections::HashSet, error::Error};
    // use std::io;

    use super::*;

    #[test]
    fn test_init_arr() -> Result<(), &'static str> {
        const SIZE: usize = 10;
        let arr = init_arr::<SIZE>();
        assert_eq!(SIZE, arr.len());
        Ok(())
    }

    #[test]
    fn test_process_arr() -> Result<(), String> {
        const SIZE: usize = 3;
        let mut arr = [3,2,1];
        let expected_arr = [1,2,3];
        let result = process_array::<SIZE>(&mut arr);
        assert_eq!(arr, expected_arr);
        return Ok(result);
    }

    #[test]
    fn test_process_arr_2() -> Result<(), String> {
        const SIZE: usize = 6;
        let mut arr = [3,2,89,211,0,1];
        let expected_arr = [0,1,2,3,89,211];
        let result = process_array::<SIZE>(&mut arr);
        assert_eq!(arr, expected_arr);
        return Ok(result);
    }
}
