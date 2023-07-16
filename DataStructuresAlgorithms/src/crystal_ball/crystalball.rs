// use std::error::Error;

pub fn init_arr<const SIZE: usize>() -> [i32; SIZE] {
    let mut array: [i32; SIZE] = [0; SIZE]; // Initialize with default values (0)

    for i in 0..SIZE {
        array[i] = i as i32 * 2; // index * 2. [0,2,4,6....]
    }

    array
}

pub fn process_array(array: &[i32], condition: fn(i32) -> bool) -> Result<(), String> {
    for &element in array {
        // if condition is false, then return Error
        if !condition(element) {
            return Err(format!("Condition not met {}", element));
        }
    }
    Ok(())
}

// checks array for condition
// assumes that condition will be true for first part of array and false for last part.
// goal is to find last index, in which condition is true, rest will be false
// returns Ok(index), or Err("String")
pub fn check_crystal_ball<const SIZE: usize>(
    array: &[i32],
    condition: fn(i32) -> bool,
) -> Result<usize, String> {
    // define jump amount sqrt(array.len())
    let square_root: usize = (SIZE as f64).sqrt().floor() as usize;

    let mut saved_i = 0;
    // SIZE index excluded, up to and not included
    for i in (square_root..SIZE).step_by(square_root) {
        saved_i = i;
        if condition(array[i]) {
            break;
        }
    }
    // saved_i not included because already checked
    for i in (saved_i-square_root..saved_i).step_by(1) {
        if condition(array[i]) {
            return Ok(i);
        }
    }

    return Err(format!("Both balls broke without finding true condition"));
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
        const SIZE: usize = 10;
        let arr = init_arr::<SIZE>();

        let condition = |x: i32| x % 2 == 0;

        let result = process_array(&arr, condition);
        assert_eq!(result, Ok(()));
        return result;
    }

    #[test]
    fn test_process_arr_error() -> Result<(), String> {
        const SIZE: usize = 10;
        let arr = init_arr::<SIZE>();

        let condition = |x: i32| x <= 8;

        let result = process_array(&arr, condition);
        assert_eq!(result, Err(format!("Condition not met 10")));
        return Ok(());
    }

    #[test]
    fn test_check_crystal_ball() -> Result<(), String> {
        const SIZE: usize = 10;
        let arr = init_arr::<SIZE>();

        let condition = |x: i32| x >= 8;
        // expecting index 4, because condition will be true up until and including this index
        let result = check_crystal_ball::<SIZE>(&arr, condition);
        assert_eq!(result, Ok(4));
        return Ok(());
    }

    #[test]
    fn test_check_crystal_ball_error() -> Result<(), String> {
        const SIZE: usize = 10;
        let arr = init_arr::<SIZE>();

        let condition = |x: i32| x <= -1;

        // error because condition will be false for all
        let result = check_crystal_ball::<SIZE>(&arr, condition);
        assert_eq!(
            result,
            Err(format!("Both balls broke without finding true condition"))
        );
        return Ok(());
    }
}
