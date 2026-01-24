/*
 * cube_solver.h
 *
 *  Created on: Jan 23, 2026
 *      Author: jorgelarach
 */

#ifndef CUBE_SOLVER_H
#define CUBE_SOLVER_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    MOVE_U = 0,
    MOVE_Ui,
    MOVE_U2,
    MOVE_D,
    MOVE_Di,
    MOVE_D2,
    MOVE_L,
    MOVE_Li,
    MOVE_L2,
    MOVE_R,
    MOVE_Ri,
    MOVE_R2,
    MOVE_F,
    MOVE_Fi,
    MOVE_F2,
    MOVE_B,
    MOVE_Bi,
    MOVE_B2,
} Move;


// Rust signature:
// extern "C" fn solve_cube(stickers: *const u8,
//                          out_moves: *mut Move,
//                          max_moves: usize) -> usize;

uint32_t solve_cube(
		const uint8_t *cube_raw,
		Move *out_moves,
		size_t max_moves
);


#ifdef __cplusplus
}
#endif
#endif /* CUBE_SOLVER_H_ */
