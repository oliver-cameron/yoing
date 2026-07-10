use std::ops::Index;
use std::ops::Range;
use fixed;
mod aux;
pub fn Bmain() {
    println!("Hello, world!");
}

pub enum colourStyle{
    Colour([u8;3])
}

pub struct Thing<const STATELENGTH: usize, const AUXLENGTH: usize, const SHAPECOUNT: usize>{
    state_pos: aux::State<STATELENGTH>,
    state_vel: aux::State<STATELENGTH>,
    aux_config: aux::AuxState<STATELENGTH, AUXLENGTH>,
    aux_pos: aux::State<AUXLENGTH>,
    aux_vel: aux::State<AUXLENGTH>,
    matrix_aux_restore: aux::IOState<AUXLENGTH>,
    matrix_aux_dampen: aux::IOState<AUXLENGTH>,
    matrix_bias_column: aux::State<AUXLENGTH>,
    shape_config: [(Range<usize>,colourStyle);SHAPECOUNT],
    // forces?
}

impl <const STATELENGTH: usize, const AUXLENGTH: usize, const SHAPECOUNT: usize> Thing<STATELENGTH, AUXLENGTH, SHAPECOUNT> {
    pub fn update_aux(mut self){
       self.aux_pos = self.state_pos.forward_aux(&self.aux_config); 
       self.aux_vel = self.state_vel.forward_aux(&self.aux_config); 
    }
    pub fn pre_force_calc(self){
        self.update_aux();
    }
    pub fn post_force_calc(self){
        // Compute residuals of restore and dampen blocks. These are influence matrcies, and should
        // really just have the difference of pos or vel as input. This probably makes no sense so
        // come talk to me.
        let restore_residual: aux::State<AUXLENGTH> = self.matrix_aux_restore.multiply_column(&self.aux_pos);
        let dampen_residual : aux::State<AUXLENGTH> = self.matrix_aux_dampen .multiply_column(&self.aux_vel);
        let bias_column_state: aux::State<STATELENGTH> = aux::State{points: std::array::from_fn(|i|
                {
                    let orig = self.matrix_bias_column.points[i];
                    aux::Point{x: orig.x - restore_residual.points[i].x - dampen_residual.points[i].x,
                       y: orig.y - restore_residual.points[i].y - dampen_residual.points[i].x}
                }
        )
            }.backward_aux(&self.aux_config);

        let matrix_state_restore = self.matrix_aux_restore.backward_aux(&self.aux_config);
        let matrix_state_dampen  = self.matrix_aux_dampen .backward_aux(&self.aux_config);

        
    }
}
