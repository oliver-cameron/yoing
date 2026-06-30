use std::ops::Index;
use fixed::{FixedI8, traits::Fixed, types::extra::U4};
pub struct SafeIndex<const Max: usize>(pub usize);

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
#[derive(Clone, Copy)]
pub struct Point {x: f64, y: f64}
#[derive(Clone, Copy)]
pub enum Weight {Zero, Some {parr: Lin, perp: Lin}}
pub struct State<const LENGTH: usize> {points: [Point; LENGTH]}
#[derive(Copy, Clone)]
pub struct AuxRow<const INLENGTH: usize> {weights: [Weight; INLENGTH]}
pub struct AuxState<const INLENGTH: usize, const OUTLENGTH: usize> {transform: [AuxRow<INLENGTH>; OUTLENGTH]}

impl <const INLENGTH: usize> AuxRow<INLENGTH>{
    pub fn new(contents: [Weight; INLENGTH]){
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

impl <const LENGTH: usize> State<LENGTH>{
    pub fn forward_aux <const OUTLENGTH: usize>(&self, aux_state: &AuxState<LENGTH, OUTLENGTH>) -> State<OUTLENGTH>{
        State{ points: aux_state.transform.map(|out| out.weights.iter().enumerate().fold(
                Point{x:0.,y:0.,},
                |acc, (in_index, in_point)| {
                    match in_point{
                        Weight::Zero => acc,
                        Weight::Some{parr, perp} => {
                            let parr_f64 = parr.to_num::<f64>();
                            let perp_f64 = perp.to_num::<f64>();
                            Point{x: acc.x + self.points[in_index].x * parr_f64 - self.points[in_index].y * perp_f64, y: acc.y + self.points[in_index].y * parr_f64 + self.points[in_index].x * perp_f64}}
                    }
                }))}
    }
}

