use std::ops::Mul;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix<const HEIGHT: usize, const WIDTH: usize> {pub contents: [[f64; WIDTH]; HEIGHT]}
impl <const HEIGHT: usize, const WIDTH: usize> Matrix <HEIGHT, WIDTH> {
    pub fn join_right<const INWIDTH: usize, const OUTWIDTH: usize>(self, input: Matrix<HEIGHT, INWIDTH>) -> Matrix <HEIGHT, OUTWIDTH> {
        const{
            assert!(OUTWIDTH == INWIDTH + WIDTH, "OUTWIDTH must equal INWIDTH + WIDTH")
        }
        Matrix {contents: core::array::from_fn(|i|{
            let mut combined: [f64; OUTWIDTH] = [0.0; OUTWIDTH];
            combined[..WIDTH].copy_from_slice(&self .contents[i]);
            combined[WIDTH..].copy_from_slice(&input.contents[i]);
            combined
        }
        )
        }
    }
    pub fn join_down<const INHEIGHT: usize, const OUTHEIGHT: usize>(self, input: Matrix<INHEIGHT, WIDTH>) -> Matrix <OUTHEIGHT, WIDTH> {
        const {
            assert!(OUTHEIGHT == INHEIGHT + HEIGHT, "OUTHEIGHT must equal INHEIGHT + HEIGHT")
        }
        Matrix {contents: {
            let mut combined: [[f64; WIDTH]; OUTHEIGHT] = [[0.0; WIDTH]; OUTHEIGHT];
            combined[..HEIGHT].copy_from_slice(&self .contents);
            combined[HEIGHT..].copy_from_slice(&input.contents);
            combined
        }
        }
    }
    pub fn zero() -> Self {
        Self {contents: {
            core::array::from_fn(|_|
                core::array::from_fn(|_|
                    0.0
                )
            )
        }
        }
    }
}

// Squaries
impl <const DIMENSION: usize> Matrix<DIMENSION, DIMENSION> {
    pub fn identity () -> Self {
        Self { contents: core::array::from_fn(|i|
            core::array::from_fn(|j| {
                if j == i {
                    1.0
                } else {
                    0.0
                }
            }
            )
        )
    }
}
}

impl <const LHSHEIGHT: usize, const RHSWIDTH: usize, const INTERFACELENGTH: usize> Mul<Matrix<INTERFACELENGTH, RHSWIDTH>> for Matrix<LHSHEIGHT, INTERFACELENGTH> {
    type Output = Matrix::<LHSHEIGHT, RHSWIDTH>;
    fn mul(self, rhs: Matrix<INTERFACELENGTH, RHSWIDTH>) -> Self::Output {
        Matrix{ contents: self.contents.map(|lhs_array|
            core::array::from_fn(|i| 
                lhs_array.iter().enumerate().fold(
                    0.0,
                    |acc, (index, lhs_value)|
                    acc + lhs_value * rhs.contents[index][i]
                )
            )
        )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply(){
        let matrix_a = Matrix::<3, 4> {
            contents: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 0.0, 1.0, 2.0],
            ],
        };

        // Matrix B: 4 rows, 2 columns
        let matrix_b = Matrix::<4, 2> {
            contents: [
                [1.0, 2.0],
                [3.0, 4.0],
                [5.0, 6.0],
                [7.0, 8.0],
            ],
        };
        let expected = Matrix::<3, 2> {
            contents: [
                [ 50.0,  60.0],
                [114.0, 140.0],
                [ 28.0,  40.0],
            ],
        };
        assert_eq!(matrix_a * matrix_b, expected)
    }
}
