//! This module contains tests for various complex chains of `if` clauses
//! in different contexts. The tests in the sub modules should be very similar
//! to each other, and try to cover all paths the parser in the `comp` macro
//! takes while parsing if-chains, to make sure all generated code is sane
//! and correct.
//!
//! If you add a test to one of the sub modules, you probably want to add an
//! equivalent one to the other sub modules.

mod test_initial_if_in_final_for;
mod test_initial_if_in_non_final_for;
mod test_initial_if_let_in_final_for;
mod test_initial_if_let_in_non_final_for;
