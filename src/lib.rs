use std::ops::Index;
use fixed;
mod aux;
pub fn Bmain() {
    println!("Hello, world!");
}
//
// pub struct ThingKind<const StateLength: usize, const auxilLength: usize, const innerAuxilLength: [usize; auxilLength]>{
//     auxil: [[(SafeIndex<StateLength>,f32); innerAuxilLength[index]]; auxilLength]
// }
//
// pub struct Thing<const StateLength: usize>{
//     state: [f64; StateLength],
//     kind: &ThingKind<StateLength>,
// }
//
