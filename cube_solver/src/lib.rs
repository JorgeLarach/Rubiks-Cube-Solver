
// This program hasn't seen the last of me yet

#![cfg_attr(not(feature = "std-env"), no_std)]

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

#[derive(Copy, Clone, Debug)]
pub struct Cube {
    pub stickers: [u8; 54],
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

pub const U_FACE_MIN_IDX: usize = 0;
pub const U_FACE_MAX_IDX: usize = 8;
pub const D_FACE_MIN_IDX: usize = 9;
pub const D_FACE_MAX_IDX: usize = 17;
pub const L_FACE_MIN_IDX: usize = 18;
pub const L_FACE_MAX_IDX: usize = 26;
pub const R_FACE_MIN_IDX: usize = 27;
pub const R_FACE_MAX_IDX: usize = 35;
pub const F_FACE_MIN_IDX: usize = 36;
pub const F_FACE_MAX_IDX: usize = 44;
pub const B_FACE_MIN_IDX: usize = 45;
pub const B_FACE_MAX_IDX: usize = 53;

pub const EDGES: [(usize, usize); 12] = [
    // There are 24 edge stickers in the cube (four on each face). 
    // Each edge sticker is adjacent to an edge sticker on another face. 
    // That pair makes an edge piece. These are used in the algorithm
    // Each face's edge stickers are located at [1, 3, 5, 7]
    (U_FACE * 9 + 1, B_FACE * 9 + 1), // Edge Piece 0 (White-Orange)
    (U_FACE * 9 + 3, L_FACE * 9 + 1), // Edge Piece 1 (White-Green)
    (U_FACE * 9 + 5, R_FACE * 9 + 1), // Edge Piece 2 (White-Blue)
    (U_FACE * 9 + 7, F_FACE * 9 + 1), // Edge Piece 3 (White-Red)

    (D_FACE * 9 + 1, F_FACE * 9 + 7), // Edge Piece 4 (Yellow-Red)
    (D_FACE * 9 + 3, L_FACE * 9 + 7), // Edge Piece 5 (Yellow-Green)
    (D_FACE * 9 + 5, R_FACE * 9 + 7), // Edge Piece 6 (Yellow-Blue)
    (D_FACE * 9 + 7, B_FACE * 9 + 7), // Edge Piece 7 (Yellow-Orange)

    (L_FACE * 9 + 3, B_FACE * 9 + 5), // Edge Piece 8 (Green-Orange)
    (L_FACE * 9 + 5, F_FACE * 9 + 3), // Edge Piece 9 (Green-Red)

    (R_FACE * 9 + 3, F_FACE * 9 + 5), // Edge Piece 10 (Right-Front)
    (R_FACE * 9 + 5, B_FACE * 9 + 3), // Edge Piece 11 (Right-Back)
];

// Helper function: get face from sticker index
pub fn identify_face_from_sticker_idx(sticker_idx: usize) -> usize {
    match sticker_idx {
        i if i < 9   => U_FACE,
        i if i < 18  => D_FACE,
        i if i < 27  => L_FACE,
        i if i < 36  => R_FACE,
        i if i < 45  => F_FACE,
        i if i < 54  => B_FACE,
        _ => panic!("sticker index out of range"),
    }
}

pub fn find_inverse_move(input_move: solver_move_t) -> solver_move_t{
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


impl Cube {
    /* SOLVER STUFF */

    pub fn solve_white_cross(&mut self, _out: &mut [solver_move_t]) -> usize {
        // About 1000 lines later, I decided to leave this function empty. Maybe I should learn to solve a cube first.
        0
    }

    
    pub fn find_edge(&self, color1: u8, color2: u8) -> Option<(usize, usize)> {
        // Search through all edge pieces (in EDGES array) until match found
        
        for &(idx1, idx2) in EDGES.iter() {
            let sticker_color1 = self.stickers[idx1];
            let sticker_color2 = self.stickers[idx2];
            
            // Always return index of color1 as first val in return tuple 
            // e.g. find_edge(WHITE, ORANGE) returns tuple where first val is idx of WHITE sticker for that edge piece
            if sticker_color1 == color1 && sticker_color2 == color2 {
                return Some((idx1, idx2));
            } else if sticker_color1 == color2 && sticker_color2 == color1 {
                return Some((idx2, idx1));
            }
        }
        None
    }

    #[inline]
    pub fn apply_and_record(&mut self, out: &mut [solver_move_t], out_idx: &mut usize, m: solver_move_t) {
        self.apply_move(m);
        self.record_move(out, *out_idx, m);
        *out_idx += 1;
    }

    pub fn record_move(&self, out: &mut [solver_move_t], out_idx: usize, m: solver_move_t) {
        if out_idx < out.len() {
            out[out_idx] = m
        }
    }
    
    /* ROTATIONS */

    pub fn u(&mut self) {
        // Affected stickers:
        // U has face stickers rotated, D is unaffected
        // L[0, 1, 2]    R[0, 1, 2]
        // F[0, 1, 2]    B[0, 1, 2]
        // B's values go into R (saved in temp bc right of U)
        // L's values go into B
        // F's values go into L
        // R's values go into F

        self.rotate_face_cw(U_FACE);

        let temp_r_0: u8 = self.stickers[R_FACE * 9 + 0];
        let temp_r_1: u8 = self.stickers[R_FACE * 9 + 1];
        let temp_r_2: u8 = self.stickers[R_FACE * 9 + 2];

        self.stickers[R_FACE * 9 + 0] = self.stickers[B_FACE * 9 + 0];
        self.stickers[R_FACE * 9 + 1] = self.stickers[B_FACE * 9 + 1];
        self.stickers[R_FACE * 9 + 2] = self.stickers[B_FACE * 9 + 2];

        self.stickers[B_FACE * 9 + 0] = self.stickers[L_FACE * 9 + 0];
        self.stickers[B_FACE * 9 + 1] = self.stickers[L_FACE * 9 + 1];
        self.stickers[B_FACE * 9 + 2] = self.stickers[L_FACE * 9 + 2];

        self.stickers[L_FACE * 9 + 0] = self.stickers[F_FACE * 9 + 0];
        self.stickers[L_FACE * 9 + 1] = self.stickers[F_FACE * 9 + 1];
        self.stickers[L_FACE * 9 + 2] = self.stickers[F_FACE * 9 + 2];

        self.stickers[F_FACE * 9 + 0] = temp_r_0;
        self.stickers[F_FACE * 9 + 1] = temp_r_1;
        self.stickers[F_FACE * 9 + 2] = temp_r_2;
    }
    
    pub fn d(&mut self) {
        // Affected stickers:
        // U is unaffected, D has face stickers rotated
        // L[6, 7, 8]    R[6, 7, 8]
        // F[6, 7, 8]    B[6, 7, 8]
        // F's values go into R (saved in temp bc right of D)
        // L's values go into F
        // B's values go into L
        // R's values go into B

        self.rotate_face_cw(D_FACE);

        let temp_r_6: u8 = self.stickers[R_FACE * 9 + 6];
        let temp_r_7: u8 = self.stickers[R_FACE * 9 + 7];
        let temp_r_8: u8 = self.stickers[R_FACE * 9 + 8];
        
        self.stickers[R_FACE * 9 + 6] = self.stickers[F_FACE * 9 + 6];
        self.stickers[R_FACE * 9 + 7] = self.stickers[F_FACE * 9 + 7];
        self.stickers[R_FACE * 9 + 8] = self.stickers[F_FACE * 9 + 8];

        self.stickers[F_FACE * 9 + 6] = self.stickers[L_FACE * 9 + 6];
        self.stickers[F_FACE * 9 + 7] = self.stickers[L_FACE * 9 + 7];
        self.stickers[F_FACE * 9 + 8] = self.stickers[L_FACE * 9 + 8];

        self.stickers[L_FACE * 9 + 6] = self.stickers[B_FACE * 9 + 6];
        self.stickers[L_FACE * 9 + 7] = self.stickers[B_FACE * 9 + 7];
        self.stickers[L_FACE * 9 + 8] = self.stickers[B_FACE * 9 + 8];
        
        self.stickers[B_FACE * 9 + 6] = temp_r_6;
        self.stickers[B_FACE * 9 + 7] = temp_r_7;
        self.stickers[B_FACE * 9 + 8] = temp_r_8;
    }

    pub fn l(&mut self) {
        // Affected stickers:
        // U[0, 3, 6]    D[0, 3, 6]
        // L has face stickers rotated, R is unaffected
        // F[0, 3, 6]    B[2, 5, 8]
        // U's values go into F (saved in temp bc right of L)
        // B's values go into U
        // D's values go into B
        // F's values go into D
        
        self.rotate_face_cw(L_FACE);

        let temp_f_0: u8 = self.stickers[F_FACE * 9 + 0];
        let temp_f_3: u8 = self.stickers[F_FACE * 9 + 3];
        let temp_f_6: u8 = self.stickers[F_FACE * 9 + 6];

        self.stickers[F_FACE * 9 + 0] = self.stickers[U_FACE * 9 + 0];
        self.stickers[F_FACE * 9 + 3] = self.stickers[U_FACE * 9 + 3];
        self.stickers[F_FACE * 9 + 6] = self.stickers[U_FACE * 9 + 6];

        self.stickers[U_FACE * 9 + 0] = self.stickers[B_FACE * 9 + 8];
        self.stickers[U_FACE * 9 + 3] = self.stickers[B_FACE * 9 + 5];
        self.stickers[U_FACE * 9 + 6] = self.stickers[B_FACE * 9 + 2];

        self.stickers[B_FACE * 9 + 2] = self.stickers[D_FACE * 9 + 6];
        self.stickers[B_FACE * 9 + 5] = self.stickers[D_FACE * 9 + 3];
        self.stickers[B_FACE * 9 + 8] = self.stickers[D_FACE * 9 + 0];

        self.stickers[D_FACE * 9 + 0] = temp_f_0;
        self.stickers[D_FACE * 9 + 3] = temp_f_3;
        self.stickers[D_FACE * 9 + 6] = temp_f_6;
    }

    pub fn r(&mut self) {
        // Affected stickers:
        // U[2, 5, 8]    D[2, 5, 8]
        // L is unaffected, R has face stickers rotated
        // F[2, 5, 8]    B[0, 3, 6]
        // U's values go into B (saved in temp bc right of R)
        // F's values go into U
        // D's values go into F
        // B's values go into D
        self.rotate_face_cw(R_FACE);

        let temp_b_6: u8 = self.stickers[B_FACE * 9 + 6];
        let temp_b_0: u8 = self.stickers[B_FACE * 9 + 0];
        let temp_b_3: u8 = self.stickers[B_FACE * 9 + 3];

        self.stickers[B_FACE * 9 + 0] = self.stickers[U_FACE * 9 + 8];
        self.stickers[B_FACE * 9 + 3] = self.stickers[U_FACE * 9 + 5];
        self.stickers[B_FACE * 9 + 6] = self.stickers[U_FACE * 9 + 2];

        self.stickers[U_FACE * 9 + 2] = self.stickers[F_FACE * 9 + 2];
        self.stickers[U_FACE * 9 + 5] = self.stickers[F_FACE * 9 + 5];
        self.stickers[U_FACE * 9 + 8] = self.stickers[F_FACE * 9 + 8];

        self.stickers[F_FACE * 9 + 2] = self.stickers[D_FACE * 9 + 2];
        self.stickers[F_FACE * 9 + 5] = self.stickers[D_FACE * 9 + 5];
        self.stickers[F_FACE * 9 + 8] = self.stickers[D_FACE * 9 + 8];

        self.stickers[D_FACE * 9 + 2] = temp_b_6;
        self.stickers[D_FACE * 9 + 5] = temp_b_3;
        self.stickers[D_FACE * 9 + 8] = temp_b_0;
    }

    pub fn f(&mut self) {
        // Affected stickers:
        // U[6, 7, 8]    D[6, 7, 8]
        // L[2, 5, 8]    R[0, 3, 6]
        // F has face stickers rotated, B is unaffected
        // U's values go into R (saved in temp bc right of F)
        // L's values go into U
        // D's values go into L
        // R's values go into D
        self.rotate_face_cw(F_FACE);

        let temp_r_0: u8 = self.stickers[R_FACE * 9 + 0];
        let temp_r_3: u8 = self.stickers[R_FACE * 9 + 3];
        let temp_r_6: u8 = self.stickers[R_FACE * 9 + 6];

        self.stickers[R_FACE * 9 + 0] = self.stickers[U_FACE * 9 + 6];
        self.stickers[R_FACE * 9 + 3] = self.stickers[U_FACE * 9 + 7];
        self.stickers[R_FACE * 9 + 6] = self.stickers[U_FACE * 9 + 8];

        self.stickers[U_FACE * 9 + 6] = self.stickers[L_FACE * 9 + 8];
        self.stickers[U_FACE * 9 + 7] = self.stickers[L_FACE * 9 + 5];
        self.stickers[U_FACE * 9 + 8] = self.stickers[L_FACE * 9 + 2];

        self.stickers[L_FACE * 9 + 2] = self.stickers[D_FACE * 9 + 0];
        self.stickers[L_FACE * 9 + 5] = self.stickers[D_FACE * 9 + 1];
        self.stickers[L_FACE * 9 + 8] = self.stickers[D_FACE * 9 + 2];

        self.stickers[D_FACE * 9 + 0] = temp_r_6;
        self.stickers[D_FACE * 9 + 1] = temp_r_3;
        self.stickers[D_FACE * 9 + 2] = temp_r_0;
    }

    pub fn b(&mut self) {
        // Affected stickers:
        // U[0, 1, 2]    D[6, 7, 8]
        // L[0, 3, 6]    R[2, 5, 8]
        // F is unaffected, B has face stickers rotated
        // U's values go into L (saved in temp bc right of B)
        // R's values go into U
        // D's values go into R
        // L's values go into D
        self.rotate_face_cw(B_FACE);

        let temp_l_0: u8 = self.stickers[L_FACE * 9 + 0];
        let temp_l_3: u8 = self.stickers[L_FACE * 9 + 3];
        let temp_l_6: u8 = self.stickers[L_FACE * 9 + 6];

        self.stickers[L_FACE * 9 + 0] = self.stickers[U_FACE * 9 + 2];
        self.stickers[L_FACE * 9 + 3] = self.stickers[U_FACE * 9 + 1];
        self.stickers[L_FACE * 9 + 6] = self.stickers[U_FACE * 9 + 0];

        self.stickers[U_FACE * 9 + 0] = self.stickers[R_FACE * 9 + 2];
        self.stickers[U_FACE * 9 + 1] = self.stickers[R_FACE * 9 + 5];
        self.stickers[U_FACE * 9 + 2] = self.stickers[R_FACE * 9 + 8];

        self.stickers[R_FACE * 9 + 2] = self.stickers[D_FACE * 9 + 8];
        self.stickers[R_FACE * 9 + 5] = self.stickers[D_FACE * 9 + 7];
        self.stickers[R_FACE * 9 + 8] = self.stickers[D_FACE * 9 + 6];

        self.stickers[D_FACE * 9 + 6] = temp_l_0;
        self.stickers[D_FACE * 9 + 7] = temp_l_3;
        self.stickers[D_FACE * 9 + 8] = temp_l_6;
    }

    pub fn ui(&mut self) {self.u(); self.u(); self.u()}
    pub fn di(&mut self) {self.d(); self.d(); self.d()}
    pub fn li(&mut self) {self.l(); self.l(); self.l()}
    pub fn ri(&mut self) {self.r(); self.r(); self.r()}
    pub fn fi(&mut self) {self.f(); self.f(); self.f()}
    pub fn bi(&mut self) {self.b(); self.b(); self.b()}

    pub fn rotate_face_cw(&mut self, face: usize) {
        // Face layout              Rotated face cw
        //  0  1  2                  6  3  0
        //  3  4  5                  7  4  1
        //  6  7  8                  8  5  2
        // Corners: 0, 2, 6, 8      
        // Edges:   1, 3, 5, 7      
        
        let base: usize = face * 9; // base index into stickers array for face's 9 stickers
        
        // Save corners
        let temp_0: u8 = self.stickers[base + 0];
        let temp_2: u8 = self.stickers[base + 2];
        let temp_6: u8 = self.stickers[base + 6];
        let temp_8: u8 = self.stickers[base + 8];

        // Rotate corners
        self.stickers[base + 0] = temp_6;
        self.stickers[base + 2] = temp_0;
        self.stickers[base + 6] = temp_8;
        self.stickers[base + 8] = temp_2;

        // Save edges
        let temp_1: u8 = self.stickers[base + 1];
        let temp_3: u8 = self.stickers[base + 3];
        let temp_5: u8 = self.stickers[base + 5];
        let temp_7: u8 = self.stickers[base + 7];

        // Rotate edges
        self.stickers[base + 1] = temp_3;
        self.stickers[base + 3] = temp_7;
        self.stickers[base + 5] = temp_1;
        self.stickers[base + 7] = temp_5;

    }

    pub fn make_solved() -> Self {
        let mut stickers: [u8; 54] = [0u8; 54];
        for face in 0..6 {
            for i in 0..9 {
                stickers[face * 9 + i] = face as u8;
            }
        }

        Cube {stickers}
    }

    pub fn is_solved(&self) -> bool {
        for face in 0..6 {
            let color = self.stickers[face * 9];
            for i in 0..9 {
                if self.stickers[face * 9 + i] != color {
                    return false;
                }
            }
        }
        true
    }

    pub fn apply_move(&mut self, m: solver_move_t) {
        match m {
            solver_move_t::U  => {self.u();}
            solver_move_t::Ui => {self.ui();}
            solver_move_t::U2 => {self.u(); self.u();}

            solver_move_t::D  => {self.d();}
            solver_move_t::Di => {self.di();}
            solver_move_t::D2 => {self.d(); self.d();}

            solver_move_t::L  => {self.l();}
            solver_move_t::Li => {self.li();}
            solver_move_t::L2 => {self.l(); self.l();}

            solver_move_t::R  => {self.r();}
            solver_move_t::Ri => {self.ri();}
            solver_move_t::R2 => {self.r(); self.r();}

            solver_move_t::F  => {self.f();}
            solver_move_t::Fi => {self.fi();}
            solver_move_t::F2 => {self.f(); self.f();}

            solver_move_t::B  => {self.b();}
            solver_move_t::Bi => {self.bi();}
            solver_move_t::B2 => {self.b(); self.b();}
        }
    }
}


fn solve_internal(_cube: &mut Cube, out: &mut [solver_move_t]) -> usize{
    // if out.len() < 4{
    //     return 0;
    // }

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
    // out[1]  = solver_move_t::D;
    // out[2]  = solver_move_t::L;
    // out[3]  = solver_move_t::R;
    // out[4]  = solver_move_t::F;
    // out[5]  = solver_move_t::B;

    // out[6]  = solver_move_t::U2;
    // out[7]  = solver_move_t::D2;
    // out[8]  = solver_move_t::L2;
    // out[9]  = solver_move_t::R2;
    // out[10] = solver_move_t::F2;
    // out[11] = solver_move_t::B2;

    // out[12] = solver_move_t::Ui;
    // out[13] = solver_move_t::Di;
    // out[14] = solver_move_t::Li;
    // out[15] = solver_move_t::Ri;
    // out[16] = solver_move_t::Fi;
    // out[17] = solver_move_t::Bi;

    // // ***** inverse starts here *****

    // out[18] = solver_move_t::B;
    // out[19] = solver_move_t::F;
    // out[20] = solver_move_t::R;
    // out[21] = solver_move_t::L;
    // out[22] = solver_move_t::D;
    // out[23] = solver_move_t::U;

    // out[24] = solver_move_t::B2;
    // out[25] = solver_move_t::F2;
    // out[26] = solver_move_t::R2;
    // out[27] = solver_move_t::L2;
    // out[28] = solver_move_t::D2;
    // out[29] = solver_move_t::U2;

    // out[30] = solver_move_t::Bi;
    // out[31] = solver_move_t::Fi;
    // out[32] = solver_move_t::Ri;
    // out[33] = solver_move_t::Li;
    // out[34] = solver_move_t::Di;
    // out[35] = solver_move_t::Ui;
 
    // 36
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
    let mut cube: Cube = unsafe { // Unsafe because dereferencing raw pointer (cube_raw)
        let slice = slice::from_raw_parts(cube_raw, 54); // Does not copy
        let mut stickers = [0u8; 54];
        stickers.copy_from_slice(slice); // Copies cube data (slice) into Rust owned stack memory
        Cube { stickers }
    };
    let out_slice: &mut [solver_move_t] = unsafe {
        // Builds a mutable slice from raw pointer and length
        slice::from_raw_parts_mut(out_moves, max_moves)
    };

    solve_internal(&mut cube, out_slice)
}

#[cfg(not(feature = "std-env"))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}