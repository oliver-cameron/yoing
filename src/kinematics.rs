use crate::aux;
use crate::matrix;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Kinematics<const LENGTH: usize> {
    pub pos: aux::State<LENGTH>,
    pub vel: aux::State<LENGTH>,
    pub c: f64,
}
impl<const LENGTH: usize> Kinematics<LENGTH> {
    pub fn zero() -> Self {
        Self {
            pos: aux::State::<LENGTH>::zero(),
            vel: aux::State::<LENGTH>::zero(),
            c: 0.0,
        }
    }
}
// Rust won't lemme join matrcies so I do this instead.
pub struct IOKinematics<const LENGTH: usize> {
    // There is no pos pos, this is Zero
    // vel pos is 1/weights, component wise. Might? make this a State later if x weight is different
    // from y.
    pub weights: [f64; LENGTH],
    // There is no c pos
    // pos vel is restore.
    pub restore: aux::IOState<LENGTH>,
    // vel vel is dampen
    pub dampen: aux::IOState<LENGTH>,
    // c vel is residuals
    pub residuals: aux::State<LENGTH>,
}

impl<const LENGTH: usize> ops::Mul<&Kinematics<LENGTH>> for &IOKinematics<LENGTH> {
    type Output = Kinematics<LENGTH>;
    fn mul(self, rhs: &Kinematics<LENGTH>) -> Self::Output {
        let pos: aux::State<LENGTH> = aux::State {
            points: std::array::from_fn(|i| {
                let point = rhs.vel.points[i];
                let weight = self.weights[i];
                aux::Point {
                    x: point.x / weight,
                    y: point.y / weight,
                }
            }),
        };
        let restoreVel = self.restore * rhs.pos;
        let dampenVel = self.dampen * rhs.vel;
        let residualVel = aux::State {
            points: self.residuals.points.map(|x| aux::Point {
                x: x.x * rhs.c,
                y: x.y * rhs.c,
            }),
        };
        let vel: aux::State<LENGTH> = aux::State {
            points: std::array::from_fn(|i| {
                restoreVel.points[i] + dampenVel.points[i] + residualVel.points[i]
            }),
        };
        Kinematics {
            pos: pos,
            vel: vel,
            c: 0.0,
        }
    }
}

impl<const LENGTH: usize> ops::Mul<Kinematics<LENGTH>> for Kinematics<LENGTH> {
    type Output = f64;
    // dot product
    fn mul(self, rhs: Kinematics<LENGTH>) -> Self::Output {
        let mut output: f64 = 0.0;
        // pos
        output = self
            .pos
            .points
            .iter()
            .enumerate()
            .fold(output, |acc, (i, point)| {
                acc + point.x * rhs.pos.points[i].x + point.y * rhs.pos.points[i].y
            });
        // vel
        output = self
            .vel
            .points
            .iter()
            .enumerate()
            .fold(output, |acc, (i, point)| {
                acc + point.x * rhs.vel.points[i].x + point.y * rhs.vel.points[i].y
            });
        // c
        output + self.c * rhs.c
    }
}

impl<const LENGTH: usize> ops::Add<Kinematics<LENGTH>> for Kinematics<LENGTH> {
    type Output = Self;
    fn add(self, rhs: Kinematics<LENGTH>) -> Self::Output {
        Self {
            pos: aux::State {
                points: std::array::from_fn(|f| self.pos.points[f] + rhs.pos.points[f]),
            },
            vel: aux::State {
                points: std::array::from_fn(|f| self.vel.points[f] + rhs.vel.points[f]),
            },
            c: self.c + rhs.c,
        }
    }
}

impl<const LENGTH: usize> ops::Mul<f64> for Kinematics<LENGTH> {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            pos: aux::State {
                points: std::array::from_fn(|f| self.pos.points[f] * rhs),
            },
            vel: aux::State {
                points: std::array::from_fn(|f| self.vel.points[f] * rhs),
            },
            c: self.c * rhs,
        }
    }
}

impl<const LENGTH: usize> IOKinematics<LENGTH> {
    pub fn krylov<const DEGREE: usize>(self, _state: &Kinematics<LENGTH>) -> Kinematics<LENGTH> {
        // Matrix will be assembled like this:
        // 0, I (divided by weights), 0
        // restore, dampen, residuals
        // 0, 0, 0
        let mut subspace = [Kinematics::<LENGTH>::zero(); DEGREE];
        let mut q = matrix::Matrix::<DEGREE, DEGREE>::zero();
        let orig_scale = f64::sqrt(*_state * *_state);
        if orig_scale < 1e-12 {
            return Kinematics::<LENGTH>::zero();
        }
        subspace[0] = *_state * (1.0 / orig_scale);
        // https://phys.au.dk/~fedorov/Numeric/11/krylov.pdf
        for k in 1..=DEGREE {
            let mut qk = &self * &subspace[k - 1];
            for i in 0..k {
                let dot = subspace[i] * qk;
                q.contents[i][k - 1] = dot;
                qk = qk + subspace[i] * -dot;
            }
            let scale = f64::sqrt(qk * qk);
            if k < DEGREE {
                q.contents[k][k - 1] = scale;
                if scale < 1e-12 {
                    break;
                }
                subspace[k] = qk * (1.0 / scale);
            }
        }

        const PADE_DEGREE: usize = 13;
        let exp_q = q.exponent_pade::<PADE_DEGREE>();
        let mut y = [0.0; DEGREE];
        for i in 0..DEGREE {
            y[i] = exp_q.contents[i][0] * orig_scale;
        }
        let mut final_state = Kinematics::<LENGTH>::zero();
        for i in 0..DEGREE {
            final_state = final_state + subspace[i] * y[i];
        }
        final_state
    }
    // pub fn backward_aux<const INLENGTH: usize>(
    //     self,
    //     _auxconfig: &AuxState<INLENGTH, LENGTH>,
    // ) -> IOKinematics<INLENGTH> {
    //     IOKinematics::<INLENGTH> {
    //         restore: self.restore.backward_aux(_auxconfig),
    //         dampen: self.dampen.backward_aux(_auxconfig),
    //         residuals:
    //     }
    // }
    pub fn residual(self, _state: &Kinematics<LENGTH>) -> Self {
        Self {
            dampen: self.dampen,
            restore: self.restore,
            weights: self.weights,
            residuals: { self.residuals - self.dampen * _state.pos - self.restore * _state.vel },
        }
    }
}

impl<const LENGTH: usize> ops::Mul<f64> for IOKinematics<LENGTH> {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            weights: std::array::from_fn(|x| self.weights[x] / rhs),
            dampen: self.dampen * rhs,
            residuals: self.residuals * rhs,
            restore: self.restore * rhs,
        }
    }
}
