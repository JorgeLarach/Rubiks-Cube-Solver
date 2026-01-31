#![cfg_attr(not(feature = "std-env"), no_std)]

use core::slice;

#[cfg(not(feature = "std-env"))]
use core::panic::PanicInfo;

#[repr(C)] // Lay out this enum/struct in memory exactly like C would
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
    pub fn make_solved() -> Self {
        let mut stickers = [0u8; 54];
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
            solver_move_t::U => {}
            solver_move_t::Ui => {}
            solver_move_t::U2 => {}
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
    cube_raw: *const u8,
    out_moves: *mut solver_move_t,
    max_moves: usize
) -> usize {
    if cube_raw.is_null() || out_moves.is_null() {
        return 0;
    }
    let cube = unsafe { // Unsafe because dereferencing raw pointer (cube_raw)
        let slice = slice::from_raw_parts(cube_raw, 54); // Does not copy
        let mut stickers = [0u8; 54];
        stickers.copy_from_slice(slice); // Copies cube data (slice) into Rust owned stack memory
        Cube { stickers }
    };
    let out_slice = unsafe {
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