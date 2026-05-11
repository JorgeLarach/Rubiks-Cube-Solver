/*
 * cube_processor.c
 *
 *  Created on: Jan 27, 2026
 *      Author: jorgelarach
 */

#include "cube_processor.h"
#include "uart_cube.h"  // for rx_buffer
#include "cmsis_os.h" // for os_delay
#include "board_pins.h" // for button pins
#include "stepper_timer.h" // for tim3_enable

cube_state_t cube_state = {0};
extern UART_HandleTypeDef huart2;

void CubeProcessStart(void *argument)
{
  /* USER CODE BEGIN 5 */
  cube_processor_init();
  stepper_tim3_enable_ir();
  stepper_init_all();
  uart_start_reception(&huart2);
  /* Infinite loop */
  for(;;)
  {
    if(rx_ready){
	  rx_ready = 0;
	  cube_process_uart_packet();
    }
	osDelay(1000);
  }
  /* USER CODE END 5 */
}

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

    cube_run_solver(); // Not meant to go here obviously, just testing
    osDelay(1000);
    cube_run_motors(); // Not meant to go here obviously, just testing
}

void cube_run_solver(void) {
    if (!cube_state.cube_ready) return;  // No cube to solve

    HAL_UART_Transmit(&huart2, "SOLVING\r\n", 9, 100);
    uint32_t start = HAL_GetTick();

    // Call Rust solver
    cube_state.move_count = solve_cube(
        cube_state.cube_state,
        cube_state.solver_moves,
        MAX_MOVES
    );

    uint32_t elapsed = HAL_GetTick() - start;
    char buf[32];
    sprintf(buf, "DONE %d moves %dms\r\n", cube_state.move_count, elapsed);
    HAL_UART_Transmit(&huart2, buf, strlen(buf), 100);

    // Translate solver_moves_t to motor_moves_t
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

// Translation function (solver_move_t -> stepper_move_t)
stepper_move_t translate_solver_move(solver_move_t move) {
    stepper_move_t result = {0};

    switch(move) {
        // U face moves
        case MOVE_U:    result.motor = MOTOR_U; result.dir = TURN_CW;  result.degrees = TURN_90_DEG;  break;
        case MOVE_Ui:   result.motor = MOTOR_U; result.dir = TURN_CCW; result.degrees = TURN_90_DEG;  break;
        case MOVE_U2:   result.motor = MOTOR_U; result.dir = TURN_CW;  result.degrees = TURN_180_DEG; break;

        // D face moves
        case MOVE_D:    result.motor = MOTOR_D; result.dir = TURN_CW;  result.degrees = TURN_90_DEG;  break;
        case MOVE_Di:   result.motor = MOTOR_D; result.dir = TURN_CCW; result.degrees = TURN_90_DEG;  break;
        case MOVE_D2:   result.motor = MOTOR_D; result.dir = TURN_CW;  result.degrees = TURN_180_DEG; break;

        // L face moves
        case MOVE_L:    result.motor = MOTOR_L; result.dir = TURN_CW;  result.degrees = TURN_90_DEG;  break;
        case MOVE_Li:   result.motor = MOTOR_L; result.dir = TURN_CCW; result.degrees = TURN_90_DEG;  break;
        case MOVE_L2:   result.motor = MOTOR_L; result.dir = TURN_CW;  result.degrees = TURN_180_DEG; break;

        // R face moves
        case MOVE_R:    result.motor = MOTOR_R; result.dir = TURN_CW;  result.degrees = TURN_90_DEG;  break;
        case MOVE_Ri:   result.motor = MOTOR_R; result.dir = TURN_CCW; result.degrees = TURN_90_DEG;  break;
        case MOVE_R2:   result.motor = MOTOR_R; result.dir = TURN_CW;  result.degrees = TURN_180_DEG; break;

        // F face moves
        case MOVE_F:    result.motor = MOTOR_F; result.dir = TURN_CW;  result.degrees = TURN_90_DEG;  break;
        case MOVE_Fi:   result.motor = MOTOR_F; result.dir = TURN_CCW; result.degrees = TURN_90_DEG;  break;
        case MOVE_F2:   result.motor = MOTOR_F; result.dir = TURN_CW;  result.degrees = TURN_180_DEG; break;

        // B face moves
        case MOVE_B:    result.motor = MOTOR_B; result.dir = TURN_CW;  result.degrees = TURN_90_DEG;  break;
        case MOVE_Bi:   result.motor = MOTOR_B; result.dir = TURN_CCW; result.degrees = TURN_90_DEG;  break;
        case MOVE_B2:   result.motor = MOTOR_B; result.dir = TURN_CW;  result.degrees = TURN_180_DEG; break;

        default: break;
    }

    return result;
}

// Execute a single motor move
void execute_cube_move(stepper_move_t move) {
    stepper_move(move.motor, move.dir, move.degrees);

    while (stepper_is_busy(move.motor)) {
        osDelay(1);
    }
    osDelay(500); // small settling delay between moves
}

// Interrupt callback for control button presses
void HAL_GPIO_EXTI_Callback(uint16_t GPIO_Pin) {
    if (GPIO_Pin == SOLVE_BUTTON_Pin) {
        // Solve button pressed - run solver
        cube_run_solver(); // maybe turn this into a flag and call function in cube process task
    }
    else if (GPIO_Pin == EXECUTE_BUTTON_Pin) {
        // Execute button pressed - run motors
        cube_run_motors();
    }
}
