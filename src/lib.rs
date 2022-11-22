pub mod mul_div;
pub mod struct_test;


/// Adds two numbers.
///
/// # Examples
///
/// ```
/// let result = add(2, 2);
/// assert_eq!(result, 4);
/// ```

pub fn add(left: usize, right: usize) -> usize 
{
    left + right
}

pub fn sub(left: usize, right: usize) -> usize
{
    left - right
}

pub fn take_retrun_string(input: String) -> String
{
    println!("{}", input);
    return input;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
