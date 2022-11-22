pub mod mul_div;
pub mod struct_test;



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
