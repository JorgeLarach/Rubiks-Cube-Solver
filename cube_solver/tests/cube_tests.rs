//
// cube_tests.rs
//
//  Created on: Jan 23, 2026
//      Author: jorgelarach
//

#[cfg(test)]
mod coord_tests {
    use cube_solver::*;

    // -------------------------------------------------------
    // Helpers
    // -------------------------------------------------------

    fn solved() -> CubieState {
        CubieState::make_solved()
    }

    fn apply_moves(moves: &[solver_move_t]) -> CubieState {
        let mut cube = solved();
        for &m in moves { cube.apply_move(m); }
        cube
    }

    const ALL_MOVES: [solver_move_t; 18] = [
        solver_move_t::U,  solver_move_t::Ui, solver_move_t::U2,
        solver_move_t::D,  solver_move_t::Di, solver_move_t::D2,
        solver_move_t::L,  solver_move_t::Li, solver_move_t::L2,
        solver_move_t::R,  solver_move_t::Ri, solver_move_t::R2,
        solver_move_t::F,  solver_move_t::Fi, solver_move_t::F2,
        solver_move_t::B,  solver_move_t::Bi, solver_move_t::B2,
    ];

    // -------------------------------------------------------
    // CORNER ORIENTATION  (range 0..2187)
    // -------------------------------------------------------

    #[test]
    fn corner_orient_solved_is_zero() {
        assert_eq!(solved().corner_orient_coord(), 0);
    }

    #[test]
    fn corner_orient_in_range() {
        // Apply every move once and check range
        for &m in &ALL_MOVES {
            let coord = apply_moves(&[m]).corner_orient_coord();
            assert!(coord < 2187, "move {:?} gave coord {} >= 2187", m, coord);
        }
    }

    #[test]
    fn corner_orient_move_then_inverse_is_zero() {
        // Move followed by its inverse must return to solved coord
        let pairs = [
            (solver_move_t::U,  solver_move_t::Ui),
            (solver_move_t::D,  solver_move_t::Di),
            (solver_move_t::L,  solver_move_t::Li),
            (solver_move_t::R,  solver_move_t::Ri),
            (solver_move_t::F,  solver_move_t::Fi),
            (solver_move_t::B,  solver_move_t::Bi),
        ];
        for (m, mi) in pairs {
            let coord = apply_moves(&[m, mi]).corner_orient_coord();
            assert_eq!(coord, 0, "{:?} then {:?} should return to 0", m, mi);
        }
    }

    #[test]
    fn corner_orient_four_moves_is_zero() {
        // Any face move applied 4 times returns to solved
        for &m in &ALL_MOVES {
            let coord = apply_moves(&[m, m, m, m]).corner_orient_coord();
            assert_eq!(coord, 0, "{:?} x4 should return to 0", m);
        }
    }

    #[test]
    fn corner_orient_actually_changes() {
        // U/D don't twist corners, but R/L/F/B do — check at least one changes
        let changing_moves = [
            solver_move_t::R, solver_move_t::L,
            solver_move_t::F, solver_move_t::B,
        ];
        for &m in &changing_moves {
            let coord = apply_moves(&[m]).corner_orient_coord();
            assert_ne!(coord, 0, "{:?} should change corner orient coord", m);
        }
    }

    // -------------------------------------------------------
    // EDGE ORIENTATION  (range 0..2048)
    // -------------------------------------------------------

    #[test]
    fn edge_orient_solved_is_zero() {
        assert_eq!(solved().edge_orient_coord(), 0);
    }

    #[test]
    fn edge_orient_in_range() {
        for &m in &ALL_MOVES {
            let coord = apply_moves(&[m]).edge_orient_coord();
            assert!(coord < 2048, "move {:?} gave coord {} >= 2048", m, coord);
        }
    }

    #[test]
    fn edge_orient_move_then_inverse_is_zero() {
        let pairs = [
            (solver_move_t::U,  solver_move_t::Ui),
            (solver_move_t::D,  solver_move_t::Di),
            (solver_move_t::L,  solver_move_t::Li),
            (solver_move_t::R,  solver_move_t::Ri),
            (solver_move_t::F,  solver_move_t::Fi),
            (solver_move_t::B,  solver_move_t::Bi),
        ];
        for (m, mi) in pairs {
            let coord = apply_moves(&[m, mi]).edge_orient_coord();
            assert_eq!(coord, 0, "{:?} then {:?} should return to 0", m, mi);
        }
    }

    #[test]
    fn edge_orient_four_moves_is_zero() {
        for &m in &ALL_MOVES {
            let coord = apply_moves(&[m, m, m, m]).edge_orient_coord();
            assert_eq!(coord, 0, "{:?} x4 should return to 0", m);
        }
    }

    #[test]
    fn edge_orient_actually_changes() {
        // R/L flip edges, U/D/F/B do not
        let flipping_moves = [
            solver_move_t::R, solver_move_t::L,
        ];
        for &m in &flipping_moves {
            let coord = apply_moves(&[m]).edge_orient_coord();
            assert_ne!(coord, 0, "{:?} should change edge orient coord", m);
        }
    }

    // -------------------------------------------------------
    // UD SLICE  (range 0..495)
    // -------------------------------------------------------

    #[test]
    fn udslice_solved_is_zero() {
        assert_eq!(solved().udslice_coord(), 0);
    }

    #[test]
    fn udslice_in_range() {
        for &m in &ALL_MOVES {
            let coord = apply_moves(&[m]).udslice_coord();
            assert!(coord < 495, "move {:?} gave coord {} >= 495", m, coord);
        }
    }

    #[test]
    fn udslice_move_then_inverse_is_zero() {
        let pairs = [
            (solver_move_t::U,  solver_move_t::Ui),
            (solver_move_t::D,  solver_move_t::Di),
            (solver_move_t::L,  solver_move_t::Li),
            (solver_move_t::R,  solver_move_t::Ri),
            (solver_move_t::F,  solver_move_t::Fi),
            (solver_move_t::B,  solver_move_t::Bi),
        ];
        for (m, mi) in pairs {
            let coord = apply_moves(&[m, mi]).udslice_coord();
            assert_eq!(coord, 0, "{:?} then {:?} should return to 0", m, mi);
        }
    }

    #[test]
    fn udslice_four_moves_is_zero() {
        for &m in &ALL_MOVES {
            let coord = apply_moves(&[m, m, m, m]).udslice_coord();
            assert_eq!(coord, 0, "{:?} x4 should return to 0", m);
        }
    }

    #[test]
    fn udslice_actually_changes() {
        // U/D keep UD-slice edges in their belt, but R/L/F/B displace them
        let displacing_moves = [
            solver_move_t::R, solver_move_t::L,
            solver_move_t::F, solver_move_t::B,
        ];
        for &m in &displacing_moves {
            let coord = apply_moves(&[m]).udslice_coord();
            assert_ne!(coord, 0, "{:?} should change udslice coord", m);
        }
    }

    // -------------------------------------------------------
    // CORNER PERMUTATION  (range 0..40320)
    // -------------------------------------------------------

    #[test]
    fn corner_perm_solved_is_zero() {
        assert_eq!(solved().corner_perm_coord(), 0);
    }

    #[test]
    fn corner_perm_in_range() {
        for &m in &ALL_MOVES {
            let coord = apply_moves(&[m]).corner_perm_coord();
            assert!(coord < 40320, "move {:?} gave coord {} >= 40320", m, coord);
        }
    }

    #[test]
    fn corner_perm_move_then_inverse_is_zero() {
        let pairs = [
            (solver_move_t::U,  solver_move_t::Ui),
            (solver_move_t::D,  solver_move_t::Di),
            (solver_move_t::L,  solver_move_t::Li),
            (solver_move_t::R,  solver_move_t::Ri),
            (solver_move_t::F,  solver_move_t::Fi),
            (solver_move_t::B,  solver_move_t::Bi),
        ];
        for (m, mi) in pairs {
            let coord = apply_moves(&[m, mi]).corner_perm_coord();
            assert_eq!(coord, 0, "{:?} then {:?} should return to 0", m, mi);
        }
    }

    #[test]
    fn corner_perm_four_moves_is_zero() {
        for &m in &ALL_MOVES {
            let coord = apply_moves(&[m, m, m, m]).corner_perm_coord();
            assert_eq!(coord, 0, "{:?} x4 should return to 0", m);
        }
    }

    #[test]
    fn corner_perm_actually_changes() {
        for &m in &ALL_MOVES {
            let coord = apply_moves(&[m]).corner_perm_coord();
            assert_ne!(coord, 0, "{:?} should change corner perm coord", m);
        }
    }

    // -------------------------------------------------------
    // CROSS-COORDINATE
    // -------------------------------------------------------

    #[test]
    fn all_coords_zero_means_solved() {
        // If all four coords are 0, the cube must be solved
        // (This is a sanity check on the solved state, not a proof of the converse)
        let cube = solved();
        assert_eq!(cube.corner_orient_coord(), 0);
        assert_eq!(cube.edge_orient_coord(),   0);
        assert_eq!(cube.udslice_coord(),        0);
        assert_eq!(cube.corner_perm_coord(),    0);
        assert!(cube.is_solved());
    }
}
mod rotation_tests {
    use cube_solver::*;

    
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
mod ida_tests {
    use cube_solver::*;

    // Helper
    fn solve_scramble(scramble: &[solver_move_t]) -> usize {
        let mut cube = CubieState::make_solved();
        for &m in scramble {
            cube.apply_move(m);
        }
        let mut out = [solver_move_t::U; 30];
        let n = solve_internal(cube, &mut out);

        let mut verify = cube;
        for i in 0..n {
            verify.apply_move(out[i]);
        }
        assert!(verify.is_solved());
        n
    }

    #[test]
    fn solved_cube_needs_zero_moves() {
        let cube = CubieState::make_solved();
        let mut out = [solver_move_t::U; 30];
        let n = solve_internal(cube, &mut out);
        assert_eq!(n, 0, "Solved cube should need 0 moves");
    }

    #[test]
    fn solve_u() { assert_eq!(solve_scramble(&[solver_move_t::U]), 1); }

    #[test]
    fn solve_r() { assert_eq!(solve_scramble(&[solver_move_t::R]), 1); }

    #[test]
    fn solve_f() { assert_eq!(solve_scramble(&[solver_move_t::F]), 1); }

    #[test]
    fn solve_l() { assert_eq!(solve_scramble(&[solver_move_t::L]), 1); }

    #[test]
    fn solve_d() { assert_eq!(solve_scramble(&[solver_move_t::D]), 1); }

    #[test]
    fn solve_b() { assert_eq!(solve_scramble(&[solver_move_t::B]), 1); }

    #[test]
    fn solve_two_moves_same_face() {
        // U then U = U2, should solve in 1 move (U2), not 2
        assert_eq!(solve_scramble(&[solver_move_t::U, solver_move_t::U]), 1);
    }

    #[test]
    fn solve_two_moves_different_faces() {
        let n = solve_scramble(&[solver_move_t::R, solver_move_t::U]);
        assert_eq!(n, 2);
    }

    #[test]
    fn solve_two_moves_commuting() {
        // R and L commute — still 2 moves to undo
        let n = solve_scramble(&[solver_move_t::R, solver_move_t::L]);
        assert_eq!(n, 2);
    }

    #[test]
    fn solve_sexy_move() {
        // R U Ri Ui — the "sexy move". Applied once = 6 moves to undo
        // (6 repetitions return to solved, so 1 application needs 5 to undo)
        // Actually optimal solution is 4 (the inverse: U R Ui Ri)
        let n = solve_scramble(&[
            solver_move_t::R, solver_move_t::U,
            solver_move_t::Ri, solver_move_t::Ui,
        ]);
        assert_eq!(n, 4);
    }
    #[test]
    fn solve_four_move_scramble() {
        let n = solve_scramble(&[
            solver_move_t::R, solver_move_t::U,
            solver_move_t::R, solver_move_t::U,
        ]);
        // Optimal solution may be shorter than 4 if moves cancel
        assert!(n <= 4, "Should solve in at most 4 moves, got {}", n);
    }
    #[test]
    fn solve_eight_move_scramble() {
        let n = solve_scramble(&[
            solver_move_t::R, solver_move_t::U, solver_move_t::Ri,
            solver_move_t::F, solver_move_t::L, solver_move_t::D,
            solver_move_t::Fi, solver_move_t::Ui,
        ]);
        assert!(n > 0 && n <= 20, "Expected valid solution length, got {}", n);
    }

    use::std::time::Instant;
    #[test]
    fn solve_n_move_scrambles() {
        let all_moves = [
            solver_move_t::R,  solver_move_t::U,  solver_move_t::Ri,
            solver_move_t::Ui, solver_move_t::F,  solver_move_t::R,
            solver_move_t::U,  solver_move_t::Ri, solver_move_t::Ui,
            solver_move_t::Fi, 
        ];
        let n = all_moves.len();


        for i in 0..=n {
            let scramble = &all_moves[..i];
            println!("SOLVING {} MOVES!", i);
            let start = Instant::now();
            let moves = solve_scramble(scramble);
            if i >= 6 {println!("DONE {} MOVES IN {}ms", moves, start.elapsed().as_millis());}
            else {println!("DONE {} MOVES IN {}micros", moves, start.elapsed().as_micros());}
            assert!(
                i == 0 || (moves > 0 && moves <= 30),
                "Expected valid solution length for {}-move scramble, got {}",
                i, moves
            );
        }
    }
    // -------------------------------------------------------
    // Heuristic sanity: h(solved) == 0, h(scrambled) > 0
    // -------------------------------------------------------

    #[test]
    fn heuristic_solved_is_zero() {
        assert_eq!(heuristic(&CubieState::make_solved()), 0);
    }

    #[test]
    fn heuristic_scrambled_is_nonzero() {
        let mut cube = CubieState::make_solved();
        cube.apply_move(solver_move_t::R);
        cube.apply_move(solver_move_t::U);
        assert!(heuristic(&cube) > 0, "Scrambled cube should have nonzero heuristic");
    }
    #[test]
    fn solution_actually_solves_the_cube() {
        let scramble = [
            solver_move_t::R, solver_move_t::U, solver_move_t::Fi,
            solver_move_t::L, solver_move_t::D, solver_move_t::Bi,
        ];
        let mut cube = CubieState::make_solved();
        for &m in &scramble { cube.apply_move(m); }

        let mut out = [solver_move_t::U; 30];
        let n = solve_internal(cube, &mut out);

        // Apply solution to the scrambled cube
        let mut result = cube;
        for i in 0..n { result.apply_move(out[i]); }

        assert!(result.is_solved(),
            "Applying solution to scrambled cube did not produce solved state");
    }

    // #[test]
    // fn solve_test_cube_lol() {
    //     let stickers: [u8; 54] = [5,5,0,0,0,2,2,3,5,4,5,0,1,1,5,5,1,4,3,3,4,1,2,4,1,2,1,1,0,5,2,3,4,4,0,3,0,4,3,2,4,5,2,1,3,2,3,0,0,5,3,1,4,2];
    //     let cube = convert_to_cubie(stickers);
    //     let mut out = [solver_move_t::U; 30];
        
    //     let n = solve_internal(cube, &mut out);
    //     let mut result = cube;

    //     for i in 0..n { result.apply_move(out[i]);}

    //     assert!(result.is_solved());
    // }

}