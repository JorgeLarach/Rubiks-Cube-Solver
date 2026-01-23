#[repr(C)] // Lay out this enum/struct in memory exactly like C would
#[derive(Copy, Clone, Debug)]
pub enum Move {
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

fn solve_internal(_cube: &Cube, out: &mut [Move]) -> usize{
    if out.len() < 4{
        return 0;
    }

    out[0] = Move::R;
    out[1] = Move::Bi;
    out[2] = Move::D;
    out[3] = Move::F2;

    4
}

#[unsafe(no_mangle)] // Prevents function renaming (mangling) during compiling. C expects symbol named solve_cube
pub extern "C" fn solve_cube(
    cube_raw: *const u8,
    out_moves: *mut Move,
    max_moves: usize
) -> usize {
    let cube = unsafe { // unsafe bc cube_raw = raw pointer and those can be invalid
        Cube {
            stickers: *(cube_raw as *const [u8; 54])
        }
    };
    let out_slice = unsafe {
        // Builds a mutable slice from raw pointer and length
        std::slice::from_raw_parts_mut(out_moves, max_moves)
    };

    solve_internal(&cube, out_slice)
}








pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
