// use std::error::Error;

pub fn init_arr<const SIZE: usize>() -> [i32; SIZE] {
    let mut array: [i32; SIZE] = [0; SIZE]; // Initialize with default values (0)

    for i in 0..SIZE {
        array[i] = i as i32 * 2; // index * 2. [0,2,4,6....]
    }

    array
}

// return the Ok(index) of value if in array, otherwise Err("String")
pub fn process_array(array: &[i32], needle: i32) -> Result<usize, String> {
    let mut low: usize = 0;
    let mut high: usize = array.len() - 1;
    let mut mid: usize;

    loop {
        mid = low + (high - low) / 2; // this should be floor automatically because usize datatype

        let value = array[mid];

        if value == needle {
            return Ok(mid);
        }
        // break condition if low exeeds high
        else if low >= high {
            return Err(format!("needle not found in array"));
        }
        // if the mid value is greater than needle, exclude upper part of array
        else if value > needle {
            high = mid;
        // if the mid value is smaller or equal to needle, exclude upper part of array
        } else {
            low = mid + 1;
        }
    }

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
    fn test_init_arr() -> Result<(), &'static str> {
        const SIZE: usize = 10;
        let arr = init_arr::<SIZE>();
        assert_eq!(SIZE, arr.len());
        Ok(())
        // return init_arr(5);
    }

    #[test]
    fn test_process_arr() -> Result<(), String> {
        const SIZE: usize = 10;
        let arr = init_arr::<SIZE>();

        let result = process_array(&arr,0);
        assert_eq!(result, Ok(0));
        let result = process_array(&arr,2);
        assert_eq!(result, Ok(1));
        let result = process_array(&arr,4);
        assert_eq!(result, Ok(2));
        let result = process_array(&arr,6);
        assert_eq!(result, Ok(3));
        let result = process_array(&arr,8);
        assert_eq!(result, Ok(4));
        let result = process_array(&arr,10);
        assert_eq!(result, Ok(5));
        let result = process_array(&arr,18);
        assert_eq!(result, Ok(9));
        return Ok(());
    }

    #[test]
    fn test_process_arr_error() -> Result<(), String> {
        const SIZE: usize = 10;
        let arr = init_arr::<SIZE>();

        let result = process_array(&arr,1);
        assert_eq!(result, Err(format!("needle not found in array")));
        return Ok(());
    }
}
