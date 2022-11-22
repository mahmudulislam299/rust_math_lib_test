//! THis is is test library
//!
//! you add , sub , mul , div two number using this create
//! this is very useful library 
//!
//! # Quick Start
//!
//! To get you started quickly, the easiest and highest-level way to get
//! a random value is to use [`random()`]; alternatively you can use
//! [`thread_rng()`]. The [`Rng`] trait provides a useful API on all RNGs, while
//! the [`distributions`] and [`seq`] modules provide further
//! functionality on top of RNGs.
//!
//! # The Book
//!
//! For the user guide and further documentation, please read
//! [The Rust Rand Book](https://rust-random.github.io/book)



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
