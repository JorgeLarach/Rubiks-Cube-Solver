use cube_solver::*;
use std::collections::VecDeque;
fn main() {
    let table = bfs_corner_orient();
    print!("pub const CORNER_ORIENT_TABLE: [u8; 2187] = [");
    for (i, &v) in table.iter().enumerate() {
        print!("{}, ", v);
    }
    println!("];");
}

fn bfs_corner_orient() -> [u8; 2187] {
    let mut table = [u8::MAX; 2187];
    let mut queue:VecDeque<(CubieState, u8)> = VecDeque::new();

    let solved = CubieState::make_solved();
    let start_coord = CubieState::corner_orient_coord(&solved);
    table[start_coord] = 0;
    queue.push_back((solved, 0u8));

    while let Some((cube, dist)) = queue.pop_front() {
        for &m in ALL_MOVES.iter() {
            let mut next = cube;
            next.apply_move(m);
            let coord = CubieState::corner_orient_coord(&next);
            if table[coord] == u8::MAX { // unvisited
                table[coord] = dist + 1;
                queue.push_back((next, dist + 1));
            }
        }
    }
    table
}


