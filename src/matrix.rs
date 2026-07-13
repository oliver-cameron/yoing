use factorial::Factorial;
use std::ops;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix<const HEIGHT: usize, const WIDTH: usize> {
    pub contents: [[f64; WIDTH]; HEIGHT],
}
impl<const HEIGHT: usize, const WIDTH: usize> Matrix<HEIGHT, WIDTH> {
    pub fn join_right<const INWIDTH: usize, const OUTWIDTH: usize>(
        self,
        input: Matrix<HEIGHT, INWIDTH>,
    ) -> Matrix<HEIGHT, OUTWIDTH> {
        const {
            assert!(
                OUTWIDTH == INWIDTH + WIDTH,
                "OUTWIDTH must equal INWIDTH + WIDTH"
            )
        }
        Matrix {
            contents: core::array::from_fn(|i| {
                let mut combined: [f64; OUTWIDTH] = [0.0; OUTWIDTH];
                combined[..WIDTH].copy_from_slice(&self.contents[i]);
                combined[WIDTH..].copy_from_slice(&input.contents[i]);
                combined
            }),
        }
    }
    pub fn join_down<const INHEIGHT: usize, const OUTHEIGHT: usize>(
        self,
        input: Matrix<INHEIGHT, WIDTH>,
    ) -> Matrix<OUTHEIGHT, WIDTH> {
        const {
            assert!(
                OUTHEIGHT == INHEIGHT + HEIGHT,
                "OUTHEIGHT must equal INHEIGHT + HEIGHT"
            )
        }
        Matrix {
            contents: {
                let mut combined: [[f64; WIDTH]; OUTHEIGHT] = [[0.0; WIDTH]; OUTHEIGHT];
                combined[..HEIGHT].copy_from_slice(&self.contents);
                combined[HEIGHT..].copy_from_slice(&input.contents);
                combined
            },
        }
    }
    pub fn zero() -> Self {
        Self {
            contents: { core::array::from_fn(|_| core::array::from_fn(|_| 0.0)) },
        }
    }
    pub fn infinity_norm(&self) -> f64 {
        self.contents
            .iter()
            .map(|row| {
                // 1. Compute the sum of absolute values for this specific row
                row.iter().map(|val| val.abs()).sum::<f64>()
            })
            // 2. Use .fold() to safely find the maximum float value without panicking
            .fold(0.0, |max_so_far, row_sum| {
                if row_sum > max_so_far {
                    row_sum
                } else {
                    max_so_far
                }
            })
    }
}

pub struct LUPair<const DIMENSION: usize> {
    l: Matrix<DIMENSION, DIMENSION>,
    u: Matrix<DIMENSION, DIMENSION>,
}
// Squaries
impl<const DIMENSION: usize> Matrix<DIMENSION, DIMENSION> {
    pub fn identity() -> Self {
        Self {
            contents: core::array::from_fn(|i| {
                core::array::from_fn(|j| if j == i { 1.0 } else { 0.0 })
            }),
        }
    }
    pub fn lu_decompose(&self) -> LUPair<DIMENSION> {
        let mut l = Self::identity();
        let mut u = self.clone();
        for u_column in 0..DIMENSION {
            for u_row in (u_column + 1)..DIMENSION {
                let div = u.contents[u_row][u_column] / u.contents[u_column][u_column];
                l.contents[u_row][u_column] = div;
                for index in (u_column + 1)..DIMENSION {
                    u.contents[u_row][index] -= div * u.contents[u_column][index];
                }
                u.contents[u_row][u_column] = 0.0;
            }
        }
        return LUPair { l: l, u: u };
    }
    pub fn exponent_pade<const POWER: usize>(&self) -> Self {
        // Scale
        // // Find largest value
        let infin_norm = self.infinity_norm();
        let coeff_cache: [f64; POWER] = core::array::from_fn(|i| pade_coeff_cache(POWER, i));
        let scaled_matrix: Self = Self {
            contents: self.contents.map(|i| i.map(|j| j / infin_norm)),
        };

        // Construct matrix power cache
        let power_cache: [Self; POWER] = [Self::zero(); POWER];
        power_cache[0] = Self::identity();
        power_cache[1] = scaled_matrix;
        for index in 2..POWER {
            power_cache[index] = power_cache[index - 1] * scaled_matrix;
        }
        let numerator = Self::zero();
        let denominator = Self::zero();
        for index in 0..POWER {
            numerator += power_cache[index] * coeff_cache[index];
            match index & 1 == 1 {
                true => denominator -= power_cache[index] * coeff_cache[index],
                false => denominator += power_cache[index] * coeff_cache[index],
            }
        }
        // Solve for the result
        let lu = denominator.lu_decompose();
        let mut y = Self::zero();

    }
}

fn pade_coeff_cache(m: usize, k: usize) -> f64 {
    let num = (((2 * m) - k).factorial() * m.factorial()) as f64;
    let dem = ((2 * m).factorial() * k.factorial() * (m - k).factorial()) as f64;
    num / dem
}

impl<const LHSHEIGHT: usize, const RHSWIDTH: usize, const INTERFACELENGTH: usize>
    ops::Mul<Matrix<INTERFACELENGTH, RHSWIDTH>> for Matrix<LHSHEIGHT, INTERFACELENGTH>
{
    type Output = Matrix<LHSHEIGHT, RHSWIDTH>;
    fn mul(self, rhs: Matrix<INTERFACELENGTH, RHSWIDTH>) -> Self::Output {
        Matrix {
            contents: self.contents.map(|lhs_array| {
                core::array::from_fn(|i| {
                    lhs_array
                        .iter()
                        .enumerate()
                        .fold(0.0, |acc, (index, lhs_value)| {
                            acc + lhs_value * rhs.contents[index][i]
                        })
                })
            }),
        }
    }
}
impl<const HEIGHT: usize, const WIDTH: usize> ops::Add<Matrix<HEIGHT, WIDTH>>
    for Matrix<HEIGHT, WIDTH>
{
    type Output = Matrix<HEIGHT, WIDTH>;
    fn add(self, rhs: Matrix<HEIGHT, WIDTH>) -> Self::Output {
        Matrix {
            contents: core::array::from_fn(|i| {
                core::array::from_fn(|j| self.contents[i][j] + rhs.contents[i][j])
            }),
        }
    }
}

impl<const HEIGHT: usize, const WIDTH: usize> ops::AddAssign<Matrix<HEIGHT, WIDTH>>
    for Matrix<HEIGHT, WIDTH>
{
    fn add_assign(&mut self, rhs: Matrix<HEIGHT, WIDTH>) {
        *self = Matrix {
            contents: core::array::from_fn(|i| {
                core::array::from_fn(|j| self.contents[i][j] + rhs.contents[i][j])
            }),
        };
    }
}


impl<const HEIGHT: usize, const WIDTH: usize> ops::SubAssign<Matrix<HEIGHT, WIDTH>>
    for Matrix<HEIGHT, WIDTH>
{
    fn sub_assign(&mut self, rhs: Matrix<HEIGHT, WIDTH>) {
        *self = Matrix {
            contents: core::array::from_fn(|i| {
                core::array::from_fn(|j| self.contents[i][j] - rhs.contents[i][j])
            }),
        };
    }
}

impl<const HEIGHT: usize, const WIDTH: usize> ops::Sub<Matrix<HEIGHT, WIDTH>>
    for Matrix<HEIGHT, WIDTH>
{
    type Output = Matrix<HEIGHT, WIDTH>;
    fn sub(self, rhs: Matrix<HEIGHT, WIDTH>) -> Self::Output {
        Matrix {
            contents: core::array::from_fn(|i| {
                core::array::from_fn(|j| self.contents[i][j] - rhs.contents[i][j])
            }),
        }
    }
}

impl<const HEIGHT: usize, const WIDTH: usize> ops::Mul<f64> for Matrix<HEIGHT, WIDTH> {
    type Output = Matrix<HEIGHT, WIDTH>;
    fn mul(self, rhs: f64) -> Self::Output {
        Matrix {
            contents: core::array::from_fn(|i| {
                core::array::from_fn(|j| self.contents[i][j] * rhs)
            }),
        }
    }
}

impl<const HEIGHT: usize, const WIDTH: usize> ops::MulAssign<f64> for Matrix<HEIGHT, WIDTH> {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Matrix {
            contents: core::array::from_fn(|i| {
                core::array::from_fn(|j| self.contents[i][j] * rhs)
            }),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply() {
        let matrix_a = Matrix::<3, 4> {
            contents: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 0.0, 1.0, 2.0],
            ],
        };

        // Matrix B: 4 rows, 2 columns
        let matrix_b = Matrix::<4, 2> {
            contents: [[1.0, 2.0], [3.0, 4.0], [5.0, 6.0], [7.0, 8.0]],
        };
        let expected = Matrix::<3, 2> {
            contents: [[50.0, 60.0], [114.0, 140.0], [28.0, 40.0]],
        };
        assert_eq!(matrix_a * matrix_b, expected)
    }
}
