use super::cube::{Coordinate, CubeState};
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
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
                let new_coordinate = (face, i, j);
                let &(old_face, old_i, old_j) =
                    self.moves.get(&new_coordinate).unwrap_or(&new_coordinate);
                new_face[i][j] = state[old_face][old_i][old_j];
            }
        }
        new_face
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

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

    #[test]
    fn all_transformations_are_correct() {
        let transformations = Transformation::all_transformations();
        let state = checkable_state();
        for transformation in transformations {
            let new_state = transformation.apply(&state);
            assert_transfomation_changes(&transformation, &state, new_state);
        }
    }

    fn checkable_state() -> CubeState {
        let mut state = [[[0u8; 3]; 3]; 6];
        let mut val = 0u8;
        for face in 0..6 {
            for i in 0..3 {
                for j in 0..3 {
                    state[face][i][j] = val;
                    val += 1;
                }
            }
        }
        state
    }

    fn assert_transfomation_changes(
        transformation: &Transformation,
        old_state: &CubeState,
        new_state: CubeState,
    ) {
        let mut new_state_values = HashSet::new();
        for face in 0..6 {
            for i in 0..3 {
                for j in 0..3 {
                    let new_coordinate = (face, i, j);
                    let &(old_face, old_i, old_j) = transformation
                        .moves
                        .get(&new_coordinate)
                        .unwrap_or(&new_coordinate);
                    assert_eq!(old_state[old_face][old_i][old_j], new_state[face][i][j]);
                    new_state_values.insert(new_state[face][i][j]);
                }
            }
        }
        assert_eq!(new_state_values.len(), 54);
    }
}
