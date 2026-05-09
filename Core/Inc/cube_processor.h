/*
 * cube_processing.h
 *
 *  Created on: Jan 27, 2026
 *      Author: jorgelarach
 */

#ifndef CUBE_PROCESSOR_H
#define CUBE_PROCESSOR_H

#include <stdint.h>
#include <stdbool.h>
#include "cube_solver.h"
#include "stepper.h" // for stepper_move_t

#define MAX_MOVES 80

typedef struct {
	bool cube_ready; // UART packet processed into cube_state array
	bool moves_ready; // Rust solver populated solver_moves
	bool motors_running; // motors are executing moves

	uint8_t cube_state[54];
	solver_move_t solver_moves[MAX_MOVES];
	stepper_move_t motor_moves[MAX_MOVES];
	uint16_t move_count;

} cube_state_t;

extern cube_state_t cube_state;

void cube_processor_init(void);
void cube_process_uart_packet(void); // called when uart data arrives
void cube_run_solver(void); // called when solve button pressed
void cube_run_motors(void); // called when run motors button pressed
stepper_move_t translate_solver_move(solver_move_t move);
void execute_cube_move(stepper_move_t move);

#endif /* CUBE_PROCESSOR_H */
