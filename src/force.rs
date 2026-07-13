use crate::aux::State;
pub trait Force<const AUXLENGTH: usize> {
    fn apply(&self, state: State<AUXLENGTH>);
}
