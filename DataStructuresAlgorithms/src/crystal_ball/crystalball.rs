// use std::error::Error;

pub fn init_arr<const SIZE: usize>() -> [i32; SIZE] {
    let mut array: [i32; SIZE] = [0; SIZE]; // Initialize with default values (0)

    for i in 0..SIZE {
        array[i] = i as i32 * 2; // index * 2. [0,2,4,6....]
    }

    array
}


pub fn process_array(array: &[i32], condition: fn(i32) -> bool) -> Result<(),String> {
    for &element in array {
        // if condition is false, then return Error
        if !condition(element) {
            return Err(format!("Condition not met {}", element));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    // use std::{collections::HashSet, error::Error};
    use std::io;

    use super::*;

    #[test]
    fn test_b() -> io::Result<()> {
        Ok(())
    }

    #[test]
    fn test_init_arr() -> Result<(),&'static str> {
        const SIZE: usize = 10;
        let arr = init_arr::<SIZE>();
        assert_eq!(SIZE, arr.len());
        Ok(())
        // return init_arr(5);
    }


    #[test]
    fn test_process_arr() -> Result<(),String> {
        const SIZE: usize = 10;
        let arr = init_arr::<SIZE>();
    
        let condition = |x: i32| x % 2 == 0;
        
        let result = process_array(&arr, condition);
        assert_eq!(result, Ok(()));
        return result
    }

    
    #[test]
    fn test_process_arr_error() -> Result<(),String> {
        const SIZE: usize = 10;
        let arr = init_arr::<SIZE>();
    
        let condition = |x: i32| x % 2 != 0;
        
        let result = process_array(&arr, condition);
        assert_eq!(result, Err(format!("Condition not met 0")));
        return Ok(())
    }
}