pub struct Cube {
    pub transformations: i64,

    array: [[[u8; 3]; 3]; 6],
}

impl Cube {
    pub fn solved() -> Self {
        Self {
            transformations: 0,
            array: [
                [[0; 3]; 3],
                [[1; 3]; 3],
                [[2; 3]; 3],
                [[3; 3]; 3],
                [[4; 3]; 3],
                [[5; 3]; 3],
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved_cube_is_solved_and_untouched() {
        let solved_cube = Cube::solved();
        for face in 0..6 {
            for i in 0..3 {
                for j in 0..3 {
                    assert_eq!(solved_cube.array[face][i][j], face as u8);
                }
            }
        }
        assert_eq!(solved_cube.transformations, 0);
    }
}