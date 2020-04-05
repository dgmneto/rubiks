use super::cube::{Coordinate, CubeState};
use std::collections::HashMap;

#[derive(PartialEq,Debug)]
pub struct Transformation {
    moves: HashMap<Coordinate, Coordinate>,
}

impl Transformation {
    pub fn all_transformations() -> Vec<Transformation> {
        Vec::new()
    }

    pub(super) fn apply(&self, state: &CubeState) -> CubeState {
        [
            self.apply_fo_face(state, 0),
            self.apply_fo_face(state, 1),
            self.apply_fo_face(state, 2),
            self.apply_fo_face(state, 3),
            self.apply_fo_face(state, 4),
            self.apply_fo_face(state, 5),
        ]
    }

    fn apply_fo_face(&self, state: &CubeState, face: usize) -> [[u8; 3]; 3] {
        let mut new_face = [[0u8; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                let coordinate = (face, i, j);
                new_face[i][j] = if self.moves.contains_key(&coordinate) {
                    let (face, i, j) = self.moves[&coordinate];
                    state[face][i][j]
                } else {
                    state[face][i][j]
                }
            }
        }
        new_face
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_two_transformation_are_equal() {
        let transformations = Transformation::all_transformations();
        for i in 0..transformations.len() {
            for j in 0..i {
                assert_ne!(transformations[i], transformations[j]);
            }
        }
    }

    // Ignoring for now as we didn't implement the preprocessing yet
    #[test]
    #[ignore]
    fn exactly_12_transformations() {
        let transformations = Transformation::all_transformations();
        assert_eq!(transformations.len(), 12);
    }
}
