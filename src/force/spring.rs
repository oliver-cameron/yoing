use crate::{
    aux::{IOPoint, IOState, Point, State},
    force::{self, ForceOutput},
};
pub struct Spring<const AUXLENGTH: usize> {
    p1: usize,
    p2: usize,
    stiffness: f64,
    target_length: f64,
}

impl<const AUXLENGTH: usize> force::Force<AUXLENGTH> for Spring<AUXLENGTH> {
    fn apply(
        &self,
        state: &crate::kinematics::Kinematics<AUXLENGTH>,
    ) -> force::ForceOutput<AUXLENGTH> {
        let point1 = state.pos.points[self.p1];
        let point2 = state.pos.points[self.p2];
        let vector = point1 - point2;
        let dist = (point1 - point2).len();
        let invdist = 1.0 / dist;
        let inv3dist = invdist * invdist * invdist;

        let mut out_mat = IOState::<AUXLENGTH>::zero();
        let mut out_col = State::<AUXLENGTH>::zero();

        let distdir = 1.0 - self.target_length * invdist;
        let xdir = 2.0 * vector.x * distdir;
        let ydir = 2.0 * vector.y * distdir;

        let dist2dir = self.target_length * inv3dist;
        let xydir = 2.0 * vector.x * vector.y * dist2dir;
        let xxdir = 2.0 * (distdir + vector.x * vector.x * dist2dir);
        let yydir = 2.0 * (distdir + vector.y * vector.y * dist2dir);

        out_col.points[self.p2] = Point { x: xdir, y: ydir } * self.stiffness;
        out_col.points[self.p1] = Point { x: -xdir, y: -ydir } * self.stiffness;

        let posIOPoint = IOPoint {
            xx: xxdir,
            xy: xydir,
            yx: xydir,
            yy: yydir,
        } * self.stiffness;
        let negIOPoint = IOPoint {
            xx: -xxdir,
            xy: -xydir,
            yx: -xydir,
            yy: -yydir,
        } * self.stiffness;

        out_mat.matrix[self.p1][self.p1] = posIOPoint;
        out_mat.matrix[self.p2][self.p2] = posIOPoint;
        out_mat.matrix[self.p1][self.p2] = negIOPoint;
        out_mat.matrix[self.p2][self.p1] = negIOPoint;

        force::ForceOutput::Stable {
            restore: out_mat,
            residuals: out_col,
        }
    }
}
