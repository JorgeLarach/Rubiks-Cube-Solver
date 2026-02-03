use cube_solver::*;

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


#[cfg(test)]
mod tests {
    use super::*;
    
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
}