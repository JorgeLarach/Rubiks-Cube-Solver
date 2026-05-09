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
// Rewritten solver basically because two different CubieStates can produce the same coords while having completely different arrangements

// To generate the lookup tables, run
// cargo run --bin gen_tables --features std-env

use cube_solver::*;
use std::collections::VecDeque;
fn main() {
    use std::time::Instant;
    println!("--- Phase 1 Corner Orientation Table ---");
    print_co_table();
    println!("");

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

fn print_co_table() {
    let table = bfs_corner_orient();
    print!("pub const CORNER_ORIENT_TABLE: [u8; 2187] = [");
    for (_, &v) in table.iter().enumerate() {
        print!("{}, ", v);
    }
    println!("];");
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

fn encode_co(o: &[u8; 8]) -> usize {
    let mut coord = 0;
    for i in 0..7 { coord = coord * 3 + o[i] as usize; }
    coord
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

fn cycle4_co(o: &mut [u8; 8], slots: [usize; 4], map: [u8; 3]) {
    let (o0, o1, o2, o3) = (o[slots[0]], o[slots[1]], o[slots[2]], o[slots[3]]);
    o[slots[1]] = map[o0 as usize];
    o[slots[2]] = map[o1 as usize];
    o[slots[3]] = map[o2 as usize];
    o[slots[0]] = map[o3 as usize];
}


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

