//! Blunka BPMS
//!
//!

use axum::Router;
pub fn main() {
    byz_core::run::main(Router::new());
}
