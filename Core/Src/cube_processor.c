/*
 * cube_processor.c
 *
 *  Created on: Jan 27, 2026
 *      Author: jorgelarach
 */

#include "cube_processor.h"
#include "uart_cube.h"  // For rx_buffer
#include "cmsis_os.h" // for os_delay

cube_state_t cube_state = {0};

void cube_processor_init(void) {
    cube_state.cube_ready = false;
    cube_state.moves_ready = false;
    cube_state.motors_running = false;
    cube_state.move_count = 0;
}

void cube_process_uart_packet(void) {
    // Copy UART data to cube state
    for (int i = 0; i < 54; i++) {
        cube_state.cube_state[i] = rx_buffer[i];
    }
    cube_state.cube_ready = true;
    cube_state.moves_ready = false;  // Invalidate old moves
}

void cube_run_solver(void) {
    if (!cube_state.cube_ready) return;  // No cube to solve

    // Call Rust solver
    cube_state.move_count = solve_cube(
        cube_state.cube_state,
        cube_state.solver_moves,
        MAX_MOVES
    );

    // Translate solver_moves to motor_moves
    for (int i = 0; i < cube_state.move_count; i++) {
        cube_state.motor_moves[i] = translate_solver_move(cube_state.solver_moves[i]);
    }

    cube_state.moves_ready = true;
}

void cube_run_motors(void) {
    if (!cube_state.moves_ready || cube_state.motors_running) return;

    cube_state.motors_running = true;

    // Execute all motor moves
    for (int i = 0; i < cube_state.move_count; i++) {
        execute_cube_move(cube_state.motor_moves[i]);
    }

    cube_state.motors_running = false;
}

// Translation function (solver_move → stepper_move_t)
stepper_move_t translate_solver_move(solver_move move) {
    stepper_move_t result = {0};

    switch(move) {
        // U face moves
        case MOVE_U:    result.motor = MOTOR_U; result.dir = TURN_CW;  break;
        case MOVE_Ui:   result.motor = MOTOR_U; result.dir = TURN_CCW; break;
        case MOVE_U2:   // Handle double moves
        // Add all 18 cases...
        // D face moves
        case MOVE_D:    result.motor = MOTOR_D; result.dir = TURN_CW;  break;
        case MOVE_Di:   result.motor = MOTOR_D; result.dir = TURN_CCW; break;
        // etc...
        default: break;
    }

    return result;
}

// Execute a single motor move
void execute_cube_move(stepper_move_t move) {
    // For now, just move 90 degrees
    // You can expand this for 180 degree moves (U2, etc.)
    stepper_move_90(move.motor, move.dir);

    // Small delay between moves
    osDelay(300);  // Adjust as needed
}
