#![feature(box_syntax, box_patterns)]
#![feature(unsized_fn_params)]
#![cfg_attr(not(feature = "contracts"), feature(stmt_expr_attributes, proc_macro_hygiene))]
#![allow(dead_code)]

pub mod prelude;

pub mod all_zero;
pub mod even_cell;
pub mod fib_cell;
pub mod inc_vec;
pub mod reversal;
pub mod concurrent;
pub mod knights_tour;
