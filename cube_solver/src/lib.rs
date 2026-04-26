//
// lib.rs
//
//  Created on: Jan 23, 2026
//      Author: jorgelarach
//

#![cfg_attr(not(feature = "std-env"), no_std)]

mod tables;
use core::slice;

#[cfg(not(feature = "std-env"))]
use core::panic::PanicInfo;

#[repr(C)] // Lays out this enum/struct in memory exactly like C would
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum solver_move_t {
    U, Ui, U2,
    D, Di, D2,
    L, Li, L2,
    R, Ri, R2,
    F, Fi, F2,
    B, Bi, B2
}

pub const U_FACE: usize = 0;
pub const D_FACE: usize = 1;
pub const L_FACE: usize = 2;
pub const R_FACE: usize = 3;
pub const F_FACE: usize = 4;
pub const B_FACE: usize = 5;

pub const WHITE:  u8 = 0;
pub const YELLOW: u8 = 1;
pub const GREEN:  u8 = 2;
pub const BLUE:   u8 = 3;
pub const RED:    u8 = 4;
pub const ORANGE: u8 = 5;

/* Constants */
const CORNER_CUBIE_COORDINATES: [(usize, usize, usize); 8] = [
    // There are 8 corner cubies in the cube
    // Each face's corner cubies are located at [0, 2, 6, 8]
    // Each one is either on the U_FACE or the D_FACE
    
    (U_FACE * 9 + 0, B_FACE * 9 + 2, L_FACE * 9 + 0), // 0. UBL
    (U_FACE * 9 + 2, B_FACE * 9 + 0, R_FACE * 9 + 2), // 1. UBR
    (U_FACE * 9 + 6, F_FACE * 9 + 0, L_FACE * 9 + 2), // 2. UFL
    (U_FACE * 9 + 8, F_FACE * 9 + 2, R_FACE * 9 + 0), // 3. UFR

    (D_FACE * 9 + 0, F_FACE * 9 + 6, L_FACE * 9 + 8), // 4. DFL
    (D_FACE * 9 + 2, F_FACE * 9 + 8, R_FACE * 9 + 6), // 5. DFR
    (D_FACE * 9 + 6, B_FACE * 9 + 8, L_FACE * 9 + 6), // 6. DBL
    (D_FACE * 9 + 8, B_FACE * 9 + 6, R_FACE * 9 + 8)  // 7. DBR
];

const EDGE_CUBIE_COORDINATES: [(usize, usize); 12] = [
    // There are 12 edge stickers in the cube (four on each face). 
    // Each face's edge stickers are located at [1, 3, 5, 7]

    (U_FACE * 9 + 1, B_FACE * 9 + 1), // 0. UB
    (U_FACE * 9 + 3, L_FACE * 9 + 1), // 1. UL 
    (U_FACE * 9 + 5, R_FACE * 9 + 1), // 2. UR 
    (U_FACE * 9 + 7, F_FACE * 9 + 1), // 3. UF 
    (D_FACE * 9 + 1, F_FACE * 9 + 7), // 4. DF 
    (D_FACE * 9 + 3, L_FACE * 9 + 7), // 5. DL 
    (D_FACE * 9 + 5, R_FACE * 9 + 7), // 6. DR 
    (D_FACE * 9 + 7, B_FACE * 9 + 7), // 7. DB 
    (L_FACE * 9 + 3, B_FACE * 9 + 5), // 8. LB 
    (L_FACE * 9 + 5, F_FACE * 9 + 3), // 9. LF 
    (R_FACE * 9 + 3, F_FACE * 9 + 5), // 10. RF 
    (R_FACE * 9 + 5, B_FACE * 9 + 3)  // 11. RB 
];

const CORNER_CUBIE_COLORS: [[u8; 3]; 8] = [
    // Colors of corners on solved cube
    [WHITE,  GREEN,  ORANGE],  // cubie 0 (UBL)
    [WHITE,  ORANGE, BLUE  ],  // cubie 1 (UBR)
    [WHITE,  RED,    GREEN ],  // cubie 2 (UFL)
    [WHITE,  BLUE,   RED   ],  // cubie 3 (UFR)
    [YELLOW, RED,    GREEN ],  // cubie 4 (DFL)
    [YELLOW, RED,    BLUE  ],  // cubie 5 (DFR)
    [YELLOW, ORANGE, GREEN ],  // cubie 6 (DBL)
    [YELLOW, BLUE,   ORANGE],  // cubie 7 (DBR)
];

const EDGE_CUBIE_COLORS: [[u8; 2]; 12] = [
    // Colors of edges on solved cube
    [WHITE,  ORANGE],  // cubie 0  (UB)
    [WHITE,  GREEN ],  // cubie 1  (UL)
    [WHITE,  BLUE  ],  // cubie 2  (UR)
    [WHITE,  RED   ],  // cubie 3  (UF)
    [YELLOW, RED   ],  // cubie 4  (DF)
    [YELLOW, GREEN ],  // cubie 5  (DL)
    [YELLOW, BLUE  ],  // cubie 6  (DR)
    [YELLOW, ORANGE],  // cubie 7  (DB)
    [GREEN,  ORANGE],  // cubie 8  (LB)
    [GREEN,  RED   ],  // cubie 9  (LF)
    [BLUE,   RED   ],  // cubie 10 (RF)
    [BLUE,   ORANGE],  // cubie 11 (RB)
];

pub const SOLVED_CUBE_STICKERS:[u8; 54] = 
   [0,0,0,0,0,0,0,0,0,
    1,1,1,1,1,1,1,1,1,
    2,2,2,2,2,2,2,2,2,
    3,3,3,3,3,3,3,3,3,
    4,4,4,4,4,4,4,4,4,
    5,5,5,5,5,5,5,5,5];

pub const ALL_MOVES: [solver_move_t; 18] = [
    solver_move_t::U,  solver_move_t::Ui, solver_move_t::U2,
    solver_move_t::D,  solver_move_t::Di, solver_move_t::D2,
    solver_move_t::L,  solver_move_t::Li, solver_move_t::L2,
    solver_move_t::R,  solver_move_t::Ri, solver_move_t::R2,
    solver_move_t::F,  solver_move_t::Fi, solver_move_t::F2,
    solver_move_t::B,  solver_move_t::Bi, solver_move_t::B2,
];
/* Data Structures */
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CornerState {
    pub position: u8,    // which of the 8 corner slots this cubie is sitting in (0–7)
    pub orientation: u8, // 0, 1, or 2
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EdgeState {
    pub position: u8,  // which of the 12 edge slots this cubie is sitting in (0–11)
    pub flipped: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CubieState {
    pub corners: [CornerState; 8],  // corners[i] = state of corner cubie i
    pub edges:   [EdgeState;  12],  // edges[i]   = state of edge cubie i
}

/* Helper Functions */
pub fn convert_to_cubie(stickers: [u8; 54]) -> CubieState {
    let mut cube = CubieState {
        corners: [CornerState {position: 0, orientation: 0}; 8],
        edges: [EdgeState {position: 0, flipped: false}; 12],
    };

    // Populate corners array
    for slot in 0..8 {
        let idxs = CORNER_CUBIE_COORDINATES[slot]; // Returns a triple of sticker indices

        // Track the three colors for current corner cubie
        let colors = [
            stickers[idxs.0],
            stickers[idxs.1],
            stickers[idxs.2]
        ];

        // cubie_id identifies which corner cubie this is based on its colors
        let cubie_id = (0..8).find(|&c| {
            // Grabs the known canonical colors for candidate cubie c
            // E.g. if c is 0, returns [WHITE, BLUE, RED]
            let corner_colors = CORNER_CUBIE_COLORS[c]; 

            // Ignores order, just matches candidate cubie c with existing corner cubie
            colors.iter().all(|col| corner_colors.contains(col))
        }).expect("Invalid cube: no matching corner cubie");

        let ud_color = CORNER_CUBIE_COLORS[cubie_id][0]; // First element of triple is always White or Yellow

        // .position() is like .find() but returns the index instead of the value
        // If the UD color is at index 0 -> it's sitting on the U/D face where it belongs -> orientation 0
        // If the UD color is at index 1 -> it's been twisted once                        -> orientation 1
        // If the UD color is at index 2 -> it's been twisted twice                       -> orientation 2
        let orientation = colors.iter().position(|&c| c == ud_color).unwrap() as u8;

        cube.corners[cubie_id] = CornerState { position: slot as u8, orientation};
    }

    // Populate edges array
    for slot in 0..12 {
        let idxs = EDGE_CUBIE_COORDINATES[slot];

        let colors = [
            stickers[idxs.0],
            stickers[idxs.1]
        ];

        let cubie_id = (0..12).find(|&c| {
            let edge_colors = EDGE_CUBIE_COLORS[c];
            colors.iter().all(|col| edge_colors.contains(col))
        }).expect("Invaid cube: no matching edge cubie");

        // Flipped if the cubie's primary color is at the secondary sticker position
        let flipped = colors[0] != EDGE_CUBIE_COLORS[cubie_id][0];

        cube.edges[cubie_id] = EdgeState {
            position: slot as u8,
            flipped
        };
    }

    cube
}

pub fn inverse_move(input_move: solver_move_t) -> solver_move_t{
    match input_move {
        solver_move_t::U => solver_move_t::Ui,
        solver_move_t::Ui => solver_move_t::U,

        solver_move_t::D => solver_move_t::Di,
        solver_move_t::Di => solver_move_t::D,

        solver_move_t::L => solver_move_t::Li,
        solver_move_t::Li => solver_move_t::L,

        solver_move_t::R => solver_move_t::Ri,
        solver_move_t::Ri => solver_move_t::R,

        solver_move_t::F => solver_move_t::Fi,
        solver_move_t::Fi => solver_move_t::F,

        solver_move_t::B => solver_move_t::Bi,
        solver_move_t::Bi => solver_move_t::B,

        _ => input_move
    }
}

// BINOMIAL COEFFICIENT HELPER
// Computes C(n, k) = n! / (k! * (n-k)!) — "n choose k"
// Used by udslice_coord to rank combinations.
// const fn means this can be evaluated at compile time.
pub const fn choose(n: usize, k: usize) -> usize {
    if k > n { return 0; }
    match (n, k) {
        (_, 0) => 1, // C(n,0) = 1: there's exactly one way to choose nothing
        (0, _) => 0, // C(0,k>0) = 0: can't choose k>0 items from empty set
        // Pascal's triangle recurrence: C(n,k) = C(n-1,k-1) + C(n-1,k)
        _ => choose(n - 1, k - 1) + choose(n - 1, k)
    }
}

impl CubieState {
    pub fn make_solved() -> CubieState {
        convert_to_cubie(SOLVED_CUBE_STICKERS)
    }

    pub fn is_solved(&self) -> bool {
        for i in 0..8 {
            if self.corners[i].position != i as u8 { return false; }
            if self.corners[i].orientation != 0     { return false; }
        }
        for i in 0..12 {
            if self.edges[i].position != i as u8 { return false; }
            if self.edges[i].flipped              { return false; }
        }
        true
    }

    /* State Space Compression */

    // CORNER ORIENTATION COORDINATE
    // Range: 0..2187 (= 3^7)
    //
    // Each of the 8 corners can be twisted into 3 orientations (0, 1, 2).
    // However, on any legally scrambled cube, the 8th corner's orientation
    // is completely determined by the other 7 — they must sum to 0 mod 3.
    // This is a physical invariant of the cube; you cannot twist one corner
    // without affecting others. So we only need to encode 7 corners, giving
    // 3^7 = 2187 possible values.
    //
    // We encode it as a base-3 number: the orientation of corners[0] is the
    // most significant digit, corners[6] is the least significant.
    // Solved state = all orientations 0 = coord 0.
    pub fn corner_orient_coord(&self) -> usize {
        let mut slot_orients = [0u8; 8];
        for i in 0..8 {
            slot_orients[self.corners[i].position as usize] = self.corners[i].orientation;
        }

        let mut coord = 0usize;
        // coord = Σ(i = 0 to 6) oi * 3^(6-i)
        // Horner's Method of polynomial evaluation (below) https://en.wikipedia.org/wiki/Horner%27s_method is exactly the same as above
        for s in 0..7 {
            coord = coord * 3 + slot_orients[s] as usize;
        }
        coord
    }

    // EDGE ORIENTATION COORDINATE
    // Range: 0..2048 (= 2^11)
    //
    // Each of the 12 edges can be in one of 2 orientations: correct (false) or
    // flipped (true). Same physical invariant as corners: on a legal cube, the
    // sum of all edge flip states is always even, meaning the 12th edge's flip
    // state is determined by the other 11. So we encode only 11 edges, giving
    // 2^11 = 2048 possible values.
    //
    // Encoded as a base-2 (binary) number: edges[0] is the most significant bit,
    // edges[10] is the least significant bit.
    // Solved state = all unflipped = coord 0.

    pub fn edge_orient_coord(&self) -> usize {
        let mut slot_flips = [0u8; 12];
        for i in 0..12 {
            slot_flips[self.edges[i].position as usize] = self.edges[i].flipped as u8;
        }


        let mut coord = 0usize;
        // coord = Σ(i = 0 to 10) ei * 2^(10-i)
        // Horner's method of polynomial evaluation (below) is the same as above
        for s in 0..11 {
            // Shift left one binary digit, add 1 if flipped, 0 if not
            coord = coord * 2 + slot_flips[s] as usize;
        }
        coord
    }

    // UDSLICE COORDINATE
    // Range: 0..495 (= C(12,4))
    //
    // WHAT IS THE UD-SLICE?
    // The cube has three "layers" of edges when viewed from the side:
    //   - U-layer edges: UB(0), UL(1), UR(2), UF(3) — adjacent to the U face
    //   - D-layer edges: DF(4), DL(5), DR(6), DB(7) — adjacent to the D face
    //   - UD-slice edges: LB(8), LF(9), RF(10), RB(11) — the "equatorial belt"
    //     running between the U and D layers, around the middle of the cube
    //
    // The UD-slice edges are special: on a solved cube they occupy exactly the
    // 4 "equatorial" slots (slots 8-11). During a scramble they get displaced
    // into other slots. This coordinate asks: which 4 of the 12 edge slots
    // are currently occupied by UD-slice edges, regardless of their order?
    //
    // The number of ways to choose 4 slots out of 12 is C(12,4) = 495.
    // We assign each such combination a unique number 0..494 using a standard
    // combinatorial ranking formula. Solved state = UD-slice edges in slots
    // 8,9,10,11 = coord 0.
    pub fn udslice_coord(&self) -> usize {
        // Step 1: Mark which slots currently contain a UD-slice edge.
        // UD-slice edges are cubie identities 8, 9, 10, 11.
        // Identify which positions (0 to 11) are occupied by a UD-edge
        let mut occupied = [false; 12];
        for i in 8..12 {
            occupied[self.edges[i].position as usize] = true;
        }

        // Step 2: Compute the combinatorial rank of this 4-element subset.
        // This is a standard algorithm that maps any combination of 4 chosen
        // slots out of 12 to a unique integer in 0..495.
        // We walk slots from high to low, tracking how many UD-slice edges
        // we've "seen" so far (k), and accumulating C(i,k) for each
        // unoccupied slot below the current count of occupied slots.
        let mut coord = 0usize;
        let mut k = 3usize; // counts down from 3 to 0 as we find occupied slots
        for i in (0..12).rev() {
            if occupied[i] {
                // This slot has a UD-slice edge — decrement k and move on
                if k == 0 { break; }
                k -= 1;
            } else {
                // This slot does NOT have a UD-slice edge — add C(i,k) to rank
                coord += choose(i, k);
            }
        }
        coord
    }

    // CORNER PERMUTATION COORDINATE
    // Range: 0..40320 (= 8!)
    //
    // This coordinate encodes WHICH slot each corner cubie is sitting in,
    // completely ignoring orientation. There are 8! = 40,320 possible
    // permutations of 8 corners across 8 slots.
    //
    // We use the Lehmer code (factoriadic encoding): https://en.wikipedia.org/wiki/Lehmer_code 
    // For each corner i (left to right), count how many corners to its RIGHT
    // have a smaller position value. Multiply that count by (7-i)! and sum.
    //
    // Example: if corners are in order [0,1,2,3,4,5,6,7] (solved),
    // every corner has 0 corners to its right with smaller position,
    // so coord = 0. Any permutation maps to a unique number 0..40319.
    pub fn corner_perm_coord(&self) -> usize {
        // Precomputed factorials 0! through 7!
        const FACTORIAL: [usize; 8] = [1, 1, 2, 6, 24, 120, 720, 5040];
        let mut coord = 0usize;
        for i in 0..8 {
            // Count how many cubies at positions AFTER i have a smaller position value
            // than the cubie currently at index i
            let smaller = ((i + 1)..8)
                .filter(|&j| self.corners[j].position < self.corners[i].position)
                .count();
            // Each such count contributes smaller * (7-i)! to the Lehmer code
            coord += smaller * FACTORIAL[7 - i];
        }
        coord
    }



    /* Rotation Functions  */


    // Orientation maps derived from CORNER_CUBIE_COORDINATES sticker ordering [UD, FB, LR]
    // Each move swaps the two sticker positions NOT on its own axis
    const ORIENT_UD: [u8; 3] = [0, 2, 1]; // U, D: swap FB(1) and LR(2)
    const ORIENT_LR: [u8; 3] = [1, 0, 2]; // R, L: swap UD(0) and FB(1)
    const ORIENT_FB: [u8; 3] = [2, 1, 0]; // F, B: swap UD(0) and LR(2)

    fn find_corner_at(&self, slot: u8) -> usize {
        (0..8).find(|&i| self.corners[i].position == slot)
            .expect("no corner found in slot")
    }

    fn find_edge_at(&self, slot: u8) -> usize {
        (0..12).find(|&i| self.edges[i].position == slot)
            .expect("no edge found in slot")
    }

    // Cycles 4 corners: cubie at slots[i] moves to slots[(i+1) % 4]
    // Applies orientation map to each corner as it moves
    fn cycle_corners(&mut self, slots: [u8; 4], map: [u8; 3]) {
        let ids = [
            self.find_corner_at(slots[0]),
            self.find_corner_at(slots[1]),
            self.find_corner_at(slots[2]),
            self.find_corner_at(slots[3]),
        ];
        // Save all orientations before modifying anything
        let or = [
            self.corners[ids[0]].orientation,
            self.corners[ids[1]].orientation,
            self.corners[ids[2]].orientation,
            self.corners[ids[3]].orientation,
        ];
        self.corners[ids[0]].position    = slots[1];
        self.corners[ids[0]].orientation = map[or[0] as usize];
        self.corners[ids[1]].position    = slots[2];
        self.corners[ids[1]].orientation = map[or[1] as usize];
        self.corners[ids[2]].position    = slots[3];
        self.corners[ids[2]].orientation = map[or[2] as usize];
        self.corners[ids[3]].position    = slots[0];
        self.corners[ids[3]].orientation = map[or[3] as usize];
    }

    // Cycles 4 edges: cubie at slots[i] moves to slots[(i+1) % 4]
    // flip=true means all 4 edges toggle their flip state (XOR)
    fn cycle_edges(&mut self, slots: [u8; 4], flip: bool) {
        let ids = [
            self.find_edge_at(slots[0]),
            self.find_edge_at(slots[1]),
            self.find_edge_at(slots[2]),
            self.find_edge_at(slots[3]),
        ];
        let fl = [
            self.edges[ids[0]].flipped,
            self.edges[ids[1]].flipped,
            self.edges[ids[2]].flipped,
            self.edges[ids[3]].flipped,
        ];
        self.edges[ids[0]].position = slots[1];
        self.edges[ids[0]].flipped  = fl[0] ^ flip;
        self.edges[ids[1]].position = slots[2];
        self.edges[ids[1]].flipped  = fl[1] ^ flip;
        self.edges[ids[2]].position = slots[3];
        self.edges[ids[2]].flipped  = fl[2] ^ flip;
        self.edges[ids[3]].position = slots[0];
        self.edges[ids[3]].flipped  = fl[3] ^ flip;
    }

    fn u(&mut self) {
        // Corners: UFR(3) -> UFL(2) -> UBL(0) -> UBR(1)
        self.cycle_corners([3, 2, 0, 1], Self::ORIENT_UD);
        // Edges:   UB(0) -> UR(2) -> UF(3) -> UL(1)  — no flips
        self.cycle_edges([0, 2, 3, 1], false);
    }

    fn d(&mut self) {
        // Corners: DFL(4) -> DFR(5) -> DBR(7) -> DBL(6)
        self.cycle_corners([4, 5, 7, 6], Self::ORIENT_UD);
        // Edges:   DF(4) -> DR(6) -> DB(7) -> DL(5)  — no flips
        self.cycle_edges([4, 6, 7, 5], false);
    }

    fn r(&mut self) {
        // Corners: UFR(3) -> UBR(1) -> DBR(7) -> DFR(5)
        self.cycle_corners([3, 1, 7, 5], Self::ORIENT_LR);
        // Edges:   UR(2) -> RB(11) -> DR(6) -> RF(10)  — all flip
        self.cycle_edges([2, 11, 6, 10], true);
    }

    fn l(&mut self) {
        // Corners: UBL(0) -> UFL(2) -> DFL(4) -> DBL(6)
        self.cycle_corners([0, 2, 4, 6], Self::ORIENT_LR);
        // Edges:   UL(1) -> LF(9) -> DL(5) -> LB(8)  — all flip
        self.cycle_edges([1, 9, 5, 8], true);
    }

    fn f(&mut self) {
        // Corners: UFL(2) -> UFR(3) -> DFR(5) -> DFL(4)
        self.cycle_corners([2, 3, 5, 4], Self::ORIENT_FB);
        // Edges:   UF(3) -> RF(10) -> DF(4) -> LF(9)  — no flips
        self.cycle_edges([3, 10, 4, 9], false);
    }

    fn b(&mut self) {
        // Corners: UBL(0) -> DBL(6) -> DBR(7) -> UBR(1)
        self.cycle_corners([0, 6, 7, 1], Self::ORIENT_FB);
        // Edges:   UB(0) -> LB(8) -> DB(7) -> RB(11)  — no flips
        self.cycle_edges([0, 8, 7, 11], false);
    }

    pub fn apply_move(&mut self, m: solver_move_t) {
        match m {
            solver_move_t::U  => {self.u();}
            solver_move_t::Ui => {self.u(); self.u(); self.u();}
            solver_move_t::U2 => {self.u(); self.u();}

            solver_move_t::D  => {self.d();}
            solver_move_t::Di => {self.d();self.d();self.d();}
            solver_move_t::D2 => {self.d(); self.d();}

            solver_move_t::L  => {self.l();}
            solver_move_t::Li => {self.l();self.l();self.l();}
            solver_move_t::L2 => {self.l(); self.l();}

            solver_move_t::R  => {self.r();}
            solver_move_t::Ri => {self.r();self.r();self.r();}
            solver_move_t::R2 => {self.r(); self.r();}

            solver_move_t::F  => {self.f();}
            solver_move_t::Fi => {self.f();self.f();self.f();}
            solver_move_t::F2 => {self.f(); self.f();}

            solver_move_t::B  => {self.b();}
            solver_move_t::Bi => {self.b();self.b();self.b();}
            solver_move_t::B2 => {self.b(); self.b();}
        }
    }
}


fn solve_internal(_cube: CubieState, out: &mut [solver_move_t]) -> usize{
    // out[0]  = solver_move_t::U;
    // out[1]  = solver_move_t::D;
    // out[2]  = solver_move_t::L;
    // out[3]  = solver_move_t::R;
    // out[4]  = solver_move_t::F;
    // out[5]  = solver_move_t::B;

    // out[6] = solver_move_t::Bi;
    // out[7] = solver_move_t::Fi;
    // out[8] = solver_move_t::Ri;
    // out[9] = solver_move_t::Li;
    // out[10] = solver_move_t::Di;
    // out[11] = solver_move_t::Ui;

    // 12

    out[0]  = solver_move_t::U;
    out[1]  = solver_move_t::D;
    out[2]  = solver_move_t::L;
    out[3]  = solver_move_t::R;
    out[4]  = solver_move_t::F;
    out[5]  = solver_move_t::B;

    out[6]  = solver_move_t::U2;
    out[7]  = solver_move_t::D2;
    out[8]  = solver_move_t::L2;
    out[9]  = solver_move_t::R2;
    out[10] = solver_move_t::F2;
    out[11] = solver_move_t::B2;

    out[12] = solver_move_t::Ui;
    out[13] = solver_move_t::Di;
    out[14] = solver_move_t::Li;
    out[15] = solver_move_t::Ri;
    out[16] = solver_move_t::Fi;
    out[17] = solver_move_t::Bi;

    // ***** inverse starts here *****

    out[18] = solver_move_t::B;
    out[19] = solver_move_t::F;
    out[20] = solver_move_t::R;
    out[21] = solver_move_t::L;
    out[22] = solver_move_t::D;
    out[23] = solver_move_t::U;

    out[24] = solver_move_t::B2;
    out[25] = solver_move_t::F2;
    out[26] = solver_move_t::R2;
    out[27] = solver_move_t::L2;
    out[28] = solver_move_t::D2;
    out[29] = solver_move_t::U2;

    out[30] = solver_move_t::Bi;
    out[31] = solver_move_t::Fi;
    out[32] = solver_move_t::Ri;
    out[33] = solver_move_t::Li;
    out[34] = solver_move_t::Di;
    out[35] = solver_move_t::Ui;
 
    36
}

#[unsafe(no_mangle)] // Prevents function renaming (mangling) during compiling. C expects symbol named solve_cube
pub extern "C" fn solve_cube(
    cube_raw:  *const u8,
    out_moves: *mut solver_move_t,
    max_moves: usize
) -> usize {
    if cube_raw.is_null() || out_moves.is_null() {
        return 0;
    }

    let cube: CubieState = unsafe { // Unsafe because dereferencing raw pointer (cube_raw)
        let slice = slice::from_raw_parts(cube_raw, 54); // Does not copy
        let mut stickers = [0u8; 54];
        stickers.copy_from_slice(slice); // Copies cube data (slice) into Rust owned stack memory
        convert_to_cubie(stickers)
    };

    let out_slice: &mut [solver_move_t] = unsafe {
        // Builds a mutable slice from raw pointer and length
        slice::from_raw_parts_mut(out_moves, max_moves)
    };

    solve_internal(cube, out_slice)
}

#[cfg(not(feature = "std-env"))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}