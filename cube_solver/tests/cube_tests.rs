//
// cube_tests.rs
//
//  Created on: Jan 23, 2026
//      Author: jorgelarach
//

use std::sync::OnceLock;

static TABLES_INIT: OnceLock<()> = OnceLock::new();

fn init_tables() {
    TABLES_INIT.get_or_init(|| {
        cube_solver::runtime_tables::init();
    });
}

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
    // EDGE PERMUTATION (0..40320)
    // -------------------------------------------------------

   #[test]
    fn edge_perm_solved_is_zero() {
        assert_eq!(solved().edge_perm_coord(), 0);
    }

    #[test]
    fn edge_perm_in_range() {
        for &m in &ALL_MOVES {
            let coord = apply_moves(&[m]).edge_perm_coord();
            assert!(coord < 40320, "move {:?} gave coord {} >= 40320", m, coord);
        }
    }

    #[test]
    fn edge_perm_move_then_inverse_is_zero() {
        let pairs = [
            (solver_move_t::U,  solver_move_t::Ui),
            (solver_move_t::D,  solver_move_t::Di),
            (solver_move_t::L,  solver_move_t::Li),
            (solver_move_t::R,  solver_move_t::Ri),
            (solver_move_t::F,  solver_move_t::Fi),
            (solver_move_t::B,  solver_move_t::Bi),
        ];
        for (m, mi) in pairs {
            let coord = apply_moves(&[m, mi]).edge_perm_coord();
            assert_eq!(coord, 0, "{:?} then {:?} should return to 0", m, mi);
        }
    }

    #[test]
    fn edge_perm_four_moves_is_zero() {
        for &m in &ALL_MOVES {
            let coord = apply_moves(&[m, m, m, m]).edge_perm_coord();
            assert_eq!(coord, 0, "{:?} x4 should return to 0", m);
        }
    }

    #[test]
    fn edge_perm_actually_changes() {
        for &m in &ALL_MOVES {
            let coord = apply_moves(&[m]).edge_perm_coord();
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

mod kociemba_tests {
    use cube_solver::*;

    use super::*;

    // -------------------------------------------------------
    // Helper: apply a scramble, run the full solver, verify
    // the solution actually solves the cube, return total length
    // -------------------------------------------------------
    fn run_kociemba(scramble: &[solver_move_t]) -> usize {
        let mut cube = CubieState::make_solved();
        for &m in scramble {
            cube.apply_move(m);
        }

        let mut out = [solver_move_t::U; 70]; // 30 phase1 + 40 phase2
        let n = solve_internal(cube, &mut out);

        // VERIFY: applying the solution to the scrambled cube must yield solved
        let mut verify = cube;
        for i in 0..n {
            verify.apply_move(out[i]);
        }
        assert!(
            verify.is_solved(),
            "Solution FAILED to solve the cube.\nScramble: {:?}\nSolution ({} moves): {:?}",
            scramble,
            n,
            &out[..n]
        );

        n
    }

    // -------------------------------------------------------
    // Helper: apply a scramble and check Phase 1 specifically.
    // Verifies that after applying the returned moves, the cube
    // is in G1 (CO=0, EO=0, UD-slice=0).
    // -------------------------------------------------------
    fn run_phase1_only(scramble: &[solver_move_t]) -> usize {
        runtime_tables::init();
        let mut cube = CubieState::make_solved();
        let mut total_nodes: u32 = 0;
        for &m in scramble {
            cube.apply_move(m);
        }

        let mut path = [solver_move_t::U; 30];
        let n = kociemba_phase1(&cube, &mut path, &mut total_nodes);

        // Apply phase 1 moves and verify G1
        let mut g1 = cube;
        for i in 0..n {
            g1.apply_move(path[i]);
        }

        assert!(
            is_phase1_solved(&g1),
            "Phase 1 did not reach G1.\nScramble: {:?}\nPhase 1 moves ({} moves): {:?}\nCO: {}, EO: {}, UD: {}",
            scramble, n, &path[..n],
            g1.corner_orient_coord(),
            g1.edge_orient_coord(),
            g1.udslice_coord()
        );

        n
    }

    // -------------------------------------------------------
    // Helper: build a G1 cube by applying phase 1 first,
    // then run phase 2 and verify fully solved
    // -------------------------------------------------------
    fn run_phase2_only(scramble: &[solver_move_t]) -> usize {
        runtime_tables::init();
        let mut cube = CubieState::make_solved();
        let mut total_nodes: u32 = 0;
        for &m in scramble {
            cube.apply_move(m);
        }

        // Get to G1 first
        let mut p1_path = [solver_move_t::U; 30];
        let p1_len = kociemba_phase1(&cube, &mut p1_path, &mut total_nodes);
        let mut g1_cube = cube;
        for i in 0..p1_len {
            g1_cube.apply_move(p1_path[i]);
        }
        assert!(is_phase1_solved(&g1_cube), "Could not reach G1 for phase 2 test");

        // Now run phase 2
        let mut p2_path = [solver_move_t::U; 40];
        let p2_len = kociemba_phase2(&g1_cube, &mut p2_path, &mut total_nodes);

        // Apply phase 2 and verify fully solved
        let mut result = g1_cube;
        for i in 0..p2_len {
            result.apply_move(p2_path[i]);
        }
        assert!(
            result.is_solved(),
            "Phase 2 did not fully solve the cube.\nPhase 2 moves ({} moves): {:?}",
            p2_len, &p2_path[..p2_len]
        );

        p2_len
    }

    // -------------------------------------------------------
    // TRIVIAL CASES
    // -------------------------------------------------------

    #[test]
    fn already_solved_returns_zero() {
        let cube = CubieState::make_solved();
        let mut out = [solver_move_t::U; 70];
        let n = solve_internal(cube, &mut out);
        assert_eq!(n, 0, "Already solved cube should need 0 moves");
    }

    #[test]
    fn phase1_solved_cube_is_already_g1() {
        assert!(is_phase1_solved(&CubieState::make_solved()));
    }

    #[test]
    fn phase2_solved_cube_is_already_solved() {
        assert!(is_phase2_solved(&CubieState::make_solved()));
    }

    #[test]
    fn phase1_on_solved_cube_returns_zero() {
        let cube = CubieState::make_solved();
        let mut total_nodes: u32 = 0;
        let mut path = [solver_move_t::U; 30];
        let n = kociemba_phase1(&cube, &mut path, &mut total_nodes);
        assert_eq!(n, 0);
    }

    #[test]
    fn phase2_on_solved_cube_returns_zero() {
        let cube = CubieState::make_solved();
        let mut total_nodes: u32 = 0;
        let mut path = [solver_move_t::U; 40];
        let n = kociemba_phase2(&cube, &mut path, &mut total_nodes);
        assert_eq!(n, 0);
    }

    // -------------------------------------------------------
    // HEURISTIC SANITY
    // -------------------------------------------------------

    #[test]
    fn phase1_heuristic_solved_is_zero() {
        runtime_tables::init();
        assert_eq!(heuristic_phase1(&CubieState::make_solved()), 0);
    }

    #[test]
    fn phase2_heuristic_solved_is_zero() {
        runtime_tables::init();
        assert_eq!(heuristic_phase2(&CubieState::make_solved()), 0);
    }

    #[test]
    fn phase1_heuristic_scrambled_is_nonzero() {
        runtime_tables::init();
        // R flips edges and twists corners — definitely not in G1
        let mut cube = CubieState::make_solved();
        cube.apply_move(solver_move_t::R);
        assert!(heuristic_phase1(&cube) > 0);
    }

    #[test]
    fn phase2_heuristic_scrambled_is_nonzero() {
        runtime_tables::init();
        // U moves corners out of home slots
        let mut cube = CubieState::make_solved();
        cube.apply_move(solver_move_t::U);
        assert!(heuristic_phase2(&cube) > 0);
    }

    // -------------------------------------------------------
    // 1-MOVE SCRAMBLES — full solver
    // -------------------------------------------------------

    #[test]
    fn solve_u()  { assert!(run_kociemba(&[solver_move_t::U])  <= 2); }

    #[test]
    fn solve_d()  { assert!(run_kociemba(&[solver_move_t::D])  <= 2); }

    #[test]
    fn solve_r()  { assert!(run_kociemba(&[solver_move_t::R])  <= 2); }

    #[test]
    fn solve_l()  { assert!(run_kociemba(&[solver_move_t::L])  <= 2); }

    #[test]
    fn solve_f()  { assert!(run_kociemba(&[solver_move_t::F])  <= 2); }

    #[test]
    fn solve_b()  { assert!(run_kociemba(&[solver_move_t::B])  <= 2); }

    #[test]
    fn solve_r2() { assert!(run_kociemba(&[solver_move_t::R2]) <= 2); }

    #[test]
    fn solve_u2() { assert!(run_kociemba(&[solver_move_t::U2]) <= 2); }

    // -------------------------------------------------------
    // PHASE 1 ISOLATION TESTS
    // Verify G1 is reached correctly before worrying about Phase 2
    // -------------------------------------------------------

    #[test]
    fn phase1_reaches_g1_after_r() {
        run_phase1_only(&[solver_move_t::R]);
    }

    #[test]
    fn phase1_reaches_g1_after_sexy_move() {
        // R U Ri Ui twists corners and flips edges — not in G1
        run_phase1_only(&[
            solver_move_t::R, solver_move_t::U,
            solver_move_t::Ri, solver_move_t::Ui,
        ]);
    }

    #[test]
    fn phase1_reaches_g1_after_6_moves() {
        run_phase1_only(&[
            solver_move_t::R, solver_move_t::U, solver_move_t::Fi,
            solver_move_t::L, solver_move_t::D, solver_move_t::Bi,
        ]);
    }

    #[test]
    fn phase1_reaches_g1_after_10_moves() {
        run_phase1_only(&[
            solver_move_t::R,  solver_move_t::U,  solver_move_t::Ri,
            solver_move_t::Ui, solver_move_t::F,  solver_move_t::R,
            solver_move_t::U,  solver_move_t::Ri, solver_move_t::Ui,
            solver_move_t::Fi,
        ]);
    }

    #[test]
    fn phase1_timing_benchmark() {
        use std::time::Instant;
        runtime_tables::init();

        // These are specifically chosen to stress Phase 1
        let scrambles: &[(&str, &[solver_move_t])] = &[
            ("R",          &[solver_move_t::R]),
            ("R U Ri Ui",  &[solver_move_t::R, solver_move_t::U, solver_move_t::Ri, solver_move_t::Ui]),
            ("6 moves",    &[solver_move_t::R, solver_move_t::U, solver_move_t::Fi,
                            solver_move_t::L, solver_move_t::D, solver_move_t::Bi]),
            ("8 moves",    &[solver_move_t::R, solver_move_t::U, solver_move_t::Ri,
                            solver_move_t::Ui, solver_move_t::F, solver_move_t::R,
                            solver_move_t::U,  solver_move_t::Ri]),
        ];

        for (name, scramble) in scrambles {
            let mut cube = CubieState::make_solved();
            for &m in *scramble { cube.apply_move(m); }

            println!(
                "Initial coords: CO={} EO={} UD={} CP={}",
                cube.corner_orient_coord(),
                cube.edge_orient_coord(),
                cube.udslice_coord(),
                cube.corner_perm_coord()
            );

            let mut path = [solver_move_t::U; 30];
            let start = Instant::now();
            let mut total_nodes: u32 = 0;
            let n = kociemba_phase1(&cube, &mut path, &mut total_nodes);
            let elapsed = start.elapsed();

            println!(
                "Phase 1 '{}': {} moves in {}ms",
                name, n, elapsed.as_millis()
            );

            // After phase 1 completes, apply moves to get G1 cube first
            let mut g1_cube = cube;
            for j in 0..n { g1_cube.apply_move(path[j]); }

            println!(
                "P1 heuristic={} P2 heuristic(on G1 cube)={} True P1 depth={}",
                heuristic_phase1(&cube),
                heuristic_phase2(&g1_cube), // call on G1 cube, not scrambled cube
                n
            );

        }
    }

    // -------------------------------------------------------
    // PHASE 2 ISOLATION TESTS
    // These confirm Phase 2 can close out from a G1 state
    // -------------------------------------------------------

    #[test]
    fn phase2_solves_from_g1_after_r() {
        run_phase2_only(&[solver_move_t::R]);
    }

    #[test]
    fn phase2_solves_from_g1_after_sexy_move() {
        run_phase2_only(&[
            solver_move_t::R, solver_move_t::U,
            solver_move_t::Ri, solver_move_t::Ui,
        ]);
    }

    #[test]
    fn phase2_solves_from_g1_after_10_moves() {
        run_phase2_only(&[
            solver_move_t::R,  solver_move_t::U,  solver_move_t::Ri,
            solver_move_t::Ui, solver_move_t::F,  solver_move_t::R,
            solver_move_t::U,  solver_move_t::Ri, solver_move_t::Ui,
            solver_move_t::Fi,
        ]);
    }

    // -------------------------------------------------------
    // FULL SOLVER — correctness at increasing depths
    // We don't assert exact solution length — Kociemba is not
    // always optimal — but we verify the solution is valid and
    // within a reasonable upper bound (70 moves worst case)
    // -------------------------------------------------------

    #[test]
    fn solve_4_move_scramble() {
        let n = run_kociemba(&[
            solver_move_t::R, solver_move_t::U,
            solver_move_t::L, solver_move_t::D,
        ]);
        assert!(n > 0 && n <= 70);
    }

    #[test]
    fn solve_sexy_move_once() {
        
        // R U Ri Ui — classic 4 move pattern
        let n = run_kociemba(&[
            solver_move_t::R,  solver_move_t::U,
            solver_move_t::Ri, solver_move_t::Ui,
        ]);
        assert!(n > 0 && n <= 70);
    }

    #[test]
    fn solve_6_move_scramble() {
        let n = run_kociemba(&[
            solver_move_t::R, solver_move_t::U, solver_move_t::Fi,
            solver_move_t::L, solver_move_t::D, solver_move_t::Bi,
        ]);
        assert!(n > 0 && n <= 70);
    }

    #[test]
    fn solve_8_move_scramble() {
        let n = run_kociemba(&[
            solver_move_t::R, solver_move_t::U, solver_move_t::Ri,
            solver_move_t::F, solver_move_t::L, solver_move_t::D,
            solver_move_t::Fi, solver_move_t::Ui,
        ]);
        assert!(n > 0 && n <= 70);
    }

    #[test]
    fn solve_10_move_scramble() {
        let n = run_kociemba(&[
            solver_move_t::R,  solver_move_t::U,  solver_move_t::Ri,
            solver_move_t::Ui, solver_move_t::F,  solver_move_t::R,
            solver_move_t::U,  solver_move_t::Ri, solver_move_t::Ui,
            solver_move_t::Fi,
        ]);
        assert!(n > 0 && n <= 70);
    }

    #[test]
    fn solve_12_move_scramble() {
        let n = run_kociemba(&[
            solver_move_t::R,  solver_move_t::U,  solver_move_t::Ri,
            solver_move_t::Ui, solver_move_t::R,  solver_move_t::U,
            solver_move_t::Ri, solver_move_t::Ui, solver_move_t::R,
            solver_move_t::U,  solver_move_t::Ri, solver_move_t::Ui,
        ]);
        assert!(n > 0 && n <= 70);
    }

    #[test]
    fn solve_15_move_scramble() {
        let n = run_kociemba(&[
            solver_move_t::R,  solver_move_t::U,  solver_move_t::Bi,
            solver_move_t::D,  solver_move_t::Li, solver_move_t::F,
            solver_move_t::R2, solver_move_t::U2, solver_move_t::B,
            solver_move_t::Di, solver_move_t::L,  solver_move_t::Fi,
            solver_move_t::R,  solver_move_t::U,  solver_move_t::Ri,
        ]);
        assert!(n > 0 && n <= 70);
    }

    #[test]
    fn solve_20_move_scramble() {
        // A deep scramble — the real test of the algorithm
        let n = run_kociemba(&[
            solver_move_t::R,  solver_move_t::U,  solver_move_t::Bi,
            solver_move_t::D,  solver_move_t::Li, solver_move_t::F,
            solver_move_t::R2, solver_move_t::U2, solver_move_t::B,
            solver_move_t::Di, solver_move_t::L,  solver_move_t::Fi,
            solver_move_t::R,  solver_move_t::U,  solver_move_t::Ri,
            solver_move_t::Ui, solver_move_t::F2, solver_move_t::L2,
            solver_move_t::D2, solver_move_t::B2,
        ]);
        assert!(n > 0 && n <= 70);
    }

    #[test]
    fn solve_n_move_scrambles_kociemba() {
        use std::time::Instant;
        runtime_tables::init();

        let all_moves = [
            solver_move_t::R,  solver_move_t::U,  solver_move_t::Bi,
            solver_move_t::D,  solver_move_t::Li, solver_move_t::F,
            solver_move_t::R2, solver_move_t::U2, solver_move_t::B,
            solver_move_t::Di, solver_move_t::L,  solver_move_t::Fi,
            solver_move_t::R,  solver_move_t::U,  solver_move_t::Ri,
            solver_move_t::Ui, solver_move_t::F2, solver_move_t::L2,
            solver_move_t::D2, solver_move_t::B2, solver_move_t::R,
            solver_move_t::F2, solver_move_t::Li, solver_move_t::R,
            solver_move_t::L,  solver_move_t::Ui,  solver_move_t::Ri,
            solver_move_t::D2,  solver_move_t::U,  solver_move_t::Fi,
            solver_move_t::Li, solver_move_t::F2, solver_move_t::B2,
            solver_move_t::R2, solver_move_t::Li, solver_move_t::Di,
            solver_move_t::U2,  solver_move_t::L,  solver_move_t::R2,
            solver_move_t::U,
        ];

        let n = all_moves.len();

        for i in 0..=n {
            let scramble = &all_moves[..i];

            // Build scrambled cube
            let mut cube = CubieState::make_solved();
            let mut total_nodes: u32 = 0;
            for &m in scramble { cube.apply_move(m); }

            // let mut out = [solver_move_t::U; 70];

            println!("--- SOLVING {}-MOVE SCRAMBLE ---", i);

            // Time Phase 1 in isolation
            let mut p1_path = [solver_move_t::U; 30];
            let p1_start = Instant::now();
            let p1_len = kociemba_phase1(&cube, &mut p1_path, &mut total_nodes);
            let p1_elapsed = p1_start.elapsed();

            // Apply Phase 1 to get G1 cube
            let mut g1_cube = cube;
            for j in 0..p1_len { g1_cube.apply_move(p1_path[j]); }

            println!(
                "  Phase 1: {} moves in {}",
                p1_len,
                if p1_elapsed.as_millis() > 0 {
                    format!("{}ms", p1_elapsed.as_millis())
                } else {
                    format!("{}μs", p1_elapsed.as_micros())
                }
            );
            println!(
                "  Phase 1 total nodes explored: {}",
                total_nodes
            );
            println!(
                "  G1 check after Phase 1: CO={} EO={} UD={}",
                g1_cube.corner_orient_coord(),
                g1_cube.edge_orient_coord(),
                g1_cube.udslice_coord()
            );

            // Time Phase 2 in isolation
            let mut p2_path = [solver_move_t::U; 40];
            let p2_start = Instant::now();
            let p2_len = kociemba_phase2(&g1_cube, &mut p2_path, &mut total_nodes);
            let p2_elapsed = p2_start.elapsed();

            println!(
                "  Phase 2: {} moves in {}",
                p2_len,
                if p2_elapsed.as_millis() > 0 {
                    format!("{}ms", p2_elapsed.as_millis())
                } else {
                    format!("{}μs", p2_elapsed.as_micros())
                }
            );
            println!(
                "  Phase 2 total nodes explored: {}",
                total_nodes
            );

            // // Time full solver end to end
            // let total_start = Instant::now();
            // let total_len = solve_internal(cube, &mut out);
            // let total_elapsed = total_start.elapsed();

            // println!(
            //     "  Total:   {} moves in {}",
            //     total_len,
            //     if total_elapsed.as_millis() > 0 {
            //         format!("{}ms", total_elapsed.as_millis())
            //     } else {
            //         format!("{}μs", total_elapsed.as_micros())
            //     }
            // );

            let total_len = p1_len + p2_len;
            // Verify solution correctness
            if total_len > 0 {
                let mut verify = cube;
                for j in 0..p1_len { verify.apply_move(p1_path[j]); }
                for j in 0..p2_len { verify.apply_move(p2_path[j]); }
                assert!(
                    verify.is_solved(),
                    "{}-move scramble: solution is WRONG ({} moves returned)",
                    i, total_len
                );
                println!("  Correctness: PASS");
            }

            assert!(
                i == 0 || (total_len > 0 && total_len <= 70),
                "Expected valid solution for {}-move scramble, got {}",
                i, total_len
            );

            println!();
        }
    }

    #[test]
    fn solve_lol() {
        use std::time::Instant;
        runtime_tables::init();

        let stickers:[u8;54] = [5,5,0,0,0,2,2,3,5,4,5,0,1,1,5,5,1,4,3,3,4,1,2,4,1,2,1,1,0,5,2,3,4,4,0,3,0,4,3,2,4,5,2,1,3,2,3,0,0,5,3,1,4,2];
        let cube = convert_to_cubie(stickers);
        let mut out = [solver_move_t::U; 70];
        // Time full solver end to end
        let total_start = Instant::now();
        let total_len = solve_internal(cube, &mut out);
        let total_elapsed = total_start.elapsed();

        println!(
            "  Total:   {} moves in {}",
            total_len,
            if total_elapsed.as_millis() > 0 {
                format!("{}ms", total_elapsed.as_millis())
            } else {
                format!("{}μs", total_elapsed.as_micros())
            }
        );

        for i in 0..total_len {
            print_move(out[i]);
        }


    }

    fn print_move(m: solver_move_t) {
        match m {
            solver_move_t::U => println!("U, "),
            solver_move_t::Ui => println!("Ui, "),
            solver_move_t::U2 => println!("U2, "),
            solver_move_t::D => println!("D, "),
            solver_move_t::Di => println!("Di, "),
            solver_move_t::D2 => println!("D2, "),
            solver_move_t::L => println!("L, "),
            solver_move_t::Li => println!("Li, "),
            solver_move_t::L2 => println!("L2, "),
            solver_move_t::R => println!("R, "),
            solver_move_t::Ri => println!("Ri, "),
            solver_move_t::R2 => println!("R2, "),
            solver_move_t::F => println!("F, "),
            solver_move_t::Fi => println!("Fi, "),
            solver_move_t::F2 => println!("F2, "),
            solver_move_t::B => println!("B, "),
            solver_move_t::Bi => println!("Bi, "),
            solver_move_t::B2 => println!("B2, "),
        }
    }
    // -------------------------------------------------------
    // G1 INVARIANT: Phase 2 moves must never break G1
    // If Phase 1 solved correctly, Phase 2 moves must keep
    // CO=0, EO=0, UD-slice=0 throughout
    // -------------------------------------------------------

    #[test]
    fn phase2_moves_preserve_g1() {
        // Start from solved (which is in G1)
        // Apply every Phase 2 move and confirm G1 is preserved
        let phase2_moves = [
            solver_move_t::U,  solver_move_t::Ui, solver_move_t::U2,
            solver_move_t::D,  solver_move_t::Di, solver_move_t::D2,
            solver_move_t::R2, solver_move_t::L2,
            solver_move_t::F2, solver_move_t::B2,
        ];
        for &m in &phase2_moves {
            let mut cube = CubieState::make_solved();
            cube.apply_move(m);
            assert!(
                is_phase1_solved(&cube),
                "Phase 2 move {:?} broke G1: CO={}, EO={}, UD={}",
                m,
                cube.corner_orient_coord(),
                cube.edge_orient_coord(),
                cube.udslice_coord()
            );
        }
    }

    // -------------------------------------------------------
    // SOLUTION VALIDITY: explicitly verify the move sequence
    // works when applied step by step
    // -------------------------------------------------------

    #[test]
    fn solution_applies_correctly_step_by_step() {
        let scramble = [
            solver_move_t::R, solver_move_t::U, solver_move_t::Fi,
            solver_move_t::L, solver_move_t::D, solver_move_t::Bi,
        ];

        let mut cube = CubieState::make_solved();
        for &m in &scramble { cube.apply_move(m); }

        let mut out = [solver_move_t::U; 70];
        let n = solve_internal(cube, &mut out);
        assert!(n > 0, "Solver returned 0 moves for a scrambled cube");

        // Apply each move one at a time and confirm final state is solved
        let mut result = cube;
        for i in 0..n {
            result.apply_move(out[i]);
        }
        assert!(result.is_solved(),
            "Step-by-step application of solution did not yield solved cube");
    }

}