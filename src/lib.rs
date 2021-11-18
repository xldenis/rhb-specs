#![feature(box_syntax, box_patterns)]
#![feature(unsized_fn_params)]
#![allow(dead_code)]

pub mod cell;
pub mod vec;
pub mod prelude;
pub mod mutex;
pub mod r#fn;
pub mod thread;
pub mod r#box;

pub mod all_zero;
pub mod even_cell;
pub mod fib_cell;
pub mod iter_mut;
pub mod reversal;
pub mod concurrent;
