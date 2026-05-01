//
// gen_tables.rs
//
//  Created on: April 25, 2026
//      Author: jorgelarach
//   Co-Author: Claude
//

// Claude was a significant contributor to this.
// This program generates all six lookup tables used by the solver
// Generates the tables based on a solved cube, and uses BFS to traverse all states
// Depth of BFS corresponds to moves from current state to root (solved state)
// Rewritten solver basically because two different CubieStates can produce the same udslice_perm_coord while having completely different arrangements
use cube_solver::*;
use std::collections::VecDeque;
fn main() {
    generate_corner_orientation_table();
    generate_edge_orientation_table();
    generate_ud_slice_table();
    generate_corner_permutation_table();
    generate_udslice_permutation_table();
    generate_edge_permutation_table();
}

fn generate_corner_orientation_table() {
    let table = bfs_corner_orient();
    print!("pub const CORNER_ORIENT_TABLE: [u8; 2187] = [");
    for (_, &v) in table.iter().enumerate() {
        print!("{}, ", v);
    }
    println!("];");
}

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