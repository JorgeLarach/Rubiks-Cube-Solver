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

use crate::tables::{
    CORNER_ORIENT_TABLE, CORNER_PERMUTATION_TABLE, EDGE_ORIENT_TABLE, 
    UD_SLICE_TABLE, UDSLICE_PERMUTATION_TABLE, EDGE_PERMUTATION_TABLE
};

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

    (U_FACE * 9 + 1, B_FACE * 9 + 1), // 0.  UB
    (U_FACE * 9 + 3, L_FACE * 9 + 1), // 1.  UL 
    (U_FACE * 9 + 5, R_FACE * 9 + 1), // 2.  UR 
    (U_FACE * 9 + 7, F_FACE * 9 + 1), // 3.  UF 
    (D_FACE * 9 + 1, F_FACE * 9 + 7), // 4.  DF 
    (D_FACE * 9 + 3, L_FACE * 9 + 7), // 5.  DL 
    (D_FACE * 9 + 5, R_FACE * 9 + 7), // 6.  DR 
    (D_FACE * 9 + 7, B_FACE * 9 + 7), // 7.  DB 
    (L_FACE * 9 + 3, B_FACE * 9 + 5), // 8.  LB 
    (L_FACE * 9 + 5, F_FACE * 9 + 3), // 9.  LF 
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

pub const SOLVED_CUBE_STICKERS:[u8; 54] = [
    0,0,0,0,0,0,0,0,0,
    1,1,1,1,1,1,1,1,1,
    2,2,2,2,2,2,2,2,2,
    3,3,3,3,3,3,3,3,3,
    4,4,4,4,4,4,4,4,4,
    5,5,5,5,5,5,5,5,5
];

pub const ALL_MOVES: [solver_move_t; 18] = [
    solver_move_t::U,  solver_move_t::Ui, solver_move_t::U2,
    solver_move_t::D,  solver_move_t::Di, solver_move_t::D2,
    solver_move_t::L,  solver_move_t::Li, solver_move_t::L2,
    solver_move_t::R,  solver_move_t::Ri, solver_move_t::R2,
    solver_move_t::F,  solver_move_t::Fi, solver_move_t::F2,
    solver_move_t::B,  solver_move_t::Bi, solver_move_t::B2,
];

pub const FACTORIAL: [usize; 8] = [1, 1, 2, 6, 24, 120, 720, 5040];
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
    // Combinatorial Number System: https://en.wikipedia.org/wiki/Combinatorial_number_system
    // Interestingly also related to Lehmer
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

    // UDSLICE PERMUTATION COORDINATE
    // Range: 0..24 (= 4!)
    //
    // PRECONDITION: only meaningful when udslice_coord() == 0,
    // meaning all 4 UD-slice edges are already in slots 8-11.
    // Phase 1 guarantees this before Phase 2 runs.
    //
    // The 4 UD-slice edges are cubies 8 (LB), 9 (LF), 10 (RF), 11 (RB).
    // This coordinate encodes which of the 4! = 24 orderings they sit in
    // across slots 8, 9, 10, 11.
    //
    // We use the same Lehmer encoding as corner_perm_coord, just over
    // 4 elements instead of 8. For each slot s in {8,9,10,11}, count
    // how many slots to its right (within the belt) hold a cubie with
    // a smaller cubie ID. Multiply by (3 - local_index)! and sum.
    pub fn udslice_perm_coord(&self) -> usize {
        const FACTORIAL: [usize; 4] = [1, 1, 2, 6];

        // Extract just the 4 cubie IDs sitting in belt slots 8..11,
        // normalized to the range 0..3 for clean Lehmer encoding.
        // Cubie IDs are 8,9,10,11 so subtract 8 to get 0,1,2,3.
        let mut belt = [0u8; 4];
        for i in 0..4 {
            // find which cubie is in slot (8 + i)
            let cubie = self.find_edge_at((8 + i) as u8);
            // normalize: cubie 8->0, 9->1, 10->2, 11->3
            belt[i] = (cubie - 8) as u8;
        }

        // Lehmer code over the 4-element permutation in belt[]
        let mut coord = 0usize;
        for i in 0..4 {
            let smaller = ((i + 1)..4)
                .filter(|&j| belt[j] < belt[i])
                .count();
            coord += smaller * FACTORIAL[3 - i];
        }
        coord
    }

    // EDGE PERMUTATION COORDINATE

    pub fn edge_perm_coord(&self) -> usize {
        // PRECONDITION: only meaningful in Phase 2, when UD-slice edges
        // are guaranteed to occupy slots 8-11. That means edges 0-7
        // (UB, UL, UR, UF, DF, DL, DR, DB) are guaranteed to occupy
        // slots 0-7, so their positions form a clean permutation of 0..7
        // and the Lehmer code below is well-defined.
        // Calling this during Phase 1 (before UD-slice edges are in their
        // belt) will produce meaningless values.
        let mut coord = 0usize;
        for i in 0..8 {
            let smaller = ((i + 1)..8)
                .filter(|&j| self.edges[j].position < self.edges[i].position)
                .count();

            coord += smaller * FACTORIAL[7 - i];
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

/* IDA Star Helper Functions */
fn face_of(m: solver_move_t) -> u8 {
    match m {
        solver_move_t::U | solver_move_t::Ui | solver_move_t::U2 => U_FACE as u8,
        solver_move_t::D | solver_move_t::Di | solver_move_t::D2 => D_FACE as u8,
        solver_move_t::L | solver_move_t::Li | solver_move_t::L2 => L_FACE as u8,
        solver_move_t::R | solver_move_t::Ri | solver_move_t::R2 => R_FACE as u8,
        solver_move_t::F | solver_move_t::Fi | solver_move_t::F2 => F_FACE as u8,
        solver_move_t::B | solver_move_t::Bi | solver_move_t::B2 => B_FACE as u8

    }
}

// Technique to identify opposite faces:
// Divide each face number by 2. If they result in the same number, they are opposites:
// U_FACE (0/2) = 0, D_FACE (1/2) = 0
// L_FACE (2/2) = 1, R_FACE (3/2) = 1
// F_FACE (4/2) = 2, B_FACE (5/2) = 2

// We want to enforce 2 pruning rules:
// 1. Prune paths that apply two sequential moves on the same face. Should never be necessary
// 2. Prune paths that apply moves to opposite faces but in the incorrect order. Always allow the higher-indexed face to come after the lower.
fn should_prune(last_face: u8, m: solver_move_t) -> bool {
    if last_face == 255 {return false;} // No previous move
    let current_face = face_of(m);

    // Application of Rule 1
    if last_face == current_face {return true;}   

    // Application of Rule 2                               
    if last_face/2 == current_face/2 && current_face < last_face { return true;} 
    false
}


/* Solve Algorithm */


// ============================================================
// PHASE 1 HEURISTIC
// ============================================================
// Lower bound on moves needed to reach G1.
// G1 is reached when CO=0, EO=0, UD-slice=0 simultaneously.
// Each table gives the minimum moves to fix that one aspect alone.
// The max is a valid lower bound for fixing all three together.
pub fn heuristic_phase1(cube: &CubieState) -> u8 {
    let co = CORNER_ORIENT_TABLE[cube.corner_orient_coord()];
    let eo = EDGE_ORIENT_TABLE[cube.edge_orient_coord()];
    let ud = UD_SLICE_TABLE[cube.udslice_coord()];
    co.max(eo).max(ud)
}

// ============================================================
// PHASE 2 HEURISTIC
// ============================================================
// Lower bound on moves needed to fully solve from G1.
// Phase 2 is done when CP=0, UP=0, EP=0 simultaneously.
pub fn heuristic_phase2(cube: &CubieState) -> u8 {
    // udslice_perm_coord is only valid when UD-slice edges are
    // in their belt slots, which is only guaranteed after Phase 1.
    // If called on a non-G1 cube, return a safe fallback.
    // In practice this should never happen — Phase 2 only runs
    // on cubes that have passed through Phase 1.
    if cube.udslice_coord() != 0 {
        return u8::MAX; // signals "not in G1, don't call this"
    }
    let cp = CORNER_PERMUTATION_TABLE[cube.corner_perm_coord()];
    let up = UDSLICE_PERMUTATION_TABLE[cube.udslice_perm_coord()];
    let ep = EDGE_PERMUTATION_TABLE[cube.edge_perm_coord()];
    cp.max(up).max(ep)
}

// ============================================================
// PHASE 1 GOAL CHECK
// ============================================================
// Returns true when the cube is in G1:
// all corners untwisted, all edges unflipped, all belt edges in belt.
pub fn is_phase1_solved(cube: &CubieState) -> bool {
    cube.corner_orient_coord() == 0
        && cube.edge_orient_coord() == 0
        && cube.udslice_coord() == 0
}

// ============================================================
// PHASE 2 GOAL CHECK
// ============================================================
// Returns true when the cube is fully solved:
// corners in home slots, belt edges in correct belt order,
// non-belt edges in home slots.
// NOTE: we use coord checks rather than cube.is_solved() because
// is_solved() also checks orientations, which are guaranteed to be
// correct by Phase 1 and never disturbed by Phase 2 moves.
// Both approaches are correct — coord checks are slightly faster.
pub fn is_phase2_solved(cube: &CubieState) -> bool {
    cube.corner_perm_coord() == 0
        && cube.udslice_perm_coord() == 0
        && cube.edge_perm_coord() == 0
}

// ============================================================
// PHASE 2 MOVE SET
// ============================================================
// Only these 10 moves are allowed in Phase 2.
// Quarter turns of R, L, F, B are excluded because they:
//   - flip edges (destroying EO=0 guarantee from Phase 1)
//   - displace belt edges into non-belt slots (destroying UD-slice=0)
// Every move in this set keeps the cube within G1.
const PHASE2_MOVES: [solver_move_t; 10] = [
    solver_move_t::U,  solver_move_t::Ui, solver_move_t::U2,
    solver_move_t::D,  solver_move_t::Di, solver_move_t::D2,
    solver_move_t::R2, solver_move_t::L2,
    solver_move_t::F2, solver_move_t::B2,
];

// ============================================================
// IDA* RECURSIVE SEARCH
// ============================================================
// Resources: https://en.wikipedia.org/wiki/Iterative_deepening_A*
//
// IDA* (Iterative Deepening A*) is a memory-efficient optimal search.
// It runs repeated depth-first searches, each with a slightly higher
// cost threshold. Because the threshold starts at the heuristic value
// and grows by the minimum amount each iteration, it's guaranteed to
// find the shortest solution.
//
// At each node:
//   g = cost so far (number of moves made)
//   h = heuristic estimate of remaining cost (four-table lookup)
//   f = g + h = estimated total solution length through this node
//
// If f > threshold: prune this branch entirely (return the f value
//   so the outer loop knows the minimum useful next threshold).
// If cube is solved: we're done — record solution length and return None.
// Otherwise: try all 18 moves and recurse.
//
// RETURN VALUE:
//   None        -> solution found. sol_len has been written.
//   Some(min_f) -> not found. min_f is the smallest f that exceeded
//                  the threshold — the outer loop uses this as the
//                  next threshold value.
//
// PARAMETERS:
//   cube      - current cube state at this node
//   g         - depth / number of moves made so far
//   threshold - current IDA* cost threshold
//   last_face - face index of the last applied move (255 = none)
//   path      - the move sequence being built; path[0..g] is current path
//   sol_len   - written with g when solution is found

pub fn ida_phase1_recursive(
    cube:      &CubieState,
    g:         u8,
    threshold: u8,
    last_face: u8,
    path:      &mut [solver_move_t],
    sol_len:   &mut usize,
) -> Option<u8> {

    let h = heuristic_phase1(cube);
    let f = g.saturating_add(h);

    // Prune: this branch cannot possibly improve on the current threshold
    if f > threshold {
        return Some(f);
    }

    // Goal: cube is in G1
    if is_phase1_solved(cube) {
        *sol_len = g as usize;
        return None; // None = success
    }

    // Safety: buffer full
    if g as usize >= path.len() {
        return Some(u8::MAX);
    }

    let mut min_exceeded: u8 = u8::MAX;

    for &m in ALL_MOVES.iter() {
        if should_prune(last_face, m) {
            continue;
        }

        let mut next = *cube;
        next.apply_move(m);
        path[g as usize] = m;

        let result = ida_phase1_recursive(
            &next,
            g + 1,
            threshold,
            face_of(m),
            path,
            sol_len,
        );

        match result {
            // Success — propagate immediately, don't try other moves
            None => return None,
            Some(t) => {
                if t < min_exceeded {
                    min_exceeded = t;
                }
            }
        }
    }

    Some(min_exceeded)
}

// ============================================================
// PHASE 1 OUTER LOOP
// ============================================================
// Manages the IDA* threshold and drives iterative deepening.
// Returns the number of Phase 1 moves written to path.
pub fn kociemba_phase1(
    cube: &CubieState,
    path: &mut [solver_move_t],
) -> usize {

    // Already in G1 — nothing to do
    if is_phase1_solved(cube) {
        return 0;
    }

    // Initial threshold: tightest possible lower bound on Phase 1 depth
    let mut threshold = heuristic_phase1(cube);
    let mut sol_len = 0usize;

    loop {
        let result = ida_phase1_recursive(
            cube,
            0,
            threshold,
            255,        // 255 = no last face, no pruning at root
            path,
            &mut sol_len,
        );

        match result {
            // Solution found — sol_len moves written to path
            None => return sol_len,

            // Buffer full or other abort — should not happen
            // if path is sized correctly (30 moves is enough for Phase 1)
            Some(u8::MAX) => return 0,

            // Not found — raise threshold to minimum exceeded value
            // and try again. This is tighter than blindly adding 1.
            Some(new_threshold) => threshold = new_threshold,
        }
    }
}

// ============================================================
// PHASE 2 RECURSIVE IDA*
// ============================================================
// Same structure as Phase 1 but with two key differences:
//   1. Goal check is is_phase2_solved()
//   2. Only PHASE2_MOVES are tried — all 18 would break G1
pub fn ida_phase2_recursive(
    cube:      &CubieState,
    g:         u8,
    threshold: u8,
    last_face: u8,
    path:      &mut [solver_move_t],
    sol_len:   &mut usize,
) -> Option<u8> {

    let h = heuristic_phase2(cube);
    let f = g.saturating_add(h);

    if f > threshold {
        return Some(f);
    }

    // Goal: cube is fully solved
    if is_phase2_solved(cube) {
        *sol_len = g as usize;
        return None;
    }

    if g as usize >= path.len() {
        return Some(u8::MAX);
    }

    let mut min_exceeded: u8 = u8::MAX;

    // CRITICAL: only iterate PHASE2_MOVES, not ALL_MOVES
    for &m in PHASE2_MOVES.iter() {
        if should_prune(last_face, m) {
            continue;
        }

        let mut next = *cube;
        next.apply_move(m);
        path[g as usize] = m;

        let result = ida_phase2_recursive(
            &next,
            g + 1,
            threshold,
            face_of(m),
            path,
            sol_len,
        );

        match result {
            None => return None,
            Some(t) => {
                if t < min_exceeded {
                    min_exceeded = t;
                }
            }
        }
    }

    Some(min_exceeded)
}

// ============================================================
// PHASE 2 OUTER LOOP
// ============================================================
// path here is a slice starting AFTER the Phase 1 moves.
// The caller is responsible for passing the correct offset.
pub fn kociemba_phase2(
    cube: &CubieState,
    path: &mut [solver_move_t],
) -> usize {

    // Already fully solved — nothing to do
    if is_phase2_solved(cube) {
        return 0;
    }

    let mut threshold = heuristic_phase2(cube);
    let mut sol_len = 0usize;

    loop {
        let result = ida_phase2_recursive(
            cube,
            0,
            threshold,
            255,
            path,
            &mut sol_len,
        );

        match result {
            None => return sol_len,
            Some(u8::MAX) => return 0,
            Some(new_threshold) => threshold = new_threshold,
        }
    }
}

// ============================================================
// TOP LEVEL ENTRY POINT
// ============================================================
// Called by the existing solve_cube FFI function 
pub fn solve_internal(cube: CubieState, out: &mut [solver_move_t]) -> usize {

    // Already solved — nothing to do
    if is_phase2_solved(&cube) {
        return 0;
    }

    // Phase 1: find moves to reach G1.
    // Write Phase 1 moves into the front of out.
    // Reserve 30 slots — worst case Phase 1 is ~12 moves with headroom.
    let p1_len = kociemba_phase1(&cube, &mut out[..30]);

    // If Phase 1 returned 0 but cube isn't in G1, something is wrong
    if p1_len == 0 && !is_phase1_solved(&cube) {
        return 0; // abort
    }

    // Apply Phase 1 moves to get the G1 cube
    let mut g1_cube = cube;
    for i in 0..p1_len {
        g1_cube.apply_move(out[i]);
    }

    // Sanity check: g1_cube must now be in G1
    // If this fails, there is a bug in Phase 1
    if !is_phase1_solved(&g1_cube) {
        return 0; // abort
    }

    // Phase 2: solve from G1 to fully solved.
    // Write Phase 2 moves into out AFTER the Phase 1 moves.
    // Reserve 40 slots — worst case Phase 2 is ~18 moves with headroom.
    let p2_len = kociemba_phase2(&g1_cube, &mut out[p1_len..p1_len + 40]);

    // Total solution length is Phase 1 + Phase 2
    p1_len + p2_len
}

fn test_moves_short(out: &mut [solver_move_t]) -> usize{
    out[0]  = solver_move_t::U;
    out[1]  = solver_move_t::D;
    out[2]  = solver_move_t::L;
    out[3]  = solver_move_t::R;
    out[4]  = solver_move_t::F;
    out[5]  = solver_move_t::B;

    out[6] = solver_move_t::Bi;
    out[7] = solver_move_t::Fi;
    out[8] = solver_move_t::Ri;
    out[9] = solver_move_t::Li;
    out[10] = solver_move_t::Di;
    out[11] = solver_move_t::Ui;

    12

    // out[0]  = solver_move_t::U;
    // out[1]  = solver_move_t::L;
    // out[2]  = solver_move_t::R;
    // out[3]  = solver_move_t::F;
    // out[4]  = solver_move_t::U2;
    // out[5]  = solver_move_t::B;

    // out[6] = solver_move_t::Bi;
    // out[7]  = solver_move_t::U2;
    // out[8] = solver_move_t::Fi;
    // out[9] = solver_move_t::Ri;
    // out[10] = solver_move_t::Li;
    // out[11] = solver_move_t::Ui;
    // 12

    // out[0]  = solver_move_t::D;
    // out[1]  = solver_move_t::D2;
    // out[2]  = solver_move_t::Di;

    // 3

}

fn test_moves_long(out: &mut [solver_move_t]) -> usize {
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
    // test_moves_long(out_slice)
    // test_moves_short(out_slice)
}

#[cfg(not(feature = "std-env"))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}