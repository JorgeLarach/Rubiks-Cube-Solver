mod tests {
    use cube_solver::*;

    #[test]
    fn test_solved_cube_is_solved() {
        let cube: Cube = Cube::make_solved();
        assert!(cube.is_solved());
    }

    #[test]
    fn test_u() {
        let mut cube = Cube::make_solved();
        cube.apply_move(solver_move_t::U);
        assert!(!cube.is_solved());
        cube.apply_move(solver_move_t::Ui);
        assert!(cube.is_solved());
    }

    #[test]
    fn test_d() {
        let mut cube = Cube::make_solved();
        cube.apply_move(solver_move_t::D);
        assert!(!cube.is_solved());
        cube.apply_move(solver_move_t::Di);
        assert!(cube.is_solved());
    }

    #[test]
    fn test_l() {
        let mut cube = Cube::make_solved();
        cube.apply_move(solver_move_t::L);
        assert!(!cube.is_solved());
        cube.apply_move(solver_move_t::Li);
        assert!(cube.is_solved());
    }

    #[test]
    fn test_r() {
        let mut cube = Cube::make_solved();
        cube.apply_move(solver_move_t::R);
        assert!(!cube.is_solved());
        cube.apply_move(solver_move_t::Ri);
        assert!(cube.is_solved());
    }

    #[test]
    fn test_f() {
        let mut cube = Cube::make_solved();
        cube.apply_move(solver_move_t::F);
        assert!(!cube.is_solved());
        cube.apply_move(solver_move_t::Fi);
        assert!(cube.is_solved());
    }

    #[test]
    fn test_b() {
        let mut cube = Cube::make_solved();
        cube.apply_move(solver_move_t::B);
        assert!(!cube.is_solved());
        cube.apply_move(solver_move_t::Bi);
        assert!(cube.is_solved());
    }

    #[test]
    fn test_all_moves_return_to_solved() {
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
            for _ in 0..4 { move_fn(&mut cube); }
            assert!(cube.is_solved(), "{} failed", name);
        }
    }

    #[test]
    fn test_move_and_inverse() {
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
            assert!(cube.is_solved());
        }
    }

    #[test]
    fn test_double_moves() {
        let mut c1 = Cube::make_solved();
        let mut c2 = Cube::make_solved();
        c1.apply_move(solver_move_t::U2);
        c2.u();
        c2.u();
        assert_eq!(c1.stickers, c2.stickers);
    }

    #[test]
    fn test_find_edge_on_solved_cube() {
        let cube = Cube::make_solved();
        assert_eq!(cube.find_edge(WHITE, ORANGE), Some((0,0)));
        assert_eq!(cube.find_edge(WHITE, GREEN), Some((1,0)));
        assert_eq!(cube.find_edge(WHITE, BLUE), Some((2,0)));
        assert_eq!(cube.find_edge(WHITE, RED), Some((3,0)));
    }

    #[test]
    fn test_find_corner_on_solved_cube() {
        let cube = Cube::make_solved();
        assert_eq!(cube.find_corner(WHITE, GREEN, ORANGE), Some((0, 0)));
        assert_eq!(cube.find_corner(WHITE, BLUE, ORANGE), Some((1, 0)));
        assert_eq!(cube.find_corner(WHITE, GREEN, RED), Some((2, 0)));
        assert_eq!(cube.find_corner(WHITE, BLUE, RED), Some((3, 0)));
    }

    #[test]
    fn test_find_corner_rotated() {
        let mut cube = Cube::make_solved();
        cube.r();
        cube.u();
        cube.apply_move(solver_move_t::Ri);
        let result = cube.find_corner(WHITE, BLUE, RED);
        assert!(result.is_some(), "Should find rotated corner");
    }

    #[test]
    fn test_solve_already_solved() {
        let cube = Cube::make_solved();
        let mut moves = [solver_move_t::U; 100];
        let count = solve_cube_wrapper(&cube, &mut moves);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_solve_simple_single_move() {
        let mut cube = Cube::make_solved();
        cube.u();
        let mut moves = [solver_move_t::U; 100];
        let count = solve_cube_wrapper(&cube, &mut moves);
        let mut test_cube = cube;
        for i in 0..count {
            test_cube.apply_move(moves[i]);
        }
        assert!(test_cube.is_solved(), "Single move scramble should solve");
    }

    #[test]
    fn test_solve_two_moves() {
        let mut cube = Cube::make_solved();
        cube.u();
        cube.r();
        let mut moves = [solver_move_t::U; 100];
        let count = solve_cube_wrapper(&cube, &mut moves);
        let mut test_cube = cube;
        for i in 0..count {
            test_cube.apply_move(moves[i]);
        }
        assert!(test_cube.is_solved(), "Two move scramble should solve");
    }

    #[test]
    fn test_solve_three_moves() {
        let mut cube = Cube::make_solved();
        cube.r();
        cube.u();
        cube.f();
        let mut moves = [solver_move_t::U; 150];
        let count = solve_cube_wrapper(&cube, &mut moves);
        let mut test_cube = cube;
        for i in 0..count {
            test_cube.apply_move(moves[i]);
        }
        assert!(test_cube.is_solved(), "Three move scramble should solve");
    }

    #[test]
    fn test_solve_five_moves() {
        let mut cube = Cube::make_solved();
        cube.r();
        cube.u();
        cube.r();
        cube.u();
        cube.ri();
        let mut moves = [solver_move_t::U; 200];
        let count = solve_cube_wrapper(&cube, &mut moves);
        let mut test_cube = cube;
        for i in 0..count {
            test_cube.apply_move(moves[i]);
        }
        assert!(test_cube.is_solved(), "Five move scramble should solve");
    }

    #[test]
    fn test_solve_complex_scramble() {
        let mut cube = Cube::make_solved();
        cube.apply_move(solver_move_t::R);
        cube.apply_move(solver_move_t::U);
        cube.apply_move(solver_move_t::R2);
        cube.apply_move(solver_move_t::D);
        cube.apply_move(solver_move_t::L);
        cube.apply_move(solver_move_t::F2);
        cube.apply_move(solver_move_t::B);
        let mut moves = [solver_move_t::U; 200];
        let count = solve_cube_wrapper(&cube, &mut moves);
        let mut test_cube = cube;
        for i in 0..count {
            test_cube.apply_move(moves[i]);
        }
        assert!(test_cube.is_solved());
    }

    #[test]
    fn test_solve_heavy_scramble() {
        let mut cube = Cube::make_solved();
        let scramble = [
            solver_move_t::R, solver_move_t::U, solver_move_t::R, solver_move_t::U,
            solver_move_t::R, solver_move_t::U, solver_move_t::Ri, solver_move_t::Ui,
            solver_move_t::Ri, solver_move_t::Ui, solver_move_t::F2, solver_move_t::D,
            solver_move_t::L2, solver_move_t::B, solver_move_t::U2,
        ];
        for &m in &scramble {
            cube.apply_move(m);
        }
        let mut moves = [solver_move_t::U; 250];
        let count = solve_cube_wrapper(&cube, &mut moves);
        let mut test_cube = cube;
        for i in 0..count {
            test_cube.apply_move(moves[i]);
        }
        assert!(test_cube.is_solved(), "Heavy scramble should solve");
    }

    #[test]
    fn test_solve_move_count_reasonable() {
        let mut cube = Cube::make_solved();
        cube.r();
        cube.u();
        let mut moves = [solver_move_t::U; 150];
        let count = solve_cube_wrapper(&cube, &mut moves);
        assert!(count < 150, "Solution should be reasonable");
    }

    #[test]
    fn test_solve_doesnt_exceed_max_moves() {
        let mut cube = Cube::make_solved();
        cube.apply_move(solver_move_t::R);
        cube.apply_move(solver_move_t::U);
        cube.apply_move(solver_move_t::R2);
        cube.apply_move(solver_move_t::D);
        cube.apply_move(solver_move_t::L);
        let mut moves = [solver_move_t::U; 50];
        let count = solve_cube_wrapper(&cube, &mut moves);
        assert!(count <= 50);
    }

    fn solve_cube_wrapper(cube: &Cube, moves: &mut [solver_move_t]) -> usize {
        unsafe {
            let cube_bytes = &cube.stickers as *const [u8; 54] as *const u8;
            let moves_ptr = moves.as_mut_ptr();
            cube_solver::solve_cube(cube_bytes, moves_ptr, moves.len())
        }
    }

    fn ri(cube: &mut Cube) {
        cube.r();
        cube.r();
        cube.r();
    }
}
