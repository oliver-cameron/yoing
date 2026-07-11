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
pub struct Point {pub x: f64, pub y: f64}
#[derive(Clone, Copy)]
pub enum Weight {Zero, Some {parr: Lin, perp: Lin}}
pub struct State<const LENGTH: usize> {pub points: [Point; LENGTH]}

#[derive(Clone, Copy)]
pub struct IOPoint {pub xx: f64, pub xy: f64, pub yx: f64, pub yy: f64}
#[derive(Clone, Copy)]
pub struct IOState<const LENGTH: usize> {pub matrix: [[IOPoint; LENGTH]; LENGTH]}

#[derive(Copy, Clone)]
pub struct AuxRow<const INLENGTH: usize> {pub weights: [Weight; INLENGTH]}
pub struct AuxState<const INLENGTH: usize, const OUTLENGTH: usize> {pub transform: [AuxRow<INLENGTH>; OUTLENGTH]}

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
    pub fn backward_aux<const INLENGTH: usize>(&self, aux_state: &AuxState<INLENGTH, LENGTH>) -> State<INLENGTH> {
        let mut out_points = [Point { x: 0.0, y: 0.0 }; INLENGTH];
        for (out_idx, out_row) in aux_state.transform.iter().enumerate() {
            let current_p = self.points[out_idx];
            for (in_idx, in_weight) in out_row.weights.iter().enumerate() {
                match in_weight {
                    Weight::Zero => {}
                    Weight::Some { parr, perp } => {
                        let parr_f64 = parr.to_num::<f64>();
                        let perp_f64 = perp.to_num::<f64>();
                        out_points[in_idx].x += current_p.x * parr_f64 + current_p.y * perp_f64;
                        out_points[in_idx].y += current_p.y * parr_f64 - current_p.x * perp_f64;
                    }
                }
            }
        }

        State { points: out_points }
    }
}

impl <const LENGTH: usize> IOState<LENGTH> {
    pub fn multiply_column (&self, input_state: &State<LENGTH>) -> State<LENGTH>{
        State{ points: self.matrix.map(|out| out.iter().enumerate().fold(
                Point{x:0.,y:0.,},
                |acc, (in_index, in_point)| {
                    Point{x: acc.x + input_state.points[in_index].x * in_point.xx + input_state.points[in_index].y * in_point.yx,
                          y: acc.y + input_state.points[in_index].x * in_point.xy + input_state.points[in_index].y * in_point.yy}
                }))}
    }
        pub fn backward_aux<const INLENGTH: usize>(&self, aux_state: &AuxState<INLENGTH, LENGTH>) -> IOState<INLENGTH> {
        // 1. Right multiplication first: Middle = M * A
        // Size: [LENGTH x LENGTH] * [LENGTH x INLENGTH] -> [LENGTH x INLENGTH]
        let mut middle_matrix = [[IOPoint { xx: 0.0, xy: 0.0, yx: 0.0, yy: 0.0 }; INLENGTH]; LENGTH];
        
        for r in 0..LENGTH {
            for c in 0..INLENGTH {
                let mut sum = IOPoint { xx: 0.0, xy: 0.0, yx: 0.0, yy: 0.0 };
                for k in 0..LENGTH {
                    let m_point = self.matrix[r][k];
                    match &aux_state.transform[k].weights[c] {
                        Weight::Zero => {}
                        Weight::Some { parr, perp } => {
                            let parr_f64 = parr.to_num::<f64>();
                            let perp_f64 = perp.to_num::<f64>();
                            
                            // M * A (Standard block multiplication)
                            sum.xx += m_point.xx * parr_f64 - m_point.xy * perp_f64;
                            sum.xy += m_point.xx * perp_f64 + m_point.xy * parr_f64;
                            sum.yx += m_point.yx * parr_f64 - m_point.yy * perp_f64;
                            sum.yy += m_point.yx * perp_f64 + m_point.yy * parr_f64;
                        }
                    }
                }
                middle_matrix[r][c] = sum;
            }
        }

        // 2. Left multiplication by Transpose: Out = A^T * Middle
        // Size: [INLENGTH x LENGTH] * [LENGTH x INLENGTH] -> [INLENGTH x INLENGTH]
        let mut out_matrix = [[IOPoint { xx: 0.0, xy: 0.0, yx: 0.0, yy: 0.0 }; INLENGTH]; INLENGTH];
        
        for r in 0..INLENGTH {
            for c in 0..INLENGTH {
                let mut sum = IOPoint { xx: 0.0, xy: 0.0, yx: 0.0, yy: 0.0 };
                for k in 0..LENGTH {
                    // Notice the outer transpose: reading row 'k', col 'r' from aux matrix
                    match &aux_state.transform[k].weights[r] {
                        Weight::Zero => {}
                        Weight::Some { parr, perp } => {
                            let parr_f64 = parr.to_num::<f64>();
                            let perp_f64 = perp.to_num::<f64>();
                            let mid = middle_matrix[k][c];
                            
                            // A^T * Middle (Inner 2x2 block matrix is ALSO transposed here)
                            // [ parr  perp] * [xx xy]
                            // [-perp  parr]   [yx yy]
                            sum.xx += parr_f64 * mid.xx + perp_f64 * mid.yx;
                            sum.xy += parr_f64 * mid.xy + perp_f64 * mid.yy;
                            sum.yx += -perp_f64 * mid.xx + parr_f64 * mid.yx;
                            sum.yy += -perp_f64 * mid.xy + parr_f64 * mid.yy;
                        }
                    }
                }
                out_matrix[r][c] = sum;
            }
        }

        IOState { matrix: out_matrix }
    }
}
