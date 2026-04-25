mod solver_tests {
    use cube_solver::*;

    const SOLVED_CUBE_STICKERS:[u8; 54] = [0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5];
    
    #[test]
    fn make_solved_cubie(){
        
        let state = convert_to_cubie(SOLVED_CUBE_STICKERS);

        for i in 0..8 {
            assert_eq!(state.corners[i].position, i as u8);
            assert_eq!(state.corners[i].orientation, 0);
        }

        // edges in correct position and orientation
        for i in 0..12 {
            assert_eq!(state.edges[i].position, i as u8);
            assert_eq!(state.edges[i].flipped, false);
        }

    }

    #[test]
    fn single_turn_is_not_identity(){
        let mut state = convert_to_cubie(SOLVED_CUBE_STICKERS);

        state.apply_move(solver_move_t::U);

        let solved = convert_to_cubie(SOLVED_CUBE_STICKERS);
        assert!(state != solved);
    }

    #[test]
    fn four_turns_is_identity(){
        let mut state = convert_to_cubie(SOLVED_CUBE_STICKERS);

        state.apply_move(solver_move_t::U);
        state.apply_move(solver_move_t::U);
        state.apply_move(solver_move_t::U);
        state.apply_move(solver_move_t::U);

        let solved = convert_to_cubie(SOLVED_CUBE_STICKERS);
        assert!(state == solved);
    }

    #[test]
    fn move_and_inverse_cancel() {
        let mut state = convert_to_cubie(SOLVED_CUBE_STICKERS);

        state.apply_move(solver_move_t::R);
        state.apply_move(solver_move_t::Ri);

        let solved = convert_to_cubie(SOLVED_CUBE_STICKERS);
        assert!(state == solved);
    }


    #[test]
    fn scramble_and_inverse_returns_solved() {
        let mut state = convert_to_cubie(SOLVED_CUBE_STICKERS);

        let moves = [
            solver_move_t::R,
            solver_move_t::U,
            solver_move_t::F,
        ];

        for m in moves {
            state.apply_move(m);
        }

        for m in moves.iter().rev() {
            state.apply_move(inverse_move(*m));
        }

        let solved = convert_to_cubie(SOLVED_CUBE_STICKERS);
        assert_eq!(state, solved);
    }
}