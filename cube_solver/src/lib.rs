#![no_std]

use core::slice;
use core::panic::PanicInfo;

#[repr(C)] // Lay out this enum/struct in memory exactly like C would
#[derive(Copy, Clone, Debug)]

#[repr(C)]

pub enum solver_move {
    U, Ui, U2,
    D, Di, D2,
    L, Li, L2,
    R, Ri, R2,
    F, Fi, F2,
    B, Bi, B2
}

pub struct Cube {
    pub stickers: [u8; 54],
}

fn solve_internal(_cube: &Cube, out: &mut [solver_move]) -> usize{
    if out.len() < 4{
        return 0;
    }

    // println!("solve_internal called with out.len() = {}", out.len());

    

    out[0] = solver_move::R;
    out[1] = solver_move::Bi;
    out[2] = solver_move::D;
    out[3] = solver_move::F2;

    4
}

#[unsafe(no_mangle)] // Prevents function renaming (mangling) during compiling. C expects symbol named solve_cube
pub extern "C" fn solve_cube(
    cube_raw: *const u8,
    out_moves: *mut solver_move,
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

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}