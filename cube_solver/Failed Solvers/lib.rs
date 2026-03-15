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

pub const WHITE:  u8 = 0;
pub const YELLOW: u8 = 1;
pub const GREEN:  u8 = 2;
pub const BLUE:   u8 = 3;
pub const RED:    u8 = 4;
pub const ORANGE: u8 = 5;

pub const EDGES: [(usize, usize); 12] = [
    (U_FACE * 9 + 1, B_FACE * 9 + 1),
    (U_FACE * 9 + 3, L_FACE * 9 + 1),
    (U_FACE * 9 + 5, R_FACE * 9 + 1),
    (U_FACE * 9 + 7, F_FACE * 9 + 1),
    (D_FACE * 9 + 1, F_FACE * 9 + 7),
    (D_FACE * 9 + 3, L_FACE * 9 + 7),
    (D_FACE * 9 + 5, R_FACE * 9 + 7),
    (D_FACE * 9 + 7, B_FACE * 9 + 7),
    (L_FACE * 9 + 3, B_FACE * 9 + 5),
    (L_FACE * 9 + 5, F_FACE * 9 + 3),
    (R_FACE * 9 + 3, F_FACE * 9 + 5),
    (R_FACE * 9 + 5, B_FACE * 9 + 3),
];

pub const CORNERS: [(usize, usize, usize); 8] = [
    (U_FACE * 9 + 0, L_FACE * 9 + 0, B_FACE * 9 + 2),
    (U_FACE * 9 + 2, R_FACE * 9 + 0, B_FACE * 9 + 0),
    (U_FACE * 9 + 6, L_FACE * 9 + 2, F_FACE * 9 + 0),
    (U_FACE * 9 + 8, R_FACE * 9 + 2, F_FACE * 9 + 2),
    (D_FACE * 9 + 7, L_FACE * 9 + 8, B_FACE * 9 + 8),
    (D_FACE * 9 + 5, R_FACE * 9 + 8, B_FACE * 9 + 6),
    (D_FACE * 9 + 1, L_FACE * 9 + 6, F_FACE * 9 + 8),
    (D_FACE * 9 + 3, R_FACE * 9 + 6, F_FACE * 9 + 6),
];

#[repr(C)]
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
    pub fn find_edge(&self, color1: u8, color2: u8) -> Option<(usize, u8)> {
        for (edge_pos, &(idx1, idx2)) in EDGES.iter().enumerate() {
            let sticker1 = self.stickers[idx1];
            let sticker2 = self.stickers[idx2];
            if sticker1 == color1 && sticker2 == color2 {
                return Some((edge_pos, 0));
            } else if sticker1 == color2 && sticker2 == color1 {
                return Some((edge_pos, 1));
            }
        }
        None
    }

    pub fn find_corner(&self, color1: u8, color2: u8, color3: u8) -> Option<(usize, u8)> {
        let colors = [color1, color2, color3];
        for (corner_pos, &(idx1, idx2, idx3)) in CORNERS.iter().enumerate() {
            let stickers = [self.stickers[idx1], self.stickers[idx2], self.stickers[idx3]];
            for rotation in 0..3 {
                let mut match_found = true;
                for i in 0..3 {
                    if stickers[(i + rotation) % 3] != colors[i] {
                        match_found = false;
                        break;
                    }
                }
                if match_found {
                    return Some((corner_pos, rotation as u8));
                }
            }
        }
        None
    }

    pub fn u(&mut self) {
        self.rotate_face_cw(U_FACE);
        let t0 = self.stickers[R_FACE * 9 + 0]; let t1 = self.stickers[R_FACE * 9 + 1]; let t2 = self.stickers[R_FACE * 9 + 2];
        self.stickers[R_FACE * 9 + 0] = self.stickers[B_FACE * 9 + 0];
        self.stickers[R_FACE * 9 + 1] = self.stickers[B_FACE * 9 + 1];
        self.stickers[R_FACE * 9 + 2] = self.stickers[B_FACE * 9 + 2];
        self.stickers[B_FACE * 9 + 0] = self.stickers[L_FACE * 9 + 0];
        self.stickers[B_FACE * 9 + 1] = self.stickers[L_FACE * 9 + 1];
        self.stickers[B_FACE * 9 + 2] = self.stickers[L_FACE * 9 + 2];
        self.stickers[L_FACE * 9 + 0] = self.stickers[F_FACE * 9 + 0];
        self.stickers[L_FACE * 9 + 1] = self.stickers[F_FACE * 9 + 1];
        self.stickers[L_FACE * 9 + 2] = self.stickers[F_FACE * 9 + 2];
        self.stickers[F_FACE * 9 + 0] = t0; self.stickers[F_FACE * 9 + 1] = t1; self.stickers[F_FACE * 9 + 2] = t2;
    }
    
    pub fn d(&mut self) {
        self.rotate_face_cw(D_FACE);
        let t6 = self.stickers[R_FACE * 9 + 6]; let t7 = self.stickers[R_FACE * 9 + 7]; let t8 = self.stickers[R_FACE * 9 + 8];
        self.stickers[R_FACE * 9 + 6] = self.stickers[F_FACE * 9 + 6]; self.stickers[R_FACE * 9 + 7] = self.stickers[F_FACE * 9 + 7]; self.stickers[R_FACE * 9 + 8] = self.stickers[F_FACE * 9 + 8];
        self.stickers[F_FACE * 9 + 6] = self.stickers[L_FACE * 9 + 6]; self.stickers[F_FACE * 9 + 7] = self.stickers[L_FACE * 9 + 7]; self.stickers[F_FACE * 9 + 8] = self.stickers[L_FACE * 9 + 8];
        self.stickers[L_FACE * 9 + 6] = self.stickers[B_FACE * 9 + 6]; self.stickers[L_FACE * 9 + 7] = self.stickers[B_FACE * 9 + 7]; self.stickers[L_FACE * 9 + 8] = self.stickers[B_FACE * 9 + 8];
        self.stickers[B_FACE * 9 + 6] = t6; self.stickers[B_FACE * 9 + 7] = t7; self.stickers[B_FACE * 9 + 8] = t8;
    }

    pub fn l(&mut self) {
        self.rotate_face_cw(L_FACE);
        let t0 = self.stickers[F_FACE * 9 + 0]; let t3 = self.stickers[F_FACE * 9 + 3]; let t6 = self.stickers[F_FACE * 9 + 6];
        self.stickers[F_FACE * 9 + 0] = self.stickers[U_FACE * 9 + 0]; self.stickers[F_FACE * 9 + 3] = self.stickers[U_FACE * 9 + 3]; self.stickers[F_FACE * 9 + 6] = self.stickers[U_FACE * 9 + 6];
        self.stickers[U_FACE * 9 + 0] = self.stickers[B_FACE * 9 + 8]; self.stickers[U_FACE * 9 + 3] = self.stickers[B_FACE * 9 + 5]; self.stickers[U_FACE * 9 + 6] = self.stickers[B_FACE * 9 + 2];
        self.stickers[B_FACE * 9 + 2] = self.stickers[D_FACE * 9 + 6]; self.stickers[B_FACE * 9 + 5] = self.stickers[D_FACE * 9 + 3]; self.stickers[B_FACE * 9 + 8] = self.stickers[D_FACE * 9 + 0];
        self.stickers[D_FACE * 9 + 0] = t0; self.stickers[D_FACE * 9 + 3] = t3; self.stickers[D_FACE * 9 + 6] = t6;
    }

    pub fn r(&mut self) {
        self.rotate_face_cw(R_FACE);
        let t6 = self.stickers[B_FACE * 9 + 6]; let t0 = self.stickers[B_FACE * 9 + 0]; let t3 = self.stickers[B_FACE * 9 + 3];
        self.stickers[B_FACE * 9 + 0] = self.stickers[U_FACE * 9 + 8]; self.stickers[B_FACE * 9 + 3] = self.stickers[U_FACE * 9 + 5]; self.stickers[B_FACE * 9 + 6] = self.stickers[U_FACE * 9 + 2];
        self.stickers[U_FACE * 9 + 2] = self.stickers[F_FACE * 9 + 2]; self.stickers[U_FACE * 9 + 5] = self.stickers[F_FACE * 9 + 5]; self.stickers[U_FACE * 9 + 8] = self.stickers[F_FACE * 9 + 8];
        self.stickers[F_FACE * 9 + 2] = self.stickers[D_FACE * 9 + 2]; self.stickers[F_FACE * 9 + 5] = self.stickers[D_FACE * 9 + 5]; self.stickers[F_FACE * 9 + 8] = self.stickers[D_FACE * 9 + 8];
        self.stickers[D_FACE * 9 + 2] = t6; self.stickers[D_FACE * 9 + 5] = t3; self.stickers[D_FACE * 9 + 8] = t0;
    }

    pub fn f(&mut self) {
        self.rotate_face_cw(F_FACE);
        let t6 = self.stickers[U_FACE * 9 + 6]; let t7 = self.stickers[U_FACE * 9 + 7]; let t8 = self.stickers[U_FACE * 9 + 8];
        self.stickers[U_FACE * 9 + 6] = self.stickers[L_FACE * 9 + 8]; self.stickers[U_FACE * 9 + 7] = self.stickers[L_FACE * 9 + 5]; self.stickers[U_FACE * 9 + 8] = self.stickers[L_FACE * 9 + 2];
        self.stickers[L_FACE * 9 + 2] = self.stickers[D_FACE * 9 + 0]; self.stickers[L_FACE * 9 + 5] = self.stickers[D_FACE * 9 + 1]; self.stickers[L_FACE * 9 + 8] = self.stickers[D_FACE * 9 + 2];
        self.stickers[D_FACE * 9 + 0] = self.stickers[R_FACE * 9 + 6]; self.stickers[D_FACE * 9 + 1] = self.stickers[R_FACE * 9 + 3]; self.stickers[D_FACE * 9 + 2] = self.stickers[R_FACE * 9 + 0];
        self.stickers[R_FACE * 9 + 0] = t6; self.stickers[R_FACE * 9 + 3] = t7; self.stickers[R_FACE * 9 + 6] = t8;
    }

    pub fn b(&mut self) {
        self.rotate_face_cw(B_FACE);
        let t0 = self.stickers[L_FACE * 9 + 0]; let t3 = self.stickers[L_FACE * 9 + 3]; let t6 = self.stickers[L_FACE * 9 + 6];
        self.stickers[L_FACE * 9 + 0] = self.stickers[U_FACE * 9 + 2]; self.stickers[L_FACE * 9 + 3] = self.stickers[U_FACE * 9 + 1]; self.stickers[L_FACE * 9 + 6] = self.stickers[U_FACE * 9 + 0];
        self.stickers[U_FACE * 9 + 0] = self.stickers[R_FACE * 9 + 2]; self.stickers[U_FACE * 9 + 1] = self.stickers[R_FACE * 9 + 5]; self.stickers[U_FACE * 9 + 2] = self.stickers[R_FACE * 9 + 8];
        self.stickers[R_FACE * 9 + 2] = self.stickers[D_FACE * 9 + 8]; self.stickers[R_FACE * 9 + 5] = self.stickers[D_FACE * 9 + 7]; self.stickers[R_FACE * 9 + 8] = self.stickers[D_FACE * 9 + 6];
        self.stickers[D_FACE * 9 + 6] = t0; self.stickers[D_FACE * 9 + 7] = t3; self.stickers[D_FACE * 9 + 8] = t6;
    }

    pub fn rotate_face_cw(&mut self, face: usize) {
        let base: usize = face * 9;
        let t0 = self.stickers[base + 0]; let t2 = self.stickers[base + 2]; let t6 = self.stickers[base + 6]; let t8 = self.stickers[base + 8];
        self.stickers[base + 0] = t6; self.stickers[base + 2] = t0; self.stickers[base + 6] = t8; self.stickers[base + 8] = t2;
        let t1 = self.stickers[base + 1]; let t3 = self.stickers[base + 3]; let t5 = self.stickers[base + 5]; let t7 = self.stickers[base + 7];
        self.stickers[base + 1] = t3; self.stickers[base + 3] = t7; self.stickers[base + 5] = t1; self.stickers[base + 7] = t5;
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
        }
    }
}

fn add_move(out: &mut [solver_move_t], move_count: &mut usize, m: solver_move_t) {
    if *move_count < out.len() {
        out[*move_count] = m;
        *move_count += 1;
    }
}

fn apply_and_record(cube: &mut Cube, out: &mut [solver_move_t], move_count: &mut usize, m: solver_move_t) {
    cube.apply_move(m);
    add_move(out, move_count, m);
}

fn solve_white_cross(cube: &mut Cube, out: &mut [solver_move_t], move_count: &mut usize) {
    // Get white edges to top, then align
    for _ in 0..4 {
        if let Some((pos, _)) = cube.find_edge(WHITE, RED) {
            if pos != 3 {
                // Not on top, bring to top
                match pos {
                    0 => apply_and_record(cube, out, move_count, solver_move_t::B2),
                    1 => apply_and_record(cube, out, move_count, solver_move_t::L2),
                    2 => apply_and_record(cube, out, move_count, solver_move_t::R2),
                    4|5|6|7 => apply_and_record(cube, out, move_count, solver_move_t::F2),
                    _ => {}
                }
            }
        }
    }

    for _ in 0..4 {
        if let Some((pos, _)) = cube.find_edge(WHITE, GREEN) {
            if pos != 1 {
                match pos {
                    0 => apply_and_record(cube, out, move_count, solver_move_t::B2),
                    2 => apply_and_record(cube, out, move_count, solver_move_t::R2),
                    3 => apply_and_record(cube, out, move_count, solver_move_t::F2),
                    4|5|6|7 => apply_and_record(cube, out, move_count, solver_move_t::L2),
                    _ => {}
                }
            }
        }
    }

    for _ in 0..4 {
        if let Some((pos, _)) = cube.find_edge(WHITE, BLUE) {
            if pos != 2 {
                match pos {
                    0 => apply_and_record(cube, out, move_count, solver_move_t::B2),
                    1 => apply_and_record(cube, out, move_count, solver_move_t::L2),
                    3 => apply_and_record(cube, out, move_count, solver_move_t::F2),
                    4|5|6|7 => apply_and_record(cube, out, move_count, solver_move_t::R2),
                    _ => {}
                }
            }
        }
    }

    for _ in 0..4 {
        if let Some((pos, _)) = cube.find_edge(WHITE, ORANGE) {
            if pos != 0 {
                match pos {
                    1 => apply_and_record(cube, out, move_count, solver_move_t::L2),
                    2 => apply_and_record(cube, out, move_count, solver_move_t::R2),
                    3 => apply_and_record(cube, out, move_count, solver_move_t::F2),
                    4|5|6|7 => apply_and_record(cube, out, move_count, solver_move_t::B2),
                    _ => {}
                }
            }
        }
    }

    // Align edges
    for _ in 0..4 {
        let mut all_good = true;
        if cube.stickers[F_FACE * 9 + 1] != RED { all_good = false; }
        if cube.stickers[L_FACE * 9 + 1] != GREEN { all_good = false; }
        if cube.stickers[R_FACE * 9 + 1] != BLUE { all_good = false; }
        if cube.stickers[B_FACE * 9 + 1] != ORANGE { all_good = false; }
        if all_good { break; }
        apply_and_record(cube, out, move_count, solver_move_t::U);
    }
}

fn solve_white_corners(cube: &mut Cube, out: &mut [solver_move_t], move_count: &mut usize) {
    let target_corners = [(WHITE, RED, BLUE), (WHITE, RED, GREEN), (WHITE, ORANGE, GREEN), (WHITE, ORANGE, BLUE)];
    for (c1, c2, c3) in &target_corners {
        for _ in 0..20 {
            if let Some(_) = cube.find_corner(*c1, *c2, *c3) {
                // Position above its target
                for _ in 0..5 {
                    if cube.stickers[U_FACE * 9 + 8] == WHITE { break; }
                    apply_and_record(cube, out, move_count, solver_move_t::U);
                }
                // Insert
                for _ in 0..5 {
                    if cube.stickers[D_FACE * 9 + 8] == WHITE { break; }
                    apply_and_record(cube, out, move_count, solver_move_t::R);
                    apply_and_record(cube, out, move_count, solver_move_t::U);
                    apply_and_record(cube, out, move_count, solver_move_t::Ri);
                    apply_and_record(cube, out, move_count, solver_move_t::Ui);
                }
                break;
            }
        }
    }
}

fn solve_middle_layer(cube: &mut Cube, out: &mut [solver_move_t], move_count: &mut usize) {
    let edge_pairs = [(RED, GREEN), (RED, BLUE), (ORANGE, GREEN), (ORANGE, BLUE)];
    for (c1, c2) in &edge_pairs {
        for _ in 0..10 {
            if let Some((pos, _)) = cube.find_edge(*c1, *c2) {
                if pos >= 8 { break; } // Already in middle
                
                // Get to top
                for _ in 0..5 {
                    let aligned = (*c1 == RED && *c2 == GREEN && cube.stickers[F_FACE * 9 + 1] == *c1 && cube.stickers[L_FACE * 9 + 1] == *c2) ||
                                 (*c1 == RED && *c2 == BLUE && cube.stickers[F_FACE * 9 + 1] == *c1 && cube.stickers[R_FACE * 9 + 1] == *c2) ||
                                 (*c1 == ORANGE && *c2 == GREEN && cube.stickers[B_FACE * 9 + 1] == *c1 && cube.stickers[L_FACE * 9 + 1] == *c2) ||
                                 (*c1 == ORANGE && *c2 == BLUE && cube.stickers[B_FACE * 9 + 1] == *c1 && cube.stickers[R_FACE * 9 + 1] == *c2);
                    if aligned { break; }
                    apply_and_record(cube, out, move_count, solver_move_t::U);
                }
                
                // Insert
                match (*c1, *c2) {
                    (RED, GREEN) => {
                        apply_and_record(cube, out, move_count, solver_move_t::Ui);
                        apply_and_record(cube, out, move_count, solver_move_t::Li);
                        apply_and_record(cube, out, move_count, solver_move_t::U);
                        apply_and_record(cube, out, move_count, solver_move_t::L);
                    }
                    (RED, BLUE) => {
                        apply_and_record(cube, out, move_count, solver_move_t::U);
                        apply_and_record(cube, out, move_count, solver_move_t::R);
                        apply_and_record(cube, out, move_count, solver_move_t::Ui);
                        apply_and_record(cube, out, move_count, solver_move_t::Ri);
                    }
                    (ORANGE, GREEN) => {
                        apply_and_record(cube, out, move_count, solver_move_t::U);
                        apply_and_record(cube, out, move_count, solver_move_t::L);
                        apply_and_record(cube, out, move_count, solver_move_t::Ui);
                        apply_and_record(cube, out, move_count, solver_move_t::Li);
                    }
                    (ORANGE, BLUE) => {
                        apply_and_record(cube, out, move_count, solver_move_t::Ui);
                        apply_and_record(cube, out, move_count, solver_move_t::Ri);
                        apply_and_record(cube, out, move_count, solver_move_t::U);
                        apply_and_record(cube, out, move_count, solver_move_t::R);
                    }
                    _ => {}
                }
                break;
            }
        }
    }
}

fn solve_yellow_cross(cube: &mut Cube, out: &mut [solver_move_t], move_count: &mut usize) {
    for _ in 0..3 {
        let mut count = 0;
        if cube.stickers[U_FACE * 9 + 1] == YELLOW { count += 1; }
        if cube.stickers[U_FACE * 9 + 3] == YELLOW { count += 1; }
        if cube.stickers[U_FACE * 9 + 5] == YELLOW { count += 1; }
        if cube.stickers[U_FACE * 9 + 7] == YELLOW { count += 1; }
        if count == 4 { break; }
        
        apply_and_record(cube, out, move_count, solver_move_t::F);
        apply_and_record(cube, out, move_count, solver_move_t::R);
        apply_and_record(cube, out, move_count, solver_move_t::U);
        apply_and_record(cube, out, move_count, solver_move_t::Ri);
        apply_and_record(cube, out, move_count, solver_move_t::Ui);
        apply_and_record(cube, out, move_count, solver_move_t::Fi);
    }
}

fn solve_yellow_corners_and_edges(cube: &mut Cube, out: &mut [solver_move_t], move_count: &mut usize) {
    for _ in 0..20 {
        if cube.is_solved() { break; }
        apply_and_record(cube, out, move_count, solver_move_t::R);
        apply_and_record(cube, out, move_count, solver_move_t::U);
        apply_and_record(cube, out, move_count, solver_move_t::Ri);
        apply_and_record(cube, out, move_count, solver_move_t::U);
    }
}

fn solve_internal(cube: &Cube, out: &mut [solver_move_t]) -> usize {
    if out.len() < 1 { return 0; }
    let mut cube_copy = *cube;
    let mut move_count: usize = 0;
    if cube_copy.is_solved() { return 0; }
    
    solve_white_cross(&mut cube_copy, out, &mut move_count);
    solve_white_corners(&mut cube_copy, out, &mut move_count);
    solve_middle_layer(&mut cube_copy, out, &mut move_count);
    solve_yellow_cross(&mut cube_copy, out, &mut move_count);
    solve_yellow_corners_and_edges(&mut cube_copy, out, &mut move_count);
    
    move_count
}

#[unsafe(no_mangle)]
pub extern "C" fn solve_cube(cube_raw: *const u8, out_moves: *mut solver_move_t, max_moves: usize) -> usize {
    if cube_raw.is_null() || out_moves.is_null() { return 0; }
    let cube: Cube = unsafe {
        let slice = slice::from_raw_parts(cube_raw, 54);
        let mut stickers = [0u8; 54];
        stickers.copy_from_slice(slice);
        Cube { stickers }
    };
    let out_slice: &mut [solver_move_t] = unsafe { slice::from_raw_parts_mut(out_moves, max_moves) };
    solve_internal(&cube, out_slice)
}

#[cfg(not(feature = "std-env"))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }
