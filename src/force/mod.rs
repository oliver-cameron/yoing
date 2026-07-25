use crate::aux::IOState;
use crate::aux::State;
use crate::kinematics::Kinematics;

pub mod spring;
pub enum ForceOutput<const LENGTH: usize> {
    Simple {
        residuals: State<LENGTH>,
    },
    Stable {
        restore: IOState<LENGTH>,
        residuals: State<LENGTH>,
    },
    Damped {
        damping: IOState<LENGTH>,
        restore: IOState<LENGTH>,
        residuals: State<LENGTH>,
    },
}
pub trait Force<const AUXLENGTH: usize> {
    fn apply(&self, state: &Kinematics<AUXLENGTH>) -> ForceOutput<AUXLENGTH>;
}
