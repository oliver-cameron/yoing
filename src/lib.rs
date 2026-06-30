use std::ops::Index;
use fixed;
mod aux;
pub fn Bmain() {
    println!("Hello, world!");
}
pub struct Thing<const STATELENGTH: usize, const AUXLENGTH: usize>{
    state: aux::State<STATELENGTH>,
    aux_config: aux::AuxState<STATELENGTH, AUXLENGTH>,
    aux_state: aux::State<AUXLENGTH>,
    // forces?
}
