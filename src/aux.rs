use std::ops::Index;
use fixed::{FixedI8, traits::Fixed, types::extra::U4};
pub struct SafeIndex<const max: usize>(pub usize);

// impl<T, const N: usize, const max: usize> Index<SafeIndex<max>> for [T; N]{
//     type Output = T;
//  fn index (&self, index: SafeIndex<max>) -> &Self::Output {
//         const{
//             assert!(max <= N, "Supposedly safe index out of bounds");
//         }
//         unsafe {self.get_unchecked(index.0)}
//     }
// }
pub type Lin = FixedI8<U4>;
pub struct Point {x: f64, y: f64}
#[derive(Clone)]
pub enum Weight {Zero, Some {parr: Lin, perp: Lin}}
pub type State<const Length: usize> = [Point; Length];
pub struct AuxRow<const InLength: usize> {weights: [Weight; InLength]}
pub struct AuxState<const InLength: usize, const OutLength: usize> {transform: [AuxRow<InLength>; OutLength]}

impl <const InLength: usize> AuxRow<InLength>{
    pub fn new(contents: [Weight; InLength]){
        // [todo]
        // make it throw compile time error if the stuff doesnt sum up
        // let sum_par = contents.clone().into_iter().fold((Lin::ZERO, Lin::ZERO),
        //     |acc, x|
        //     {match x {
        //         Weight::Zero => acc,
        //         Weight::Some{parr: xpar, perp: xper} => (acc.0.strict_add(xpar), acc.1.strict_add(xper)),
        //     }}
        //         );
        // assert_eq!(sum_par.0, 1);
        // assert_eq!(sum_par.1, Lin::ZERO);
        Self {weights: contents};
    }
}



