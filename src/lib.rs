use fixed;
use std::ops::Index;
use std::ops::Range;

use crate::aux::Kinematics;
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
    weights: [f64; STATELENGTH],
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
    pub fn post_force_calc(self, dt: f64) {
        let cur_state = aux::Kinematics {
            pos: self.state_pos,
            vel: self.state_vel,
            c: 1.0,
        };
        let force_matrix = aux::IOKinematics::<STATELENGTH> {
            dampen: self.matrix_aux_dampen.backward_aux(&self.aux_config),
            restore: self.matrix_aux_restore.backward_aux(&self.aux_config),
            residuals: self.matrix_bias_column.backward_aux(&self.aux_config),
            weights: self.weights,
        }
        .residual(&cur_state);
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
