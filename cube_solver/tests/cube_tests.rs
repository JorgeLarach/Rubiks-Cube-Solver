use cube_solver::*;

#[test]
fn test_solved_cube_is_solved() {
    let cube = Cube::make_solved();
    assert!(cube.is_solved());
}

#[test]
fn debug_solved_cube() {
    let cube = Cube::make_solved();
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