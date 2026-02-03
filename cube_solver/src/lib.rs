#![cfg_attr(not(feature = "std-env"), no_std)]

use core::slice;

#[cfg(not(feature = "std-env"))]
use core::panic::PanicInfo;

pub const U_FACE: usize = 0;
pub const D_FACE: usize = 1;
pub const L_FACE: usize = 2;
pub const R_FACE: usize = 3;
pub const F_FACE: usize = 4;
pub const B_FACE: usize = 5;

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


impl Cube {
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
        self.rotate_face_cw(L_FACE);

        
    }

    pub fn r(&mut self) {
        // Affected stickers:
        // U[2, 5, 8]    D[2, 5, 8]
        // L is unaffected, R has face stickers rotated
        // F[2, 5, 8]    B[0, 3, 6]
        // U's values go into B (saved B in temp)
        // F's values go into U
        // D's values go into F
        // F's values go into U
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
        self.rotate_face_cw(F_FACE);

        
    }

    pub fn b(&mut self) {
        self.rotate_face_cw(B_FACE);

        
    }

    
    pub fn rotate_face_cw(&mut self, face: usize) {
        // Face layout              Rotated face cw
        //  0  1  2                  6  3  0
        //  3  4  5                  7  4  1
        //  6  7  8                  8  5  2
        // Corners: 0, 2, 6, 8      Corners: 0, 2, 6, 8
        // Edges:   1, 3, 5, 7      Edges:   1, 3, 5, 7
        
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

    fn rotate_face_180(&mut self, face: usize) {
        self.rotate_face_cw(face);
        self.rotate_face_cw(face);
    }

    fn rotate_face_ccw(&mut self, face: usize) {
        self.rotate_face_cw(face);
        self.rotate_face_cw(face);
        self.rotate_face_cw(face);
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
            solver_move_t::Ui => {self.u(); self.u(); self.u();}
            solver_move_t::U2 => {self.u(); self.u();}

            solver_move_t::D  => {self.d();}
            solver_move_t::Di => {self.d(); self.d(); self.d();}
            solver_move_t::D2 => {self.d(); self.d();}

            solver_move_t::L  => {self.l();}
            solver_move_t::Li => {self.l(); self.l(); self.l();}
            solver_move_t::L2 => {self.l(); self.l();}

            solver_move_t::R  => {self.r();}
            solver_move_t::Ri => {self.r(); self.r(); self.r();}
            solver_move_t::R2 => {self.r(); self.r();}

            solver_move_t::F  => {self.f();}
            solver_move_t::Fi => {self.f(); self.f(); self.f();}
            solver_move_t::F2 => {self.f(); self.f();}

            solver_move_t::B  => {self.b();}
            solver_move_t::Bi => {self.b(); self.b(); self.b();}
            solver_move_t::B2 => {self.b(); self.b();}
            _ => {}
        }
    }
}



fn solve_internal(_cube: &Cube, out: &mut [solver_move_t]) -> usize{
    if out.len() < 4{
        return 0;
    }

    out[0] = solver_move_t::Ui;
    out[1] = solver_move_t::Bi;
    out[2] = solver_move_t::D2;
    out[3] = solver_move_t::F2;

    4
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
    let cube: Cube = unsafe { // Unsafe because dereferencing raw pointer (cube_raw)
        let slice = slice::from_raw_parts(cube_raw, 54); // Does not copy
        let mut stickers = [0u8; 54];
        stickers.copy_from_slice(slice); // Copies cube data (slice) into Rust owned stack memory
        Cube { stickers }
    };
    let out_slice: &mut [solver_move_t] = unsafe {
        // Builds a mutable slice from raw pointer and length
        slice::from_raw_parts_mut(out_moves, max_moves)
    };


    solve_internal(&cube, out_slice)
}

#[cfg(not(feature = "std-env"))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}