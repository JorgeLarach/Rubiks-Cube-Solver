//
// gen_tables.rs
//
//  Created on: April 25, 2026
//      Author: jorgelarach
//   Co-Author: Claude
//

// Claude was a significant contributor to this.
// This program generates all lookup tables used by the solver
// Generates the tables based on a solved cube, and uses BFS to traverse all states
// Depth of BFS corresponds to moves from current state to root (solved state)
// Rewritten solver basically because two different CubieStates can produce the same udslice_perm_coord while having completely different arrangements
use cube_solver::*;
use std::collections::VecDeque;
fn main() {
    use std::time::Instant;

    // ---- Phase 1 move tables (internal flash) ----
    println!("--- Phase 1 move tables ---");
    let t = Instant::now();
    let co_move = gen_co_move_table();
    let eo_move = gen_eo_move_table();
    let ud_move = gen_ud_move_table();
    println!("Phase 1 move tables: {}ms", t.elapsed().as_millis());

    // Print as Rust source — these are small enough
    print_move_table_u16("CO_MOVE_TABLE", 2187, &co_move);
    print_move_table_u16("EO_MOVE_TABLE", 2048, &eo_move);
    print_move_table_u16("UD_MOVE_TABLE", 495,  &ud_move);

    // ---- Phase 2 move tables (QSPI flash) ----
    println!("--- Phase 2 move tables ---");
    let t = Instant::now();
    let cp_move = gen_cp_move_table();
    let ep_move = gen_ep_move_table();
    let sp_move = gen_sp_move_table();
    println!("Phase 2 move tables: {}ms", t.elapsed().as_millis());

    write_u16_table_10("cp_move.bin", &cp_move);
    write_u16_table_10("ep_move.bin", &ep_move);
    print_sp_table(&sp_move); // tiny — paste directly into tables.rs

    // ---- Combined pruning tables (QSPI flash) ----
    // Generated AFTER move tables because they use them directly.
    // This avoids CubieState BFS entirely for the combined tables —
    // the move tables give us direct coord transitions.
    println!("--- Combined pruning tables ---");

    let t = Instant::now();
    let flip_ud = gen_flip_udslice_table(&eo_move, &ud_move);
    println!("FLIP_UDSLICE: {}ms", t.elapsed().as_millis());
    write_u8_slice("flip_udslice.bin", &flip_ud);

    let t = Instant::now();
    let corners_s2 = gen_corners_slice2_table(&cp_move, &sp_move);
    println!("CORNERS_SLICE2: {}ms", t.elapsed().as_millis());
    write_u8_slice("corners_slice2.bin", &corners_s2);

    let t = Instant::now();
    let edges_s2 = gen_edges_slice2_table(&ep_move, &sp_move);
    println!("EDGES_SLICE2: {}ms", t.elapsed().as_millis());
    write_u8_slice("edges_slice2.bin", &edges_s2);

    // ---- Sanity summary ----
    println!("--- File sizes ---");
    println!("cp_move.bin:        {} bytes", 40320 * 10 * 2);
    println!("ep_move.bin:        {} bytes", 40320 * 10 * 2);
    println!("flip_udslice.bin:   {} bytes", 2048 * 495);
    println!("corners_slice2.bin: {} bytes", 40320 * 24);
    println!("edges_slice2.bin:   {} bytes", 40320 * 24);
    println!("Total QSPI:         {} bytes",
        40320*10*2 + 40320*10*2 + 2048*495 + 40320*24 + 40320*24);
}


// -------------------------------------------------------
// Binary file writers
// All large tables written as binary, not Rust source.
// Format is always little-endian, row-major.
// -------------------------------------------------------

fn write_u16_table(filename: &str, table: &[[u16; 18]]) {
    use std::io::Write;
    let mut f = std::fs::File::create(filename).unwrap();
    for row in table {
        for &v in row { f.write_all(&v.to_le_bytes()).unwrap(); }
    }
    println!("Written {} bytes -> {}", table.len() * 18 * 2, filename);
}

fn write_u16_table_10(filename: &str, table: &[[u16; 10]]) {
    use std::io::Write;
    let mut f = std::fs::File::create(filename).unwrap();
    for row in table {
        for &v in row { f.write_all(&v.to_le_bytes()).unwrap(); }
    }
    println!("Written {} bytes -> {}", table.len() * 10 * 2, filename);
}

fn write_u8_slice(filename: &str, table: &[u8]) {
    use std::io::Write;
    let mut f = std::fs::File::create(filename).unwrap();
    f.write_all(table).unwrap();
    println!("Written {} bytes -> {}", table.len(), filename);
}

// SP table is tiny — print it directly as Rust source
fn print_sp_table(table: &[[u8; 10]]) {
    println!("pub const SP_MOVE_TABLE: [[u8; 10]; 24] = [");
    for row in table {
        print!("    [");
        for (i, &v) in row.iter().enumerate() {
            if i < 9 { print!("{}, ", v); } else { print!("{}", v); }
        }
        println!("],");
    }
    println!("];");
}

// PHASE 1 OG Table Gen

// fn generate_corner_orientation_table() {
//     let table = bfs_corner_orient();
//     print!("pub const CORNER_ORIENT_TABLE: [u8; 2187] = [");
//     for (_, &v) in table.iter().enumerate() {
//         print!("{}, ", v);
//     }
//     println!("];");
// }

// ============================================================
// PHASE 1 MOVE TABLE GENERATION
// ============================================================
//
// A move table answers: given coordinate value c and move m,
// what coordinate value results after applying m?
//
// move_table[c][m] = new_coord
//
// This lets the search update state with ONE array lookup instead
// of: copy CubieState -> permute cubies -> run Lehmer/Horner encoding.
//
// HOW GENERATION WORKS:
// For each coordinate value c in 0..N:
//   1. Find a CubieState whose coordinate equals c
//      (done by BFS from solved — same approach as pruning tables)
//   2. For each move m:
//      apply m to that CubieState
//      compute the resulting coordinate
//      store it in table[c][m]
//
// The CubieState used as representative for coord c doesn't matter —
// any state with that coordinate gives the same transition, because
// the coordinate is slot-based and fully determines the transition.
//
// WHY u16:
// CO coords go up to 2186, EO up to 2047, UD up to 494.
// All exceed u8::MAX (255), so u16 is required.
// ============================================================

// -------------------------------------------------------
// HELPER: BFS to collect one representative CubieState
// per coordinate value. Used by all three move table
// generators below. Avoids repeating the same BFS logic
// three times.
//
// coord_fn: a closure that extracts the coordinate from a CubieState
// size:     the number of distinct coordinate values (2187, 2048, 495)
//
// Returns a Vec where vec[c] = Some(CubieState with coord == c)
// Every entry will be Some(...) — if any are None after BFS,
// the BFS or coord function has a bug.
// -------------------------------------------------------
fn collect_representatives(
    size: usize,
    coord_fn: impl Fn(&CubieState) -> usize,
) -> Vec<CubieState> {
    // None means "not yet found a CubieState for this coord"
    let mut reps: Vec<Option<CubieState>> = vec![None; size];
    let mut queue: VecDeque<CubieState> = VecDeque::new();

    let solved = CubieState::make_solved();
    let start_coord = coord_fn(&solved);
    reps[start_coord] = Some(solved);
    queue.push_back(solved);

    // BFS using all 18 moves to reach every reachable coordinate value.
    // We use ALL 18 moves here even for Phase 2 coords because we need
    // to reach all 40320 corner permutations — Phase 2's restricted move
    // set can't reach all of them from the solved state.
    while let Some(state) = queue.pop_front() {
        for &m in ALL_MOVES.iter() {
            let mut next = state;
            next.apply_move(m);
            let coord = coord_fn(&next);
            if reps[coord].is_none() {
                reps[coord] = Some(next);
                queue.push_back(next);
            }
        }
    }

    // Unwrap all — every coord must have been reached.
    // If any are None, panic with a clear message.
    reps.into_iter()
        .enumerate()
        .map(|(c, opt)| opt.unwrap_or_else(|| {
            panic!("collect_representatives: coord {} never reached — \
                    check coord function or BFS move set", c)
        }))
        .collect()
}

// -------------------------------------------------------
// CO MOVE TABLE
// Shape:  [2187][18]  (2187 CO coords × 18 moves)
// Type:   u16 (CO coords go up to 2186, exceeds u8)
// Size:   2187 × 18 × 2 bytes = 78,732 bytes ≈ 77KB
// Lives:  internal flash
//
// co_move[c][mi] = corner orient coord after applying
//                  ALL_MOVES[mi] to a cube with CO coord c
// -------------------------------------------------------
fn gen_co_move_table() -> Vec<[u16; 18]> {
    // Step 1: find one representative CubieState per CO coord
    let reps = collect_representatives(2187, |s| s.corner_orient_coord());

    // Step 2: for each coord, apply all 18 moves, record new coord
    let mut table = vec![[0u16; 18]; 2187];
    for (c, rep) in reps.iter().enumerate() {
        for (mi, &m) in ALL_MOVES.iter().enumerate() {
            let mut next = *rep;
            next.apply_move(m);
            // The new CO coord is what ends up in the table
            table[c][mi] = next.corner_orient_coord() as u16;
        }
    }

    // Sanity check: applying any move from coord 0 (solved) must
    // produce a valid coord in range, and applying a move then its
    // inverse must return to the original coord
    for (mi, &m) in ALL_MOVES.iter().enumerate() {
        let new_coord = table[0][mi];
        assert!((new_coord as usize) < 2187,
            "CO move table: move {:?} from coord 0 produced out-of-range coord {}",
            m, new_coord);
    }

    println!("// CO_MOVE_TABLE generated: {} entries", 2187 * 18);
    table
}

// -------------------------------------------------------
// EO MOVE TABLE
// Shape:  [2048][18]  (2048 EO coords × 18 moves)
// Type:   u16 (EO coords go up to 2047, exceeds u8)
// Size:   2048 × 18 × 2 bytes = 73,728 bytes ≈ 72KB
// Lives:  internal flash
//
// eo_move[c][mi] = edge orient coord after applying
//                  ALL_MOVES[mi] to a cube with EO coord c
// -------------------------------------------------------
fn gen_eo_move_table() -> Vec<[u16; 18]> {
    let reps = collect_representatives(2048, |s| s.edge_orient_coord());

    let mut table = vec![[0u16; 18]; 2048];
    for (c, rep) in reps.iter().enumerate() {
        for (mi, &m) in ALL_MOVES.iter().enumerate() {
            let mut next = *rep;
            next.apply_move(m);
            table[c][mi] = next.edge_orient_coord() as u16;
        }
    }

    for (mi, &m) in ALL_MOVES.iter().enumerate() {
        let new_coord = table[0][mi];
        assert!((new_coord as usize) < 2048,
            "EO move table: move {:?} from coord 0 produced out-of-range coord {}",
            m, new_coord);
    }

    println!("// EO_MOVE_TABLE generated: {} entries", 2048 * 18);
    table
}

// -------------------------------------------------------
// UD MOVE TABLE
// Shape:  [495][18]  (495 UD-slice coords × 18 moves)
// Type:   u16 (UD coords go up to 494, fits in u16 cleanly)
// Size:   495 × 18 × 2 bytes = 17,820 bytes ≈ 17KB
// Lives:  internal flash
//
// ud_move[c][mi] = udslice coord after applying
//                  ALL_MOVES[mi] to a cube with UD coord c
// -------------------------------------------------------
fn gen_ud_move_table() -> Vec<[u16; 18]> {
    let reps = collect_representatives(495, |s| s.udslice_coord());

    let mut table = vec![[0u16; 18]; 495];
    for (c, rep) in reps.iter().enumerate() {
        for (mi, &m) in ALL_MOVES.iter().enumerate() {
            let mut next = *rep;
            next.apply_move(m);
            table[c][mi] = next.udslice_coord() as u16;
        }
    }

    for (mi, &m) in ALL_MOVES.iter().enumerate() {
        let new_coord = table[0][mi];
        assert!((new_coord as usize) < 495,
            "UD move table: move {:?} from coord 0 produced out-of-range coord {}",
            m, new_coord);
    }

    println!("// UD_MOVE_TABLE generated: {} entries", 495 * 18);
    table
}


fn print_move_table_u16(name: &str, size: usize, table: &[[u16; 18]]) {
    println!("pub const {}: [[u16; 18]; {}] = [", name, size);
    for row in table {
        print!("    [");
        for (i, &val) in row.iter().enumerate() {
            if i < 17 { print!("{}, ", val); }
            else       { print!("{}", val); }
        }
        println!("],");
    }
    println!("];");
}
//----------------------------
// ============================================================
// PHASE 2 MOVE TABLE GENERATION
// ============================================================
//
// Same pattern as Phase 1 move tables but:
//   - Only 10 moves (PHASE2_MOVES) stored per coord
//   - Representatives collected with ALL_MOVES (need all 18
//     to reach every corner/edge permutation from solved)
//   - CP and EP coords are only valid in Phase 2 (G1 cube)
// ============================================================

// -------------------------------------------------------
// CP MOVE TABLE
// Shape:  [40320][10]  (40320 CP coords × 10 Phase 2 moves)
// Type:   u16 (CP coords go up to 40319)
// Size:   40320 × 10 × 2 bytes = 806,400 bytes ≈ 787KB
// Lives:  QSPI flash
//
// cp_move[c][mi] = corner perm coord after applying
//                  PHASE2_MOVES[mi] to a cube with CP coord c
// -------------------------------------------------------
fn gen_cp_move_table() -> Vec<[u16; 10]> {
    // Collect representatives using ALL 18 moves — Phase 2's 10 moves
    // alone cannot reach all 40320 corner permutations from solved
    let reps = collect_representatives(40320, |s| s.corner_perm_coord());

    let mut table = vec![[0u16; 10]; 40320];
    for (c, rep) in reps.iter().enumerate() {
        for (mi, &m) in PHASE2_MOVES.iter().enumerate() {
            let mut next = *rep;
            next.apply_move(m);
            table[c][mi] = next.corner_perm_coord() as u16;
        }
    }

    // Sanity: all output coords must be in valid range
    for row in &table {
        for &val in row.iter() {
            assert!((val as usize) < 40320,
                "CP move table: out-of-range coord {}", val);
        }
    }

    println!("// CP_MOVE_TABLE generated: {} entries", 40320 * 10);
    table
}

// -------------------------------------------------------
// EP MOVE TABLE
// Shape:  [40320][10]
// Type:   u16
// Size:   40320 × 10 × 2 bytes = 806,400 bytes ≈ 787KB
// Lives:  QSPI flash
//
// ep_move[c][mi] = edge perm coord after applying
//                  PHASE2_MOVES[mi] to a cube with EP coord c
//
// NOTE: edge_perm_coord() has a precondition that UD-slice
// edges are in belt slots. The representatives collected here
// may violate that. To handle this, we use a modified collection
// that only BFS through G1-reachable states.
// -------------------------------------------------------
fn gen_ep_move_table() -> Vec<[u16; 10]> {
    // For EP coord, representatives must be G1 states —
    // edge_perm_coord is only meaningful when udslice_coord == 0.
    // So we collect representatives using only Phase 2 moves,
    // starting from solved (which is in G1).
    // Phase 2 moves keep us in G1, so all reached states are valid.
    let mut reps: Vec<Option<CubieState>> = vec![None; 40320];
    let mut queue: VecDeque<CubieState> = VecDeque::new();

    let solved = CubieState::make_solved();
    // Solved cube: ep coord = 0
    reps[solved.edge_perm_coord()] = Some(solved);
    queue.push_back(solved);

    // BFS using Phase 2 moves only — stays in G1
    while let Some(state) = queue.pop_front() {
        for &m in PHASE2_MOVES.iter() {
            let mut next = state;
            next.apply_move(m);
            // Only valid EP coords when in G1
            if next.udslice_coord() != 0 { continue; }
            let coord = next.edge_perm_coord();
            if reps[coord].is_none() {
                reps[coord] = Some(next);
                queue.push_back(next);
            }
        }
    }

    // Verify all states reached
    for (c, rep) in reps.iter().enumerate() {
        assert!(rep.is_some(),
            "EP move table: coord {} never reached — \
             check Phase 2 move set or edge_perm_coord()", c);
    }

    let reps: Vec<CubieState> = reps.into_iter().map(|r| r.unwrap()).collect();

    let mut table = vec![[0u16; 10]; 40320];
    for (c, rep) in reps.iter().enumerate() {
        for (mi, &m) in PHASE2_MOVES.iter().enumerate() {
            let mut next = *rep;
            next.apply_move(m);
            table[c][mi] = next.edge_perm_coord() as u16;
        }
    }

    for row in &table {
        for &val in row.iter() {
            assert!((val as usize) < 40320,
                "EP move table: out-of-range coord {}", val);
        }
    }

    println!("// EP_MOVE_TABLE generated: {} entries", 40320 * 10);
    table
}

// -------------------------------------------------------
// SP MOVE TABLE  (SP = udslice perm)
// Shape:  [24][10]
// Type:   u8 (SP coords go up to 23, fits in u8)
// Size:   24 × 10 × 1 byte = 240 bytes — trivially small
// Lives:  internal flash (tiny, paste directly as const)
//
// sp_move[c][mi] = udslice perm coord after applying
//                  PHASE2_MOVES[mi] to a belt with SP coord c
// -------------------------------------------------------
fn gen_sp_move_table() -> Vec<[u8; 10]> {
    // SP is only valid in G1. BFS using Phase 2 moves.
    let mut reps: Vec<Option<CubieState>> = vec![None; 24];
    let mut queue: VecDeque<CubieState> = VecDeque::new();

    let solved = CubieState::make_solved();
    reps[solved.udslice_perm_coord()] = Some(solved);
    queue.push_back(solved);

    while let Some(state) = queue.pop_front() {
        for &m in PHASE2_MOVES.iter() {
            let mut next = state;
            next.apply_move(m);
            if next.udslice_coord() != 0 { continue; }
            let coord = next.udslice_perm_coord();
            if reps[coord].is_none() {
                reps[coord] = Some(next);
                queue.push_back(next);
            }
        }
    }

    for (c, rep) in reps.iter().enumerate() {
        assert!(rep.is_some(),
            "SP move table: coord {} never reached", c);
    }

    let reps: Vec<CubieState> = reps.into_iter().map(|r| r.unwrap()).collect();

    let mut table = vec![[0u8; 10]; 24];
    for (c, rep) in reps.iter().enumerate() {
        for (mi, &m) in PHASE2_MOVES.iter().enumerate() {
            let mut next = *rep;
            next.apply_move(m);
            table[c][mi] = next.udslice_perm_coord() as u8;
        }
    }

    println!("// SP_MOVE_TABLE generated: {} entries", 24 * 10);
    table
}


// ============================================================
// COMBINED PRUNING TABLE GENERATION
// ============================================================
//
// These tables capture the interaction between two coordinates
// simultaneously. The independent pruning tables miss this:
//
//   max(CO_TABLE[co], EO_TABLE[eo]) may return 4
//   but fixing BOTH CO and EO simultaneously may require 7 moves
//   because the moves that fix CO disturb EO and vice versa.
//
// The combined table stores the TRUE minimum over both at once.
//
// CRITICAL EFFICIENCY: once we have the move tables, we never
// need CubieState again for BFS. We BFS directly over coord
// pairs using the move tables. This is much faster than
// BFS over CubieState.
// ============================================================

// -------------------------------------------------------
// FLIP_UDSLICE TABLE  (combined EO + UD-slice membership)
// Size:   2048 × 495 = 1,013,760 bytes ≈ 1MB
// Index:  eo_coord * 495 + ud_coord
// Lives:  QSPI flash
//
// Answers: given this joint (EO, UD-slice) state, what is
// the minimum number of moves to reach EO=0 AND UD=0?
//
// This is the key table that fixes Phase 1 performance.
// It captures that fixing edge orientation and getting belt
// edges into the belt are coupled — you can't optimize them
// independently.
// -------------------------------------------------------
fn gen_flip_udslice_table(
    eo_move: &[[u16; 18]],
    ud_move: &[[u16; 18]],
) -> Vec<u8> {
    // Index: eo * 495 + ud
    const SIZE: usize = 2048 * 495;
    let mut table = vec![u8::MAX; SIZE];

    // BFS queue holds (eo_coord, ud_coord) pairs directly.
    // No CubieState needed — move tables handle all transitions.
    let mut queue: VecDeque<(u16, u16)> = VecDeque::new();

    // Solved state: eo=0, ud=0 → index 0, distance 0
    table[0] = 0;
    queue.push_back((0, 0));

    while let Some((eo, ud)) = queue.pop_front() {
        let idx = eo as usize * 495 + ud as usize;
        let dist = table[idx];

        // Apply all 18 moves using the move tables
        for mi in 0..18 {
            let next_eo = eo_move[eo as usize][mi];
            let next_ud = ud_move[ud as usize][mi];
            let next_idx = next_eo as usize * 495 + next_ud as usize;

            if table[next_idx] == u8::MAX {
                table[next_idx] = dist + 1;
                queue.push_back((next_eo, next_ud));
            }
        }
    }

    assert!(!table.contains(&u8::MAX),
        "FLIP_UDSLICE BFS left unreachable states");
    assert_eq!(table[0], 0, "Solved state must have distance 0");

    println!("// FLIP_UDSLICE_TABLE max distance: {}",
        table.iter().max().unwrap());
    println!("// States at each depth:");
    for d in 0..=*table.iter().max().unwrap() {
        println!("//   depth {}: {} states",
            d, table.iter().filter(|&&v| v == d).count());
    }

    table
}

// -------------------------------------------------------
// CORNERS_SLICE2 TABLE  (combined CP + SP)
// Size:   40320 × 24 = 967,680 bytes ≈ 945KB
// Index:  cp_coord * 24 + sp_coord
// Lives:  QSPI flash
//
// Answers: given this joint (corner perm, belt perm) state,
// what is the minimum Phase 2 moves to reach CP=0 AND SP=0?
// -------------------------------------------------------
fn gen_corners_slice2_table(
    cp_move: &[[u16; 10]],
    sp_move: &[[u8; 10]],
) -> Vec<u8> {
    const SIZE: usize = 40320 * 24;
    let mut table = vec![u8::MAX; SIZE];
    let mut queue: VecDeque<(u16, u8)> = VecDeque::new();

    // Solved state: cp=0, sp=0 → index 0
    table[0] = 0;
    queue.push_back((0, 0));

    while let Some((cp, sp)) = queue.pop_front() {
        let idx = cp as usize * 24 + sp as usize;
        let dist = table[idx];

        // Apply all 10 Phase 2 moves
        for mi in 0..10 {
            let next_cp = cp_move[cp as usize][mi];
            let next_sp = sp_move[sp as usize][mi];
            let next_idx = next_cp as usize * 24 + next_sp as usize;

            if table[next_idx] == u8::MAX {
                table[next_idx] = dist + 1;
                queue.push_back((next_cp, next_sp));
            }
        }
    }

    assert!(!table.contains(&u8::MAX),
        "CORNERS_SLICE2 BFS left unreachable states");
    assert_eq!(table[0], 0);

    println!("// CORNERS_SLICE2_TABLE max distance: {}",
        table.iter().max().unwrap());
    println!("// States at each depth:");
    for d in 0..=*table.iter().max().unwrap() {
        println!("//   depth {}: {} states",
            d, table.iter().filter(|&&v| v == d).count());
    }

    table
}

// -------------------------------------------------------
// EDGES_SLICE2 TABLE  (combined EP + SP)
// Size:   40320 × 24 = 967,680 bytes ≈ 945KB
// Index:  ep_coord * 24 + sp_coord
// Lives:  QSPI flash
//
// Answers: given this joint (edge perm, belt perm) state,
// what is the minimum Phase 2 moves to reach EP=0 AND SP=0?
// -------------------------------------------------------
fn gen_edges_slice2_table(
    ep_move: &[[u16; 10]],
    sp_move: &[[u8; 10]],
) -> Vec<u8> {
    const SIZE: usize = 40320 * 24;
    let mut table = vec![u8::MAX; SIZE];
    let mut queue: VecDeque<(u16, u8)> = VecDeque::new();

    table[0] = 0;
    queue.push_back((0, 0));

    while let Some((ep, sp)) = queue.pop_front() {
        let idx = ep as usize * 24 + sp as usize;
        let dist = table[idx];

        for mi in 0..10 {
            let next_ep = ep_move[ep as usize][mi];
            let next_sp = sp_move[sp as usize][mi];
            let next_idx = next_ep as usize * 24 + next_sp as usize;

            if table[next_idx] == u8::MAX {
                table[next_idx] = dist + 1;
                queue.push_back((next_ep, next_sp));
            }
        }
    }

    assert!(!table.contains(&u8::MAX),
        "EDGES_SLICE2 BFS left unreachable states");
    assert_eq!(table[0], 0);

    println!("// EDGES_SLICE2_TABLE max distance: {}",
        table.iter().max().unwrap());
    println!("// States at each depth:");
    for d in 0..=*table.iter().max().unwrap() {
        println!("//   depth {}: {} states",
            d, table.iter().filter(|&&v| v == d).count());
    }

    table
}



/* 



fn generate_edge_orientation_table() {
    let table = bfs_edge_orient();
    print!("pub const EDGE_ORIENT_TABLE: [u8; 2048] = [");
    for (_, &v) in table.iter().enumerate() {
        print!("{}, ", v);
    }
    println!("];");
}

fn generate_ud_slice_table() {
    let table = bfs_udslice();
    print!("pub const UD_SLICE_TABLE: [u8; 495] = [");
    for (_, &v) in table.iter().enumerate() {
        print!("{}, ", v);
    }
    println!("];");
}

fn generate_corner_permutation_table() {
    let table = bfs_corner_perm();
    print!("pub const CORNER_PERMUTATION_TABLE: [u8; 40320] = [");
    for (_, &v) in table.iter().enumerate() {
        print!("{}, ", v);
    }
    println!("];");
}

fn generate_udslice_permutation_table() {
    let table = bfs_udslice_perm();
    print!("pub const UDSLICE_PERMUTATION_TABLE: [u8; 24] = [");
    for (_, &v) in table.iter().enumerate() {
        print!("{}, ", v);
    }
    println!("];");
}

fn generate_edge_permutation_table() {
    let table = bfs_edge_perm();
    print!("pub const EDGE_PERMUTATION_TABLE: [u8; 40320] = [");
    for (_, &v) in table.iter().enumerate() {
        print!("{}, ", v);
    }
    println!("];");
}

const ORIENT_UD: [u8; 3] = [0, 2, 1];
const ORIENT_LR: [u8; 3] = [1, 0, 2];
const ORIENT_FB: [u8; 3] = [2, 1, 0];

// Applies one quarter-turn cycle to a slot-based orientation array.
// slots[0]->slots[1]->slots[2]->slots[3]->slots[0], each with map applied.
fn cycle4_co(o: &mut [u8; 8], slots: [usize; 4], map: [u8; 3]) {
    let (o0, o1, o2, o3) = (o[slots[0]], o[slots[1]], o[slots[2]], o[slots[3]]);
    o[slots[1]] = map[o0 as usize];
    o[slots[2]] = map[o1 as usize];
    o[slots[3]] = map[o2 as usize];
    o[slots[0]] = map[o3 as usize];
}

fn apply_co_move(o: &mut [u8; 8], m: solver_move_t) {
    match m {
        solver_move_t::U  => cycle4_co(o, [3,2,0,1], ORIENT_UD),
        solver_move_t::Ui => { for _ in 0..3 { cycle4_co(o, [3,2,0,1], ORIENT_UD); } }
        solver_move_t::U2 => { for _ in 0..2 { cycle4_co(o, [3,2,0,1], ORIENT_UD); } }

        solver_move_t::D  => cycle4_co(o, [4,5,7,6], ORIENT_UD),
        solver_move_t::Di => { for _ in 0..3 { cycle4_co(o, [4,5,7,6], ORIENT_UD); } }
        solver_move_t::D2 => { for _ in 0..2 { cycle4_co(o, [4,5,7,6], ORIENT_UD); } }

        solver_move_t::R  => cycle4_co(o, [3,1,7,5], ORIENT_LR),
        solver_move_t::Ri => { for _ in 0..3 { cycle4_co(o, [3,1,7,5], ORIENT_LR); } }
        solver_move_t::R2 => { for _ in 0..2 { cycle4_co(o, [3,1,7,5], ORIENT_LR); } }

        solver_move_t::L  => cycle4_co(o, [0,2,4,6], ORIENT_LR),
        solver_move_t::Li => { for _ in 0..3 { cycle4_co(o, [0,2,4,6], ORIENT_LR); } }
        solver_move_t::L2 => { for _ in 0..2 { cycle4_co(o, [0,2,4,6], ORIENT_LR); } }

        solver_move_t::F  => cycle4_co(o, [2,3,5,4], ORIENT_FB),
        solver_move_t::Fi => { for _ in 0..3 { cycle4_co(o, [2,3,5,4], ORIENT_FB); } }
        solver_move_t::F2 => { for _ in 0..2 { cycle4_co(o, [2,3,5,4], ORIENT_FB); } }

        solver_move_t::B  => cycle4_co(o, [0,6,7,1], ORIENT_FB),
        solver_move_t::Bi => { for _ in 0..3 { cycle4_co(o, [0,6,7,1], ORIENT_FB); } }
        solver_move_t::B2 => { for _ in 0..2 { cycle4_co(o, [0,6,7,1], ORIENT_FB); } }
    }
}

fn encode_co(o: &[u8; 8]) -> usize {
    let mut coord = 0;
    for i in 0..7 { coord = coord * 3 + o[i] as usize; }
    coord
}

fn bfs_corner_orient() -> [u8; 2187] {
    let mut table = [u8::MAX; 2187];
    let mut queue: VecDeque<[u8; 8]> = VecDeque::new();

    let start = [0u8; 8];
    table[encode_co(&start)] = 0;
    queue.push_back(start);

    while let Some(orient) = queue.pop_front() {
        let dist = table[encode_co(&orient)];
        for &m in ALL_MOVES.iter() {
            let mut next = orient;
            apply_co_move(&mut next, m);
            let coord = encode_co(&next);
            if table[coord] == u8::MAX {
                table[coord] = dist + 1;
                queue.push_back(next);
            }
        }
    }
    // Sanity checks before returning
    assert!(!table.contains(&u8::MAX), "BFS left unreachable states!");
    assert_eq!(table[0], 0, "Solved state should have distance 0");
    println!("// Max distance: {}", table.iter().max().unwrap());
    println!("// States at each depth:");
    for d in 0..=*table.iter().max().unwrap() {
        println!("//   depth {}: {} states", d, table.iter().filter(|&&v| v == d).count());
    }

    table
}

fn cycle4_eo(f: &mut [u8; 12], slots: [usize; 4], flip: bool) {
    let (f0, f1, f2, f3) = (f[slots[0]], f[slots[1]], f[slots[2]], f[slots[3]]);
    let x = flip as u8;
    f[slots[1]] = f0 ^ x;
    f[slots[2]] = f1 ^ x;
    f[slots[3]] = f2 ^ x;
    f[slots[0]] = f3 ^ x;
}

fn apply_eo_move(f: &mut [u8; 12], m: solver_move_t) {
    match m {
        solver_move_t::U  => cycle4_eo(f, [0,2,3,1], false),
        solver_move_t::Ui => { for _ in 0..3 { cycle4_eo(f, [0,2,3,1], false); } }
        solver_move_t::U2 => { for _ in 0..2 { cycle4_eo(f, [0,2,3,1], false); } }

        solver_move_t::D  => cycle4_eo(f, [4,6,7,5], false),
        solver_move_t::Di => { for _ in 0..3 { cycle4_eo(f, [4,6,7,5], false); } }
        solver_move_t::D2 => { for _ in 0..2 { cycle4_eo(f, [4,6,7,5], false); } }

        solver_move_t::R  => cycle4_eo(f, [2,11,6,10], true),
        solver_move_t::Ri => { for _ in 0..3 { cycle4_eo(f, [2,11,6,10], true); } }
        solver_move_t::R2 => { for _ in 0..2 { cycle4_eo(f, [2,11,6,10], true); } }

        solver_move_t::L  => cycle4_eo(f, [1,9,5,8], true),
        solver_move_t::Li => { for _ in 0..3 { cycle4_eo(f, [1,9,5,8], true); } }
        solver_move_t::L2 => { for _ in 0..2 { cycle4_eo(f, [1,9,5,8], true); } }

        solver_move_t::F  => cycle4_eo(f, [3,10,4,9], false),
        solver_move_t::Fi => { for _ in 0..3 { cycle4_eo(f, [3,10,4,9], false); } }
        solver_move_t::F2 => { for _ in 0..2 { cycle4_eo(f, [3,10,4,9], false); } }

        solver_move_t::B  => cycle4_eo(f, [0,8,7,11], false),
        solver_move_t::Bi => { for _ in 0..3 { cycle4_eo(f, [0,8,7,11], false); } }
        solver_move_t::B2 => { for _ in 0..2 { cycle4_eo(f, [0,8,7,11], false); } }
    }
}

fn encode_eo(f: &[u8; 12]) -> usize {
    let mut coord = 0;
    for i in 0..11 { coord = coord * 2 + f[i] as usize; }
    coord
}

fn bfs_edge_orient() -> [u8; 2048] {
    let mut table = [u8::MAX; 2048];
    let mut queue: VecDeque<[u8; 12]> = VecDeque::new();

    let start = [0u8; 12];
    table[encode_eo(&start)] = 0;
    queue.push_back(start);

    while let Some(flip) = queue.pop_front() {
        let dist = table[encode_eo(&flip)];
        for &m in ALL_MOVES.iter() {
            let mut next = flip;
            apply_eo_move(&mut next, m);
            let coord = encode_eo(&next);
            if table[coord] == u8::MAX {
                table[coord] = dist + 1;
                queue.push_back(next);
            }
        }
    }

    assert!(!table.contains(&u8::MAX), "BFS left unreachable states!");
    assert_eq!(table[0], 0, "Solved state should have distance 0");
    println!("// Max distance: {}", table.iter().max().unwrap());
    println!("// States at each depth:");
    for d in 0..=*table.iter().max().unwrap() {
        println!("//   depth {}: {} states", d, table.iter().filter(|&&v| v == d).count());
    }
    table
}

// ============================================================
// UD-SLICE BFS
// ============================================================
//
// WHAT ARE WE TRACKING?
// The UD-slice consists of the 4 "equatorial belt" edges:
// LB(8), LF(9), RF(10), RB(11). On a solved cube, these 4
// cubies sit in slots 8, 9, 10, and 11. When the cube is
// scrambled, they get displaced into other slots.
//
// The UD-slice coordinate answers ONE question:
// "Which 4 of the 12 edge slots currently contain a UD-slice edge?"
// It does NOT care about order, orientation, or which specific
// UD-slice edge is in each slot — just membership.
//
// STATE REPRESENTATION:
// We represent the state as a [bool; 12], where:
//   occupied[s] = true  means slot s contains a UD-slice edge
//   occupied[s] = false means slot s contains a non-UD-slice edge
// Exactly 4 slots will always be true (there are exactly 4 UD edges).
//
// WHY SLOT-BASED?
// Same reason as corner/edge orientation: a move's effect on
// slot membership depends only on which slots are occupied,
// not on which specific cubies occupy them. So the BFS stays
// consistent — coord X always transitions to the same new coords
// regardless of which state first produced coord X.

// Applies one quarter-turn 4-cycle to the UD-slice membership array.
// This is the slot-based version of cycle_edges in lib.rs.
// The cubie sitting in slots[0] moves to slots[1],
// the cubie in slots[1] moves to slots[2], and so on.
// We don't care about flip here — UD-slice coord ignores orientation.
fn cycle4_udslice(occupied: &mut [bool; 12], slots: [usize; 4]) {
    // Save all four membership values before any writes.
    // If we wrote first and read second, we'd corrupt values mid-cycle.
    let (o0, o1, o2, o3) = (
        occupied[slots[0]],
        occupied[slots[1]],
        occupied[slots[2]],
        occupied[slots[3]],
    );

    // Rotate: each slot receives the membership of the previous slot.
    // The cubie that WAS in slots[0] is now in slots[1], so slots[1]
    // gets o0. The cubie that was in slots[3] wraps around to slots[0].
    occupied[slots[1]] = o0;
    occupied[slots[2]] = o1;
    occupied[slots[3]] = o2;
    occupied[slots[0]] = o3;
}

// Applies any of the 18 moves to the slot-based UD-slice state.
// The slot cycles here are copied directly from lib.rs cycle_edges calls.
// We apply the quarter-turn once, three times (= inverse), or twice
// depending on the move type. This exactly mirrors how CubieState::apply_move
// works, but operating on our lightweight [bool; 12] instead.
fn apply_udslice_move(occupied: &mut [bool; 12], m: solver_move_t) {
    match m {
        // U and D moves only cycle the top/bottom 4 edge slots (0-3, 4-7).
        // None of those slots are UD-slice slots (8-11), so U/D moves
        // never actually change the UD-slice coordinate. We still apply
        // them correctly here so the BFS stays accurate.
        solver_move_t::U  => cycle4_udslice(occupied, [0, 2, 3, 1]),
        solver_move_t::Ui => { for _ in 0..3 { cycle4_udslice(occupied, [0, 2, 3, 1]); } }
        solver_move_t::U2 => { for _ in 0..2 { cycle4_udslice(occupied, [0, 2, 3, 1]); } }

        solver_move_t::D  => cycle4_udslice(occupied, [4, 6, 7, 5]),
        solver_move_t::Di => { for _ in 0..3 { cycle4_udslice(occupied, [4, 6, 7, 5]); } }
        solver_move_t::D2 => { for _ in 0..2 { cycle4_udslice(occupied, [4, 6, 7, 5]); } }

        // R, L, F, B moves each cycle one slot from the UD-slice belt
        // (slots 8-11) with three non-belt slots. These are the moves
        // that actually displace UD-slice edges and change this coordinate.
        solver_move_t::R  => cycle4_udslice(occupied, [2, 11, 6, 10]),
        solver_move_t::Ri => { for _ in 0..3 { cycle4_udslice(occupied, [2, 11, 6, 10]); } }
        solver_move_t::R2 => { for _ in 0..2 { cycle4_udslice(occupied, [2, 11, 6, 10]); } }

        solver_move_t::L  => cycle4_udslice(occupied, [1, 9, 5, 8]),
        solver_move_t::Li => { for _ in 0..3 { cycle4_udslice(occupied, [1, 9, 5, 8]); } }
        solver_move_t::L2 => { for _ in 0..2 { cycle4_udslice(occupied, [1, 9, 5, 8]); } }

        solver_move_t::F  => cycle4_udslice(occupied, [3, 10, 4, 9]),
        solver_move_t::Fi => { for _ in 0..3 { cycle4_udslice(occupied, [3, 10, 4, 9]); } }
        solver_move_t::F2 => { for _ in 0..2 { cycle4_udslice(occupied, [3, 10, 4, 9]); } }

        solver_move_t::B  => cycle4_udslice(occupied, [0, 8, 7, 11]),
        solver_move_t::Bi => { for _ in 0..3 { cycle4_udslice(occupied, [0, 8, 7, 11]); } }
        solver_move_t::B2 => { for _ in 0..2 { cycle4_udslice(occupied, [0, 8, 7, 11]); } }
    }
}

// Computes the binomial coefficient C(n, k) = n! / (k! * (n-k)!).
// This is a free-function copy of the one in lib.rs (which is private
// to CubieState). We need it here to rank combinations for udslice coord.
// Using Pascal's triangle recurrence so we never deal with large factorials.
// The compiler can evaluate this at compile time (const fn).
const fn choose(n: usize, k: usize) -> usize {
    // By definition, you cannot choose more items than exist
    if k > n { return 0; }
    match (n, k) {
        // There is exactly one way to choose nothing from any set
        (_, 0) => 1,
        // You cannot choose a positive number of items from an empty set
        (0, _) => 0,
        // Pascal's triangle: C(n,k) = C(n-1,k-1) + C(n-1,k)
        // This recurses without ever computing a factorial directly
        _ => choose(n - 1, k - 1) + choose(n - 1, k),
    }
}

// Encodes a [bool; 12] UD-slice membership array into a coordinate in [0, 494].
// This is the combinatorial ranking algorithm — the same one used in
// udslice_coord() in lib.rs — now operating directly on a bool array
// instead of going through CubieState.
//
// The ranking works by walking slots from HIGH to LOW and accumulating
// C(i, k) for each unoccupied slot, where k counts how many occupied
// slots remain to be "accounted for". Think of it as counting how many
// valid subsets you skip over before reaching this particular arrangement.
fn encode_udslice(occupied: &[bool; 12]) -> usize {
    let mut coord = 0usize;

    // k starts at 3 and counts down as we encounter occupied slots.
    // It represents: "how many more UD-slice edges do we still need to place
    // to the left of our current position in the ranking?"
    let mut k = 3usize;

    // Iterate slots from high (11) to low (0)
    for i in (0..12).rev() {
        if occupied[i] {
            // This slot holds a UD-slice edge.
            // We've "placed" one more UD-slice edge — decrement k.
            // Once k hits 0, all remaining occupied slots are already
            // accounted for and contribute nothing, so we stop.
            if k == 0 { break; }
            k -= 1;
        } else {
            // This slot does NOT hold a UD-slice edge.
            // We add C(i, k): the number of ways to arrange the remaining
            // k UD-slice edges into the slots below position i.
            // This is what gives each combination a unique rank.
            coord += choose(i, k);
        }
    }
    coord
}

// BFS over all 495 UD-slice states.
// Returns a table where table[coord] = minimum moves to get the
// 4 UD-slice edges back into slots 8-11, regardless of their order.
fn bfs_udslice() -> [u8; 495] {
    // Initialize every entry to u8::MAX as a sentinel meaning "not yet visited".
    // After BFS completes, any remaining u8::MAX would indicate a bug.
    let mut table = [u8::MAX; 495];

    // BFS queue holds the raw slot-membership state, not just the coord.
    // We need the full state to apply moves and generate neighbors.
    let mut queue: VecDeque<[bool; 12]> = VecDeque::new();

    // The solved state: UD-slice edges are in exactly slots 8, 9, 10, 11.
    // All other slots (0-7) are occupied by non-UD-slice edges.
    let mut start = [false; 12];
    start[8]  = true;
    start[9]  = true;
    start[10] = true;
    start[11] = true;

    // The solved state encodes to coord 0 by definition of our ranking.
    // Mark it visited with distance 0 and enqueue it.
    table[encode_udslice(&start)] = 0;
    queue.push_back(start);

    // Standard BFS loop: pop front, expand all 18 neighbors, enqueue unvisited ones.
    while let Some(state) = queue.pop_front() {
        // Look up the distance of the current state from the table.
        // We do this instead of storing distance in the queue because the
        // coord uniquely identifies the state and its distance.
        let dist = table[encode_udslice(&state)];

        // Try all 18 possible moves from this state
        for &m in ALL_MOVES.iter() {
            // Clone the current state and apply the move to get a neighbor
            let mut next = state;
            apply_udslice_move(&mut next, m);

            // Compute the coord of the neighbor
            let coord = encode_udslice(&next);

            // Only process this neighbor if we haven't seen this coord before.
            // First visit = shortest path, guaranteed by BFS level-by-level order.
            if table[coord] == u8::MAX {
                // This coord is newly discovered at distance dist+1
                table[coord] = dist + 1;
                queue.push_back(next);
            }
        }
    }

    // Sanity checks — if these fail, something is wrong with the BFS or encoding
    assert!(!table.contains(&u8::MAX), "BFS left unreachable UD-slice states!");
    assert_eq!(table[0], 0, "Solved state (coord 0) must have distance 0");
    println!("// Max distance: {}", table.iter().max().unwrap());
    println!("// States at each depth:");
    for d in 0..=*table.iter().max().unwrap() {
        println!("//   depth {}: {} states", d, table.iter().filter(|&&v| v == d).count());
    }

    table
}


// ============================================================
// CORNER PERMUTATION BFS
// ============================================================
//
// WHAT ARE WE TRACKING?
// The corner permutation coordinate encodes WHICH corner cubie
// is sitting in each of the 8 corner slots. It completely ignores
// orientation — it only cares about which cubie is where.
//
// STATE REPRESENTATION:
// We represent the state as a [u8; 8], where:
//   perm[slot] = cubie_id sitting in that slot
// On the solved cube, perm = [0, 1, 2, 3, 4, 5, 6, 7] — every
// cubie sits in its home slot.
//
// ENCODING:
// We use the Lehmer code (factoriadic system). For each slot i,
// count how many cubies to the RIGHT of slot i have a smaller
// cubie_id than perm[i]. Multiply that count by (7-i)! and sum.
// This gives a unique integer in [0, 40319] = [0, 8!-1].
//
// WHY SLOT-BASED IS NATURAL HERE:
// perm[slot] = cubie_id is inherently slot-indexed. Moves
// physically move cubies between slots, so we just cycle
// the perm array entries. No ambiguity like we had with
// cubie-indexed orientation encoding.

// Applies one quarter-turn 4-cycle to the corner permutation array.
// The cubie sitting in slots[0] moves to slots[1],
// the cubie in slots[1] moves to slots[2], and so on.
// Orientation is completely ignored here — we only track position.
fn cycle4_cperm(perm: &mut [u8; 8], slots: [usize; 4]) {
    // Save all four cubie IDs before any writes.
    // Without this, writing perm[slots[1]] first would corrupt the
    // value we need to read for the next step of the cycle.
    let (p0, p1, p2, p3) = (
        perm[slots[0]],
        perm[slots[1]],
        perm[slots[2]],
        perm[slots[3]],
    );

    // Rotate: the cubie that was in slots[0] is now in slots[1], etc.
    // The cubie from slots[3] wraps around to slots[0].
    perm[slots[1]] = p0;
    perm[slots[2]] = p1;
    perm[slots[3]] = p2;
    perm[slots[0]] = p3;
}

// Applies any of the 18 moves to the slot-based corner permutation state.
// The slot cycles are copied directly from lib.rs cycle_corners calls —
// the same 4-cycles, just applied to our lightweight [u8; 8] instead of CubieState.
fn apply_cperm_move(perm: &mut [u8; 8], m: solver_move_t) {
    match m {
        // U cycles the 4 top corners: UFR(3)->UFL(2)->UBL(0)->UBR(1)
        solver_move_t::U  => cycle4_cperm(perm, [3, 2, 0, 1]),
        solver_move_t::Ui => { for _ in 0..3 { cycle4_cperm(perm, [3, 2, 0, 1]); } }
        solver_move_t::U2 => { for _ in 0..2 { cycle4_cperm(perm, [3, 2, 0, 1]); } }

        // D cycles the 4 bottom corners: DFL(4)->DFR(5)->DBR(7)->DBL(6)
        solver_move_t::D  => cycle4_cperm(perm, [4, 5, 7, 6]),
        solver_move_t::Di => { for _ in 0..3 { cycle4_cperm(perm, [4, 5, 7, 6]); } }
        solver_move_t::D2 => { for _ in 0..2 { cycle4_cperm(perm, [4, 5, 7, 6]); } }

        // R cycles UFR(3)->UBR(1)->DBR(7)->DFR(5)
        solver_move_t::R  => cycle4_cperm(perm, [3, 1, 7, 5]),
        solver_move_t::Ri => { for _ in 0..3 { cycle4_cperm(perm, [3, 1, 7, 5]); } }
        solver_move_t::R2 => { for _ in 0..2 { cycle4_cperm(perm, [3, 1, 7, 5]); } }

        // L cycles UBL(0)->UFL(2)->DFL(4)->DBL(6)
        solver_move_t::L  => cycle4_cperm(perm, [0, 2, 4, 6]),
        solver_move_t::Li => { for _ in 0..3 { cycle4_cperm(perm, [0, 2, 4, 6]); } }
        solver_move_t::L2 => { for _ in 0..2 { cycle4_cperm(perm, [0, 2, 4, 6]); } }

        // F cycles UFL(2)->UFR(3)->DFR(5)->DFL(4)
        solver_move_t::F  => cycle4_cperm(perm, [2, 3, 5, 4]),
        solver_move_t::Fi => { for _ in 0..3 { cycle4_cperm(perm, [2, 3, 5, 4]); } }
        solver_move_t::F2 => { for _ in 0..2 { cycle4_cperm(perm, [2, 3, 5, 4]); } }

        // B cycles UBL(0)->DBL(6)->DBR(7)->UBR(1)
        solver_move_t::B  => cycle4_cperm(perm, [0, 6, 7, 1]),
        solver_move_t::Bi => { for _ in 0..3 { cycle4_cperm(perm, [0, 6, 7, 1]); } }
        solver_move_t::B2 => { for _ in 0..2 { cycle4_cperm(perm, [0, 6, 7, 1]); } }
    }
}

// Encodes a [u8; 8] corner permutation into a Lehmer code in [0, 40319].
// This mirrors corner_perm_coord() in lib.rs exactly, but operates on
// our raw perm array instead of going through CubieState.
//
// The Lehmer code works like a mixed-radix number:
//   - For slot 0, its digit can be 0..7 (8 choices), multiplied by 7!
//   - For slot 1, its digit can be 0..6 (7 remaining choices), multiplied by 6!
//   - And so on, down to slot 7 which always contributes 0.
// The digit for each slot is "how many cubies to my right have a smaller ID?"
fn encode_cperm(perm: &[u8; 8]) -> usize {
    // Factorials 0! through 7!, precomputed to avoid recalculating per call.
    // FACTORIAL[i] = i!, so FACTORIAL[7] = 5040.
    const FACTORIAL: [usize; 8] = [1, 1, 2, 6, 24, 120, 720, 5040];

    let mut coord = 0usize;

    for i in 0..8 {
        // Count how many cubie IDs to the RIGHT of position i are
        // strictly smaller than perm[i].
        // This is the Lehmer digit for position i.
        let smaller = ((i + 1)..8)
            .filter(|&j| perm[j] < perm[i])
            .count();

        // Each Lehmer digit is weighted by the factorial of remaining positions.
        // Position 0 has 7 positions after it  -> weight 7! = 5040
        // Position 1 has 6 positions after it  -> weight 6! = 720
        // ...
        // Position 7 has 0 positions after it  -> weight 0! = 1, but smaller is always 0
        coord += smaller * FACTORIAL[7 - i];
    }
    coord
}

// BFS over all 40320 corner permutation states.
// Returns a table where table[coord] = minimum moves needed to
// restore all 8 corners to their home slots, ignoring orientation.
// This is the most expensive of the four BFS runs: 40320 states,
// each expanded with 18 moves. Still runs in well under a second on PC.
fn bfs_corner_perm() -> [u8; 40320] {
    // Initialize every entry to u8::MAX as "not yet visited".
    // 40320 bytes = ~40KB of stack — fine for a PC binary.
    let mut table = [u8::MAX; 40320];

    // Queue holds the raw perm arrays, not just coords.
    // We need the full permutation to apply moves and generate neighbors.
    let mut queue: VecDeque<[u8; 8]> = VecDeque::new();

    // Solved state: every corner cubie sits in its home slot.
    // perm[slot] = slot means cubie 0 is in slot 0, cubie 1 in slot 1, etc.
    let start: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

    // Solved state encodes to coord 0 — the Lehmer code of the identity
    // permutation is 0 because no cubie has any smaller cubie to its right.
    table[encode_cperm(&start)] = 0;
    queue.push_back(start);

    // Standard BFS: process states level by level (one move at a time).
    // Because we expand all neighbors before going deeper, the first time
    // we reach a coord is always via the shortest possible path.
    while let Some(perm) = queue.pop_front() {
        // Retrieve this state's distance from the table using its coord.
        // We stored it when we enqueued, so it's guaranteed to be set.
        let dist = table[encode_cperm(&perm)];

        // Generate all 18 one-move neighbors
        for &m in ALL_MOVES.iter() {
            // Copy the current permutation and apply the move.
            // We work on a copy so the original stays unchanged for
            // the remaining 17 moves we still need to try.
            let mut next = perm;
            apply_cperm_move(&mut next, m);

            // Encode the neighbor permutation to get its table index
            let coord = encode_cperm(&next);

            // Only enqueue if this coord hasn't been reached before.
            // u8::MAX means "unvisited" — any other value means we
            // already found a shorter or equal path to this coord.
            if table[coord] == u8::MAX {
                // First visit: record distance and enqueue for expansion
                table[coord] = dist + 1;
                queue.push_back(next);
            }
        }
    }

    // Sanity checks
    assert!(!table.contains(&u8::MAX), "BFS left unreachable corner perm states!");
    assert_eq!(table[0], 0, "Solved state (coord 0) must have distance 0");
    println!("// Max distance: {}", table.iter().max().unwrap());
    println!("// States at each depth:");
    for d in 0..=*table.iter().max().unwrap() {
        println!("//   depth {}: {} states", d, table.iter().filter(|&&v| v == d).count());
    }

    table
}

// ============================================================
// UD-SLICE PERMUTATION TABLE
// ============================================================
//
// WHAT ARE WE TRACKING?
// Phase 1 guarantees UD-slice edges are in belt slots 8-11.
// This table asks: in what ORDER are they arranged across those 4 slots?
// There are 4! = 24 possible orderings.
//
// STATE REPRESENTATION:
// [u8; 4] where entry i = which UD-slice cubie (normalized 0-3) sits in
// belt slot (8 + i). On a solved cube this is [0, 1, 2, 3].
//
// WHY SLOT-BASED WORKS:
// Given the full ordering of which cubie is in which belt slot,
// applying a Phase 2 move produces exactly one deterministic new ordering.
// No ambiguity from hidden state elsewhere in the cube.
//
// MOVE SET:
// Only Phase 2 moves: {U, U2, Ui, D, D2, Di, R2, L2, F2, B2}
// These are the only moves that keep belt edges within the belt.
// Using all 18 moves here would be wrong — quarter turns of R/L/F/B
// kick belt edges out of the belt, making the coord undefined.

// Applies one 4-cycle to the belt permutation array.
// slots here are LOCAL indices into the belt (0-3, representing
// absolute slots 8-11), not global edge slot indices.
// The cubie sitting in local slot slots[0] moves to local slots[1], etc.
fn cycle4_udslice_perm(perm: &mut [u8; 4], local_slots: [usize; 4]) {
    // Save all four values before any writes to avoid corruption mid-cycle
    let (p0, p1, p2, p3) = (
        perm[local_slots[0]],
        perm[local_slots[1]],
        perm[local_slots[2]],
        perm[local_slots[3]],
    );
    // Rotate: cubie from local_slots[0] goes to local_slots[1], etc.
    perm[local_slots[1]] = p0;
    perm[local_slots[2]] = p1;
    perm[local_slots[3]] = p2;
    perm[local_slots[0]] = p3;
}

// Applies a Phase 2 move to the belt permutation state.
//
// CRITICAL: we only care about the 4 belt slots (8,9,10,11).
// For each Phase 2 move, we need to know which belt slots cycle
// among themselves. Let's derive these from the full edge cycles
// in lib.rs, keeping only the belt slot indices (8-11):
//
// U:  cycles [0, 2, 3, 1]      — no belt slots involved, belt unchanged
// D:  cycles [4, 6, 7, 5]      — no belt slots involved, belt unchanged
// R2: cycles [2, 11, 6, 10]    — belt slots 11 and 10 swap
//     applied twice: 11<->10 swap (within belt local indices 3<->2)
// L2: cycles [1, 9, 5, 8]      — belt slots 9 and 8 swap
//     applied twice: 9<->8 swap (within belt local indices 1<->0)
// F2: cycles [3, 10, 4, 9]     — belt slots 10 and 9 swap
//     applied twice: 10<->9 swap (within belt local indices 2<->1)
// B2: cycles [0, 8, 7, 11]     — belt slots 8 and 11 swap
//     applied twice: 8<->11 swap (within belt local indices 0<->3)
//
// For double moves (X2), a 4-cycle applied twice = two 2-swaps.
// Local belt index = absolute slot - 8.
// e.g. absolute slot 11 -> local index 3
//      absolute slot 10 -> local index 2
//      absolute slot  9 -> local index 1
//      absolute slot  8 -> local index 0
fn apply_udslice_perm_move(perm: &mut [u8; 4], m: solver_move_t) {
    match m {
        // U and D don't touch belt slots at all — no change to belt perm
        solver_move_t::U  |
        solver_move_t::Ui |
        solver_move_t::U2 |
        solver_move_t::D  |
        solver_move_t::Di |
        solver_move_t::D2 => { /* belt unchanged */ }

        // R2: belt slots 10(local 2) and 11(local 3) swap
        // A quarter-turn cycle [2,11,6,10] applied twice:
        // slot 10 goes to 11 goes back to 10, slot 11 goes to 10 goes back to 11
        // Net effect on belt: local indices 2 and 3 swap
        solver_move_t::R2 => {
            perm.swap(2, 3); // local 2 (slot 10) <-> local 3 (slot 11)
        }

        // L2: belt slots 8(local 0) and 9(local 1) swap
        // Quarter-turn cycle [1,9,5,8] applied twice:
        // Net effect on belt: local indices 0 and 1 swap
        solver_move_t::L2 => {
            perm.swap(0, 1); // local 0 (slot 8) <-> local 1 (slot 9)
        }

        // F2: belt slots 9(local 1) and 10(local 2) swap
        // Quarter-turn cycle [3,10,4,9] applied twice:
        // Net effect on belt: local indices 1 and 2 swap
        solver_move_t::F2 => {
            perm.swap(1, 2); // local 1 (slot 9) <-> local 2 (slot 10)
        }

        // B2: belt slots 8(local 0) and 11(local 3) swap
        // Quarter-turn cycle [0,8,7,11] applied twice:
        // Net effect on belt: local indices 0 and 3 swap
        solver_move_t::B2 => {
            perm.swap(0, 3); // local 0 (slot 8) <-> local 3 (slot 11)
        }

        // Quarter turns of R, L, F, B are NOT Phase 2 moves — they would
        // kick belt edges out of the belt entirely, making this coord
        // undefined. They must never be passed to this function.
        _ => { /* not a Phase 2 move — should never be called */ }
    }
}

// Encodes a [u8; 4] belt permutation using the Lehmer code.
// Identical structure to encode_cperm but over 4 elements.
// Returns a value in [0, 23] = [0, 4!-1].
fn encode_udslice_perm(perm: &[u8; 4]) -> usize {
    // 0!=1, 1!=1, 2!=2, 3!=6
    const FACTORIAL: [usize; 4] = [1, 1, 2, 6];
    let mut coord = 0usize;
    for i in 0..4 {
        // Count how many elements to the right of position i
        // have a smaller value than perm[i]
        let smaller = ((i + 1)..4)
            .filter(|&j| perm[j] < perm[i])
            .count();
        // Weight by (3-i)!: position 0 -> 3!, position 1 -> 2!, etc.
        coord += smaller * FACTORIAL[3 - i];
    }
    coord
}

// BFS over all 24 UD-slice permutation states.
// Only Phase 2 moves are applied — using all 18 would be wrong because
// quarter turns of R/L/F/B displace belt edges into non-belt slots,
// making the coord undefined and the BFS transitions nonsensical.
fn bfs_udslice_perm() -> [u8; 24] {
    // 24 = 4! possible orderings of the 4 belt edges
    let mut table = [u8::MAX; 24];
    let mut queue: VecDeque<[u8; 4]> = VecDeque::new();

    // Solved state: belt cubies 0,1,2,3 in belt slots 0,1,2,3 (local indices)
    // This corresponds to absolute slots 8,9,10,11 holding cubies 8,9,10,11
    let start: [u8; 4] = [0, 1, 2, 3];

    // Solved permutation = Lehmer code 0 (identity permutation)
    table[encode_udslice_perm(&start)] = 0;
    queue.push_back(start);

    // Only iterate over Phase 2 moves — this is the critical difference
    // from the other BFS functions which use ALL_MOVES
    const PHASE2_MOVES: [solver_move_t; 10] = [
        solver_move_t::U,  solver_move_t::Ui, solver_move_t::U2,
        solver_move_t::D,  solver_move_t::Di, solver_move_t::D2,
        solver_move_t::R2, solver_move_t::L2,
        solver_move_t::F2, solver_move_t::B2,
    ];

    while let Some(perm) = queue.pop_front() {
        let dist = table[encode_udslice_perm(&perm)];
        for &m in PHASE2_MOVES.iter() {
            let mut next = perm;
            apply_udslice_perm_move(&mut next, m);
            let coord = encode_udslice_perm(&next);
            if table[coord] == u8::MAX {
                table[coord] = dist + 1;
                queue.push_back(next);
            }
        }
    }

    assert!(!table.contains(&u8::MAX),
        "BFS left unreachable udslice perm states!");
    assert_eq!(table[0], 0, "Solved state must have distance 0");
    println!("// Max distance: {}", table.iter().max().unwrap());
    println!("// States at each depth:");
    for d in 0..=*table.iter().max().unwrap() {
        println!("//   depth {}: {} states",
            d, table.iter().filter(|&&v| v == d).count());
    }

    table
}


// ============================================================
// EDGE PERMUTATION TABLE (Phase 2)
// ============================================================
//
// WHAT ARE WE TRACKING?
// The permutation of the 8 non-belt edges across slots 0-7.
// Phase 1 guarantees belt edges are in slots 8-11, so the 8
// non-belt edges (cubies 0-7) must occupy exactly slots 0-7.
// Their arrangement is a permutation of {0..7}.
//
// STATE REPRESENTATION:
// [u8; 8] where entry s = which non-belt cubie (0-7) sits in slot s.
// On a solved cube this is [0, 1, 2, 3, 4, 5, 6, 7].
//
// ENCODING:
// Same Lehmer code as corner_perm_coord / bfs_corner_perm.
// Returns a value in [0, 40319] = [0, 8!-1].
//
// MOVE SET:
// Phase 2 moves only. Quarter turns of R/L/F/B would move non-belt
// edges into belt slots and belt edges into non-belt slots, making
// this coord undefined. Double moves (R2, L2, F2, B2) only swap
// non-belt edges among non-belt slots, keeping the coord well-defined.

// Applies one 4-cycle to the non-belt edge permutation array.
// slots are indices into the [u8; 8] array (global slot indices 0-7).
fn cycle4_eperm(perm: &mut [u8; 8], slots: [usize; 4]) {
    // Save all four values before writing — same pattern as all other cycle4 functions
    let (p0, p1, p2, p3) = (
        perm[slots[0]],
        perm[slots[1]],
        perm[slots[2]],
        perm[slots[3]],
    );
    perm[slots[1]] = p0;
    perm[slots[2]] = p1;
    perm[slots[3]] = p2;
    perm[slots[0]] = p3;
}

// Applies a Phase 2 move to the non-belt edge permutation.
//
// For each Phase 2 move we need the cycles RESTRICTED to slots 0-7
// (non-belt slots). We derive these from the full edge cycles in lib.rs:
//
// U:  full cycle [0, 2, 3, 1]   — all non-belt, keep as-is
// D:  full cycle [4, 6, 7, 5]   — all non-belt, keep as-is
// R2: full cycle [2, 11, 6, 10] — slots 11,10 are belt, skip them
//     applied twice to non-belt: slots 2 and 6 swap
// L2: full cycle [1, 9, 5, 8]   — slots 9,8 are belt, skip them
//     applied twice to non-belt: slots 1 and 5 swap
// F2: full cycle [3, 10, 4, 9]  — slots 10,9 are belt, skip them
//     applied twice to non-belt: slots 3 and 4 swap
// B2: full cycle [0, 8, 7, 11]  — slots 8,11 are belt, skip them
//     applied twice to non-belt: slots 0 and 7 swap
//
// For U/D quarter turns: a 4-cycle applied once is a full 4-cycle.
// For X2 moves: a 4-cycle applied twice with two belt slots means
// the two non-belt slots in that cycle simply swap.
fn apply_eperm_move(perm: &mut [u8; 8], m: solver_move_t) {
    match m {
        // U quarter turn: full 4-cycle over non-belt slots [0,2,3,1]
        solver_move_t::U  => cycle4_eperm(perm, [0, 2, 3, 1]),
        // Ui = three U quarter turns
        solver_move_t::Ui => {
            for _ in 0..3 { cycle4_eperm(perm, [0, 2, 3, 1]); }
        }
        // U2 = two U quarter turns
        solver_move_t::U2 => {
            for _ in 0..2 { cycle4_eperm(perm, [0, 2, 3, 1]); }
        }

        // D quarter turn: full 4-cycle over non-belt slots [4,6,7,5]
        solver_move_t::D  => cycle4_eperm(perm, [4, 6, 7, 5]),
        solver_move_t::Di => {
            for _ in 0..3 { cycle4_eperm(perm, [4, 6, 7, 5]); }
        }
        solver_move_t::D2 => {
            for _ in 0..2 { cycle4_eperm(perm, [4, 6, 7, 5]); }
        }

        // R2: non-belt slots 2 and 6 swap
        // Derivation: full cycle [2,11,6,10], applied twice.
        // Belt slots 11,10 stay in belt. Non-belt slots 2,6:
        // after 1st application: 2->11(belt, ignore), 6->2
        // after 2nd application: 2->6 effectively
        // Net: slots 2 and 6 exchange their cubies
        solver_move_t::R2 => { perm.swap(2, 6); }

        // L2: non-belt slots 1 and 5 swap
        // Derivation: full cycle [1,9,5,8], applied twice.
        // Belt slots 9,8 stay in belt. Non-belt slots 1,5 swap.
        solver_move_t::L2 => { perm.swap(1, 5); }

        // F2: non-belt slots 3 and 4 swap
        // Derivation: full cycle [3,10,4,9], applied twice.
        // Belt slots 10,9 stay in belt. Non-belt slots 3,4 swap.
        solver_move_t::F2 => { perm.swap(3, 4); }

        // B2: non-belt slots 0 and 7 swap
        // Derivation: full cycle [0,8,7,11], applied twice.
        // Belt slots 8,11 stay in belt. Non-belt slots 0,7 swap.
        solver_move_t::B2 => { perm.swap(0, 7); }

        // Quarter turns of R/L/F/B are not Phase 2 moves
        _ => { /* should never be called */ }
    }
}

// Encodes a [u8; 8] non-belt edge permutation using the Lehmer code.
// Identical to encode_cperm — same structure, same encoding, same range.
// Returns a value in [0, 40319] = [0, 8!-1].
fn encode_eperm(perm: &[u8; 8]) -> usize {
    const FACTORIAL: [usize; 8] = [1, 1, 2, 6, 24, 120, 720, 5040];
    let mut coord = 0usize;
    for i in 0..8 {
        // Count elements to the right with smaller value = Lehmer digit at i
        let smaller = ((i + 1)..8)
            .filter(|&j| perm[j] < perm[i])
            .count();
        // Weight by (7-i)!
        coord += smaller * FACTORIAL[7 - i];
    }
    coord
}

// BFS over all 40320 non-belt edge permutation states.
// Uses Phase 2 moves only — same reason as bfs_udslice_perm.
// This is the most expensive of the Phase 2 BFS runs:
// 40320 states × 10 moves each, still runs in well under a second on PC.
fn bfs_edge_perm() -> [u8; 40320] {
    let mut table = [u8::MAX; 40320];
    let mut queue: VecDeque<[u8; 8]> = VecDeque::new();

    // Solved state: non-belt cubie i sits in slot i for all i in 0..7
    let start: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    table[encode_eperm(&start)] = 0;
    queue.push_back(start);

    const PHASE2_MOVES: [solver_move_t; 10] = [
        solver_move_t::U,  solver_move_t::Ui, solver_move_t::U2,
        solver_move_t::D,  solver_move_t::Di, solver_move_t::D2,
        solver_move_t::R2, solver_move_t::L2,
        solver_move_t::F2, solver_move_t::B2,
    ];

    while let Some(perm) = queue.pop_front() {
        let dist = table[encode_eperm(&perm)];
        for &m in PHASE2_MOVES.iter() {
            let mut next = perm;
            apply_eperm_move(&mut next, m);
            let coord = encode_eperm(&next);
            if table[coord] == u8::MAX {
                table[coord] = dist + 1;
                queue.push_back(next);
            }
        }
    }

    assert!(!table.contains(&u8::MAX),
        "BFS left unreachable edge perm states!");
    assert_eq!(table[0], 0, "Solved state must have distance 0");
    println!("// Max distance: {}", table.iter().max().unwrap());
    println!("// States at each depth:");
    for d in 0..=*table.iter().max().unwrap() {
        println!("//   depth {}: {} states",
            d, table.iter().filter(|&&v| v == d).count());
    }

    table
}
*/



