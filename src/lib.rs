use fixed;
use std::ops::Index;
use std::ops::Range;
mod aux;
mod force;
mod matrix;
pub fn Bmain() {
    println!("Hello, world!");
}

pub enum colour_style {
    Colour([u8; 3]),
}

pub struct Thing<'a, const STATELENGTH: usize, const AUXLENGTH: usize, const SHAPECOUNT: usize> {
    state_pos: aux::State<STATELENGTH>,
    state_vel: aux::State<STATELENGTH>,
    aux_config: aux::AuxState<STATELENGTH, AUXLENGTH>,
    aux_pos: aux::State<AUXLENGTH>,
    aux_vel: aux::State<AUXLENGTH>,
    matrix_aux_restore: aux::IOState<AUXLENGTH>,
    matrix_aux_dampen: aux::IOState<AUXLENGTH>,
    matrix_bias_column: aux::State<AUXLENGTH>,
    shape_config: [(Range<usize>, colour_style); SHAPECOUNT],
    force_config: &'a Vec<Box<dyn force::Force<AUXLENGTH>>>,
    // forces?
    // yes forces
}

impl<'a, const STATELENGTH: usize, const AUXLENGTH: usize, const SHAPECOUNT: usize>
    Thing<'a, STATELENGTH, AUXLENGTH, SHAPECOUNT>
{
    pub fn update_aux(mut self) {
        self.aux_pos = self.state_pos.forward_aux(&self.aux_config);
        self.aux_vel = self.state_vel.forward_aux(&self.aux_config);
    }
    pub fn pre_force_calc(self) {
        self.update_aux();
    }
    pub fn post_force_calc(self) {
        todo!();
        // Compute residuals of restore and dampen blocks. These are influence matrcies, and should
        // really just have the difference of pos or vel as input. This probably makes no sense so
        // come talk to me.
        // let restore_residual: aux::State<AUXLENGTH> =
        //     self.matrix_aux_restore.multiply_column(&self.aux_pos);
        // let dampen_residual: aux::State<AUXLENGTH> =
        //     self.matrix_aux_dampen.multiply_column(&self.aux_vel);
        // let bias_column_state: aux::State<STATELENGTH> = aux::State {
        //     points: std::array::from_fn(|i| {
        //         let orig = self.matrix_bias_column.points[i];
        //         aux::Point {
        //             x: orig.x - restore_residual.points[i].x - dampen_residual.points[i].x,
        //             y: orig.y - restore_residual.points[i].y - dampen_residual.points[i].x,
        //         }
        //     }),
        // }
        // .backward_aux(&self.aux_config);
        //
        // let matrix_state_restore = self.matrix_aux_restore.backward_aux(&self.aux_config);
        // let matrix_state_dampen = self.matrix_aux_dampen.backward_aux(&self.aux_config);
        // // Now collect everything into a matrix
        // const DUBSTAT: usize = STATELENGTH * 2; //shorthand for double state
        // let top = matrix::Matrix::<DUBSTAT, DUBSTAT>::zero()
        //     .join_right(matrix::Matrix::<DUBSTAT, DUBSTAT>::identity())
        //     .join_right(matrix::Matrix::<DUBSTAT, 1>::zero());
    }
}
impl<const SIZE: usize> aux::IOState<SIZE> {
    pub fn toMatrix<const OUTSIZE: usize>(self) -> matrix::Matrix<OUTSIZE, OUTSIZE> {
        const {
            assert!(
                OUTSIZE == SIZE << 1,
                "Converting to a matrix doubles dimensions"
            );
        }
        matrix::Matrix {
            contents: core::array::from_fn(|i| {
                core::array::from_fn(|j| {
                    let spot = self.matrix[i >> 1][j >> 1];
                    match (i & 1 == 1, j & 1 == 1) {
                        (false, false) => spot.xx,
                        (false, true) => spot.yx,
                        (true, false) => spot.xy,
                        (true, true) => spot.yy,
                    }
                })
            }),
        }
    }
}
