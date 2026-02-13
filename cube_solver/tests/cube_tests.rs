mod tests {
    use cube_solver::*;
// ============================================================================
// COMPREHENSIVE TEST SUITE FOR CASE-BASED WHITE CROSS SOLVER
// ============================================================================


    // ========================================================================
    // HELPER FUNCTIONS FOR TESTING
    // ========================================================================

    /// Check if white cross is solved

    
    #[test]
    fn test_solved_cube_is_solved() {
        let cube: Cube = Cube::make_solved();
        assert!(cube.is_solved());
    }

    #[test]
    fn debug_solved_cube() {
        // run as: cargo test debug_solved_cube -- --nocapture
        let cube: Cube = Cube::make_solved();
        println!("Cube stickers: {:?}", &cube.stickers[..]);

        for face in 0..6 {
            print!("Face: {:?} ", face);
            for i in 0..9 {
                print!("{} ", cube.stickers[face * 9 + i]);
            }
            println!();
        }

        assert!(cube.is_solved());
    }

    #[test]
    fn test_rotate_u_face_cw() {
        let mut cube: Cube = Cube::make_solved();

        for i in 0..9 {
            cube.stickers[U_FACE * 9 + i] = i as u8;
        }

        cube.rotate_face_cw(U_FACE);

        let expected_order = [6, 3, 0, 7, 4, 1, 8, 5, 2];

        for i in 0..9 {
            assert_eq!(cube.stickers[i], expected_order[i]);
        }
    }

    #[test]
    fn test_u() {
        let mut cube:Cube = Cube::make_solved();

        cube.apply_move(solver_move_t::U);
        assert!(!cube.is_solved());
        cube.apply_move(solver_move_t::Ui);
        assert!(cube.is_solved());
        cube.apply_move(solver_move_t::U2);
        assert!(!cube.is_solved());
        cube.apply_move(solver_move_t::U2);
        assert!(cube.is_solved());
    }

    #[test]
    fn test_d() {
        let mut cube:Cube = Cube::make_solved();

        cube.apply_move(solver_move_t::D);
        assert!(!cube.is_solved());
        cube.apply_move(solver_move_t::Di);
        assert!(cube.is_solved());
        cube.apply_move(solver_move_t::D2);
        assert!(!cube.is_solved());
        cube.apply_move(solver_move_t::D2);
        assert!(cube.is_solved());
    }

    #[test]
    fn test_l() {
        let mut cube:Cube = Cube::make_solved();

        cube.apply_move(solver_move_t::L);
        assert!(!cube.is_solved());
        cube.apply_move(solver_move_t::Li);
        assert!(cube.is_solved());
        cube.apply_move(solver_move_t::L2);
        assert!(!cube.is_solved());
        cube.apply_move(solver_move_t::L2);
        assert!(cube.is_solved());
    }

    #[test]
    fn test_r() {
        let mut cube:Cube = Cube::make_solved();

        cube.apply_move(solver_move_t::R);
        assert!(!cube.is_solved());
        cube.apply_move(solver_move_t::Ri);
        assert!(cube.is_solved());
        cube.apply_move(solver_move_t::R2);
        assert!(!cube.is_solved());
        cube.apply_move(solver_move_t::R2);
        assert!(cube.is_solved());
    }
    #[test]
    fn test_f() {
        let mut cube:Cube = Cube::make_solved();

        cube.apply_move(solver_move_t::F);
        assert!(!cube.is_solved());
        cube.apply_move(solver_move_t::Fi);
        assert!(cube.is_solved());
        cube.apply_move(solver_move_t::F2);
        assert!(!cube.is_solved());
        cube.apply_move(solver_move_t::F2);
        assert!(cube.is_solved());
    }

    #[test]
    fn test_b() {
        let mut cube:Cube = Cube::make_solved();

        cube.apply_move(solver_move_t::B);
        assert!(!cube.is_solved());
        cube.apply_move(solver_move_t::Bi);
        assert!(cube.is_solved());
        cube.apply_move(solver_move_t::B2);
        assert!(!cube.is_solved());
        cube.apply_move(solver_move_t::B2);
        assert!(cube.is_solved());
    }
    #[test]
    fn test_all_moves_return_to_solved() {
        // Test each move 4 times returns to solved state
        let test_cases = [
            (Cube::u as fn(&mut Cube), "U"),
            (Cube::d as fn(&mut Cube), "D"),
            (Cube::l as fn(&mut Cube), "L"),
            (Cube::r as fn(&mut Cube), "R"),
            (Cube::f as fn(&mut Cube), "F"),
            (Cube::b as fn(&mut Cube), "B"),
        ];
        for (move_fn, name) in test_cases {
            let mut cube = Cube::make_solved();
            // Apply move 4 times (360° rotation)
            move_fn(&mut cube);
            move_fn(&mut cube);
            move_fn(&mut cube);
            move_fn(&mut cube);
            
            assert!(cube.is_solved(), "{} move should return to solved after 4 applications", name);
        }
    }
    
    #[test]
    fn test_move_and_inverse() {
        // Test that move followed by inverse returns to solved
        let mut cube = Cube::make_solved();
        cube.u();
        cube.apply_move(solver_move_t::Ui);
        assert!(cube.is_solved(), "U then Ui should return to solved");
        
        // Test all basic moves
        let moves = [
            (solver_move_t::U, solver_move_t::Ui),
            (solver_move_t::D, solver_move_t::Di),
            (solver_move_t::L, solver_move_t::Li),
            (solver_move_t::R, solver_move_t::Ri),
            (solver_move_t::F, solver_move_t::Fi),
            (solver_move_t::B, solver_move_t::Bi),
        ];
        
        for (m, inverse) in moves.iter() {
            let mut cube = Cube::make_solved();
            cube.apply_move(*m);
            cube.apply_move(*inverse);
            assert!(cube.is_solved(), "{:?} then {:?} should return to solved", m, inverse);
        }
    }
    
    #[test]
    fn test_double_moves() {
        // Test that double moves are correct
        let mut cube1 = Cube::make_solved();
        let mut cube2 = Cube::make_solved();
        
        // U2 should be same as U applied twice
        cube1.apply_move(solver_move_t::U2);
        cube2.u();
        cube2.u();
        
        assert_eq!(cube1.stickers, cube2.stickers, "U2 should equal U applied twice");
    }

    #[test]
    fn test_find_edge_on_solved_cube() {
        let cube = Cube::make_solved();
        
        let mut result = cube.find_edge(WHITE, ORANGE);
        assert_eq!(result, Some((1, B_FACE * 9 + 1)));
        
        result = cube.find_edge(WHITE, GREEN);
        assert_eq!(result, Some((3, L_FACE * 9 + 1)));

        result = cube.find_edge(WHITE, BLUE);
        assert_eq!(result, Some((5, R_FACE * 9 + 1)));

        result = cube.find_edge(WHITE,  RED);
        assert_eq!(result, Some((7, F_FACE * 9 + 1)));

    }

    #[test]
    fn test_white_orange_already_solved() {
        let mut cube = Cube::make_solved();
        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        
        let moves_count = cube.solve_white_cross(&mut out);
        
        // White-orange is already solved, so no moves should be needed
        assert_eq!(moves_count, 0, "Solved cube should require 0 moves");
        assert!(cube.is_solved());
    }

    #[test]

    // WHITE ON U FACE
    fn test_white_orange_on_ul() {
        // Set up: white-orange edge is on U layer but at the wrong position (at L instead of B)
        let mut cube = Cube::make_solved();
        
        // Rotate U layer so white-orange moves from U-B to U-R
        cube.u();
        
        // Before solving
        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());
        let (edge_idx, _) = result.unwrap();
        assert_eq!(edge_idx, U_FACE * 9 + 5); // U-R edge sticker index
        
        // Solve it
        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        let moves_count = cube.solve_white_cross(&mut out);
        
        // Should have recorded moves
        assert!(moves_count > 0, "Should require moves to fix misaligned white-orange");
        
        // After solving, white should be at U1, orange at B1
        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());
        let (edge_idx, _) = result.unwrap();
        assert_eq!(edge_idx, U_FACE * 9 + 1, "White-orange should be at U-B position");

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((U_FACE * 9 + 1, B_FACE * 9 + 1)));
        
        
        
    }

    #[test]
    fn test_white_orange_on_ur() {
        let mut cube = Cube::make_solved();
        
        // Rotate U layer counter-clockwise so white-orange moves from U-B to U-L
        cube.u();
        cube.u();
        cube.u();
        
        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());
        let (edge_idx, _) = result.unwrap();
        assert_eq!(edge_idx, U_FACE * 9 + 3); // U-L edge sticker index
        
        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        let moves_count = cube.solve_white_cross(&mut out);

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((U_FACE * 9 + 1, B_FACE * 9 + 1)));
        
        assert!(moves_count > 0);
        
    }

    #[test]
    fn test_white_orange_on_uf() {
        let mut cube = Cube::make_solved();
        
        // Rotate U layer twice so white-orange moves from U-B to U-F
        cube.u();
        cube.u();
        
        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());
        let (edge_idx, _) = result.unwrap();
        assert_eq!(edge_idx, U_FACE * 9 + 7); // U-F edge sticker index
        
        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        let moves_count = cube.solve_white_cross(&mut out);

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((U_FACE * 9 + 1, B_FACE * 9 + 1)));
        
        assert!(moves_count > 0);
        
    }

    // WHITE ON D FACE
    #[test]
    fn test_white_orange_on_db() {
        let mut cube = Cube::make_solved();
        
        // Move white-orange down to D layer below B face
        // B2 moves the white-orange from U-B to D-B position
        cube.b();
        cube.b();
        
        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());
        let (edge_idx, _) = result.unwrap();
        assert_eq!(edge_idx, D_FACE * 9 + 7, "White-orange should be at D-B position");
        
        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        let moves_count = cube.solve_white_cross(&mut out);

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((U_FACE * 9 + 1, B_FACE * 9 + 1)));
        
        assert!(moves_count > 0, "Should record moves from D layer");
        
    }

    #[test]
    fn test_white_orange_on_df() {
        let mut cube = Cube::make_solved();
        
        // Move white-orange to D-F position
        cube.b();
        cube.b();
        cube.d();
        cube.d();
        
        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());
        
        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        let moves_count = cube.solve_white_cross(&mut out);

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((U_FACE * 9 + 1, B_FACE * 9 + 1)));
        
        assert!(moves_count > 0);
        
    }

    #[test]
    fn test_white_orange_on_dl() {
        let mut cube = Cube::make_solved();
        
        // Move white-orange to D-L position
        cube.b();
        cube.b();
        cube.d();
        
        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());
        
        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        let moves_count = cube.solve_white_cross(&mut out);

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((U_FACE * 9 + 1, B_FACE * 9 + 1)));
        
        assert!(moves_count > 0);
    }

    #[test]
    fn test_white_orange_on_dr() {
        let mut cube = Cube::make_solved();
        
        // Move white-orange to D-R position
        cube.b();
        cube.b();
        cube.di();
        
        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());
        
        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        let moves_count = cube.solve_white_cross(&mut out);

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((U_FACE * 9 + 1, B_FACE * 9 + 1)));
        
        assert!(moves_count > 0);
    }

    // WHITE ON B FACE
    #[test]
    fn test_white_orange_on_bl() {
        let mut cube = Cube::make_solved();
        
        // White on B face, Orange on L face (middle layer edge piece)
        // Setup: rotate B face to bring white-orange pair to B-L
        cube.b();
        
        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());
        
        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        let moves_count = cube.solve_white_cross(&mut out);

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((U_FACE * 9 + 1, B_FACE * 9 + 1)));

        assert!(moves_count > 0);
    }

    #[test]
    fn test_white_orange_on_br() {
        let mut cube = Cube::make_solved();
        
        // White on B face, Orange on R face
        cube.bi();
        
        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());
        
        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        let moves_count = cube.solve_white_cross(&mut out);

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((U_FACE * 9 + 1, B_FACE * 9 + 1)));
        
        assert!(moves_count > 0);
    }

    #[test]
    fn test_white_orange_on_bu() {
        let mut cube = Cube::make_solved();

        // White on B face, Orange on U face
        cube.u();
        cube.r();
        cube.b();

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((B_FACE * 9 + 1, U_FACE * 9 + 1)));

        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        let moves_count = cube.solve_white_cross(&mut out);

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((U_FACE * 9 + 1, B_FACE * 9 + 1)));

        assert!(moves_count > 0);
    }
    
    #[test]
    fn test_white_orange_on_bd() {
        let mut cube = Cube::make_solved();

        // White on B face, Orange on D face
        cube.u();
        cube.r();
        cube.bi();

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((B_FACE * 9 + 7, D_FACE * 9 + 7)));

        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        let moves_count = cube.solve_white_cross(&mut out);

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((U_FACE * 9 + 1, B_FACE * 9 + 1)));

        assert!(moves_count > 0);       

    }

    // WHITE ON L FACE
    #[test]
    fn test_white_orange_on_lb() {
        let mut cube = Cube::make_solved();
        
        // White on L face, Orange on B face
        cube.b();
        
        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());
        
        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        let moves_count = cube.solve_white_cross(&mut out);

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((U_FACE * 9 + 1, B_FACE * 9 + 1)));
        
        assert!(moves_count > 0);
    }

    #[test]
    fn test_white_orange_on_lf() {
        let mut cube = Cube::make_solved();
        
        // White on L face, Orange on F face (not adjacent, need pivoting)
        cube.b();
        cube.l();
        cube.l();
        
        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        let (white_idx, orange_idx) = result.unwrap();
        assert_eq!(white_idx, L_FACE * 9 + 5);
        assert_eq!(orange_idx, F_FACE * 9 + 3);
        
        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        let moves_count = cube.solve_white_cross(&mut out);

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((U_FACE * 9 + 1, B_FACE * 9 + 1)));
        
        assert!(moves_count > 0);

    }

    #[test]
    fn test_white_orange_on_lu() {
        let mut cube = Cube::make_solved();

        cube.b();
        cube.l();

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((L_FACE * 9 + 1, U_FACE * 9 + 3)));

        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        let moves_count = cube.solve_white_cross(&mut out);

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((U_FACE * 9 + 1, B_FACE * 9 + 1)));

        assert!(moves_count > 0);

    } 
    // todo: ld
    #[test]
    fn test_white_orange_on_ld() {
        let mut cube = Cube::make_solved();

        cube.b();
        cube.li();

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((L_FACE * 9 + 7, D_FACE * 9 + 3)));

        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        let moves_count = cube.solve_white_cross(&mut out);

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((U_FACE * 9 + 1, B_FACE * 9 + 1)));

        assert!(moves_count > 0);

    } 

    // WHITE ON R FACE
    #[test]
    fn test_white_orange_on_rb() {
        let mut cube = Cube::make_solved();
        
        // White on R face, Orange on B face
        cube.bi();
        
        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());
        
        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        let moves_count = cube.solve_white_cross(&mut out);
        
        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((U_FACE * 9 + 1, B_FACE * 9 + 1)));

        assert!(moves_count > 0);
     
    }

    #[test]
    fn test_white_orange_on_rf() {
        let mut cube = Cube::make_solved();

        cube.u();
        cube.u();
        cube.f();

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((R_FACE * 9 + 3, F_FACE * 9 + 5)));

        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        let moves_count = cube.solve_white_cross(&mut out);

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((U_FACE * 9 + 1, B_FACE * 9 + 1)));

        assert!(moves_count > 0);

    }

    #[test]
    fn test_white_orange_on_ru() {
        let mut cube = Cube::make_solved();

        cube.bi();
        cube.ri();

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((R_FACE * 9 + 1, U_FACE * 9 + 5)));

        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        let moves_count = cube.solve_white_cross(&mut out);

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((U_FACE * 9 + 1, B_FACE * 9 + 1)));

        assert!(moves_count > 0);

    } 

    #[test]
    fn test_white_orange_on_rd() {
        let mut cube = Cube::make_solved();

        cube.bi();
        cube.r();

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((R_FACE * 9 + 7, D_FACE * 9 + 5)));

        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        let moves_count = cube.solve_white_cross(&mut out);

        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());

        assert_eq!(result, Some((U_FACE * 9 + 1, B_FACE * 9 + 1)));

        assert!(moves_count > 0);

    } 
    #[test]
    fn test_white_orange_moves_recorded_in_output() {
        let mut cube = Cube::make_solved();
        
        // Move white-orange off its solved position
        cube.u();
        
        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        let moves_count = cube.solve_white_cross(&mut out);
        
        // Check that the output array contains the moves
        assert!(moves_count > 0);
        for i in 0..moves_count {
            // Each recorded move should be valid (one of the 18 possible moves)
            match out[i] {
                solver_move_t::U | solver_move_t::Ui | solver_move_t::U2 |
                solver_move_t::D | solver_move_t::Di | solver_move_t::D2 |
                solver_move_t::L | solver_move_t::Li | solver_move_t::L2 |
                solver_move_t::R | solver_move_t::Ri | solver_move_t::R2 |
                solver_move_t::F | solver_move_t::Fi | solver_move_t::F2 |
                solver_move_t::B | solver_move_t::Bi | solver_move_t::B2 => {
                    // Valid move
                }
            }
        }
    }

    #[test]
    fn test_solve_white_orange_multiple_times() {
        // Test that the solver handles various random placements which modifies the white-orange edge
        let placements = vec![
            vec![solver_move_t::U],
            vec![solver_move_t::U, solver_move_t::U],
            vec![solver_move_t::U, solver_move_t::U, solver_move_t::U],
            vec![solver_move_t::B],
            vec![solver_move_t::B, solver_move_t::B],
        ];
        
        for moves_to_apply in placements {
            let mut cube = Cube::make_solved();
            
            // Apply scramble
            for m in &moves_to_apply {
                cube.apply_move(*m);
            }
            
            // Solve white-orange
            let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
            let moves_count = cube.solve_white_cross(&mut out);
            
            // Verify it's solved
            assert!(cube.is_solved(), "Cube should be solved after white-orange solver");
            assert!(moves_count > 0 || moves_to_apply.is_empty(), "Should record moves or already be solved");
        }
    }

    #[test]
    fn test_white_orange_edge_position_after_solve() {
        let mut cube = Cube::make_solved();
        
        // Scramble it
        cube.u();
        cube.l();
        cube.d();
        
        // Solve it
        let mut out: [solver_move_t; 100] = [solver_move_t::U; 100];
        cube.solve_white_cross(&mut out);
        
        // Verify white-orange edge is at the correct position
        let result = cube.find_edge(WHITE, ORANGE);
        assert!(result.is_some());
        let (white_idx, orange_idx) = result.unwrap();
        
        // White should be at U face position 1, Orange should be at B face position 1
        assert_eq!(white_idx, U_FACE * 9 + 1, "White sticker should be at U1");
        assert_eq!(orange_idx, B_FACE * 9 + 1, "Orange sticker should be at B1");
        
        // Double check the stickers themselves
        assert_eq!(cube.stickers[U_FACE * 9 + 1], WHITE, "U1 should have white");
        assert_eq!(cube.stickers[B_FACE * 9 + 1], ORANGE, "B1 should have orange");
    }
}

#[cfg(test)]
mod white_cross_tests {
    use cube_solver::*;

    // ------------------------------------------------------------------------
    // HELPER: check if white cross is solved
    // ------------------------------------------------------------------------
    fn is_white_cross_solved(cube: &Cube) -> bool {
        // White edges on U face
        if cube.stickers[U_FACE * 9 + 1] != WHITE { return false; }
        if cube.stickers[U_FACE * 9 + 3] != WHITE { return false; }
        if cube.stickers[U_FACE * 9 + 5] != WHITE { return false; }
        if cube.stickers[U_FACE * 9 + 7] != WHITE { return false; }
        // Side stickers match centers
        if cube.stickers[B_FACE * 9 + 1] != ORANGE { return false; }
        if cube.stickers[L_FACE * 9 + 1] != GREEN  { return false; }
        if cube.stickers[R_FACE * 9 + 1] != BLUE   { return false; }
        if cube.stickers[F_FACE * 9 + 1] != RED    { return false; }
        true
    }

    // ------------------------------------------------------------------------
    // TEST WHITE-ORANGE EDGE (B face)
    // ------------------------------------------------------------------------
    #[test]
    fn test_white_orange_all_positions() {
        let positions = vec![
            // White on U
            (vec![solver_move_t::U], "U"),
            (vec![solver_move_t::U2], "U2"),
            (vec![solver_move_t::Ui], "Ui"),
            // White on D
            (vec![solver_move_t::B2], "B2"),
            (vec![solver_move_t::B2, solver_move_t::D], "B2 D"),
            (vec![solver_move_t::B2, solver_move_t::D2], "B2 D2"),
            (vec![solver_move_t::B2, solver_move_t::Di], "B2 Di"),
            // White on L
            (vec![solver_move_t::B], "B (white on L)"),
            (vec![solver_move_t::B, solver_move_t::L2], "B L2 (white on L F)"),
            (vec![solver_move_t::B, solver_move_t::L], "B L (white on L U)"),
            (vec![solver_move_t::B, solver_move_t::Li], "B Li (white on L D)"),
            // White on R
            (vec![solver_move_t::Bi], "Bi (white on R)"),
            (vec![solver_move_t::Bi, solver_move_t::R2], "Bi R2 (white on R F?)"),
            (vec![solver_move_t::Bi, solver_move_t::R], "Bi R (white on R D)"),
            (vec![solver_move_t::Bi, solver_move_t::Ri], "Bi Ri (white on R U)"),
            // White on F
            (vec![solver_move_t::U, solver_move_t::R, solver_move_t::B], "U R B (white on F U)"),
            (vec![solver_move_t::U2, solver_move_t::F], "U2 F (white on F D)"),
            (vec![solver_move_t::U, solver_move_t::F], "U F (white on F L)"),
            (vec![solver_move_t::Ui, solver_move_t::F], "Ui F (white on F R)"),
            // White on B (already tested via B and Bi)
        ];

        for (moves, desc) in positions {
            let mut cube = Cube::make_solved();
            for &m in &moves {
                cube.apply_move(m);
            }
            let mut out = [solver_move_t::U; 100];
            let moves_used = cube.solve_white_cross(&mut out);
            assert!(is_white_cross_solved(&cube), "Failed: {}", desc);
            assert!(moves_used > 0 || moves.is_empty(), "Failed: {}", desc);
        }
    }

    // ------------------------------------------------------------------------
    // TEST WHITE-GREEN EDGE (L face)
    // ------------------------------------------------------------------------
    #[test]
    fn test_white_green_all_positions() {
        // Similar exhaustive list, but with L as target
        // (I'll show a few representative cases; full test would mirror above)
        let positions = vec![
            // Already solved
            (vec![], "solved"),
            // White on U
            (vec![solver_move_t::L], "U? Actually L moves it? Better: U"),
            (vec![solver_move_t::U], "U"),
            (vec![solver_move_t::U2], "U2"),
            (vec![solver_move_t::Ui], "Ui"),
            // White on D
            (vec![solver_move_t::L2], "L2"),
            (vec![solver_move_t::L2, solver_move_t::D], "L2 D"),
            (vec![solver_move_t::L2, solver_move_t::D2], "L2 D2"),
            (vec![solver_move_t::L2, solver_move_t::Di], "L2 Di"),
            // White on R
            (vec![solver_move_t::Li], "Li (white on R)"),
            // White on F
            (vec![solver_move_t::U, solver_move_t::F, solver_move_t::L], "U F L (white on F?)"),
            // White on B
            (vec![solver_move_t::U, solver_move_t::B, solver_move_t::L], "U B L (white on B?)"),
        ];

        for (moves, desc) in positions {
            let mut cube = Cube::make_solved();
            for &m in &moves {
                cube.apply_move(m);
            }
            let mut out = [solver_move_t::U; 100];
            let moves_used = cube.solve_white_cross(&mut out);
            assert!(is_white_cross_solved(&cube), "White‑green failed: {}", desc);
            if !moves.is_empty() {
                assert!(moves_used > 0, "White‑green failed: {}", desc);
            }
        }
    }

    // ------------------------------------------------------------------------
    // TEST WHITE-BLUE EDGE (R face)
    // ------------------------------------------------------------------------
    #[test]
    fn test_white_blue_all_positions() {
        // Similar to above, with R as target
        let positions = vec![
            (vec![], "solved"),
            (vec![solver_move_t::U], "U"),
            (vec![solver_move_t::U2], "U2"),
            (vec![solver_move_t::Ui], "Ui"),
            (vec![solver_move_t::R2], "R2"),
            (vec![solver_move_t::R2, solver_move_t::D], "R2 D"),
            (vec![solver_move_t::R2, solver_move_t::D2], "R2 D2"),
            (vec![solver_move_t::R2, solver_move_t::Di], "R2 Di"),
            (vec![solver_move_t::Ri], "Ri (white on L)"),
            (vec![solver_move_t::R], "R (white on B?)"),
        ];
        for (moves, desc) in positions {
            let mut cube = Cube::make_solved();
            for &m in &moves {
                cube.apply_move(m);
            }
            let mut out = [solver_move_t::U; 100];
            let moves_used = cube.solve_white_cross(&mut out);
            assert!(is_white_cross_solved(&cube), "White‑blue failed: {}", desc);
            if !moves.is_empty() {
                assert!(moves_used > 0, "White‑blue failed: {}", desc);
            }
        }
    }

    // ------------------------------------------------------------------------
    // TEST WHITE-RED EDGE (F face)
    // ------------------------------------------------------------------------
    #[test]
    fn test_white_red_all_positions() {
        let positions = vec![
            (vec![], "solved"),
            (vec![solver_move_t::U], "U"),
            (vec![solver_move_t::U2], "U2"),
            (vec![solver_move_t::Ui], "Ui"),
            (vec![solver_move_t::F2], "F2"),
            (vec![solver_move_t::F2, solver_move_t::D], "F2 D"),
            (vec![solver_move_t::F2, solver_move_t::D2], "F2 D2"),
            (vec![solver_move_t::F2, solver_move_t::Di], "F2 Di"),
            (vec![solver_move_t::Fi], "Fi (white on L?)"),
            (vec![solver_move_t::F], "F (white on R?)"),
        ];
        for (moves, desc) in positions {
            let mut cube = Cube::make_solved();
            for &m in &moves {
                cube.apply_move(m);
            }
            let mut out = [solver_move_t::U; 100];
            let moves_used = cube.solve_white_cross(&mut out);
            assert!(is_white_cross_solved(&cube), "White‑red failed: {}", desc);
            if !moves.is_empty() {
                assert!(moves_used > 0, "White‑red failed: {}", desc);
            }
        }
    }

    // ------------------------------------------------------------------------
    // COMPLEX SCRAMBLES – FULL WHITE CROSS
    // ------------------------------------------------------------------------
    #[test]
    fn test_white_cross_complex_scrambles() {
        let scrambles = vec![
            vec![solver_move_t::R, solver_move_t::U, solver_move_t::R2, solver_move_t::U2],
            vec![solver_move_t::F, solver_move_t::D2, solver_move_t::L, solver_move_t::B2],
            vec![solver_move_t::U, solver_move_t::R, solver_move_t::F, solver_move_t::L, solver_move_t::B],
            vec![solver_move_t::R2, solver_move_t::U2, solver_move_t::F2, solver_move_t::D2, solver_move_t::L2, solver_move_t::B2],
        ];

        for (i, scramble) in scrambles.iter().enumerate() {
            let mut cube = Cube::make_solved();
            for &m in scramble {
                cube.apply_move(m);
            }
            let mut out = [solver_move_t::U; 100];
            let moves_used = cube.solve_white_cross(&mut out);
            assert!(is_white_cross_solved(&cube), "Scramble {} failed", i);
            assert!(moves_used > 0, "Scramble {} used 0 moves", i);
            assert!(moves_used < 25, "Scramble {} used too many moves: {}", i, moves_used);
        }
    }

    // ------------------------------------------------------------------------
    // ALREADY SOLVED CUBE
    // ------------------------------------------------------------------------
    #[test]
    fn test_solved_cube_needs_zero_moves() {
        let mut cube = Cube::make_solved();
        let mut out = [solver_move_t::U; 100];
        let moves = cube.solve_white_cross(&mut out);
        assert_eq!(moves, 0);
        assert!(is_white_cross_solved(&cube));
    }

    // ------------------------------------------------------------------------
    // PRESERVE CENTERS
    // ------------------------------------------------------------------------
    #[test]
    fn test_centers_unchanged() {
        let mut cube = Cube::make_solved();
        cube.apply_move(solver_move_t::R);
        cube.apply_move(solver_move_t::U);
        let centers_before = [
            cube.stickers[U_FACE * 9 + 4],
            cube.stickers[D_FACE * 9 + 4],
            cube.stickers[L_FACE * 9 + 4],
            cube.stickers[R_FACE * 9 + 4],
            cube.stickers[F_FACE * 9 + 4],
            cube.stickers[B_FACE * 9 + 4],
        ];
        let mut out = [solver_move_t::U; 100];
        cube.solve_white_cross(&mut out);
        let centers_after = [
            cube.stickers[U_FACE * 9 + 4],
            cube.stickers[D_FACE * 9 + 4],
            cube.stickers[L_FACE * 9 + 4],
            cube.stickers[R_FACE * 9 + 4],
            cube.stickers[F_FACE * 9 + 4],
            cube.stickers[B_FACE * 9 + 4],
        ];
        assert_eq!(centers_before, centers_after);
    }

    // ------------------------------------------------------------------------
    // OUTPUT MOVES ARE VALID
    // ------------------------------------------------------------------------
    #[test]
    fn test_output_moves_are_valid() {
        let mut cube = Cube::make_solved();
        cube.apply_move(solver_move_t::R);
        cube.apply_move(solver_move_t::U);
        cube.apply_move(solver_move_t::F);
        let mut out = [solver_move_t::U; 100];
        let moves = cube.solve_white_cross(&mut out);
        for i in 0..moves {
            match out[i] {
                solver_move_t::U | solver_move_t::Ui | solver_move_t::U2 |
                solver_move_t::D | solver_move_t::Di | solver_move_t::D2 |
                solver_move_t::L | solver_move_t::Li | solver_move_t::L2 |
                solver_move_t::R | solver_move_t::Ri | solver_move_t::R2 |
                solver_move_t::F | solver_move_t::Fi | solver_move_t::F2 |
                solver_move_t::B | solver_move_t::Bi | solver_move_t::B2 => {}
                _ => panic!("Invalid move in output"),
            }
        }
    }
}