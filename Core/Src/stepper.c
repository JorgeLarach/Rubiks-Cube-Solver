/*
 * stepper.c
 *
 *  Created on: Jan 20, 2026
 *      Author: jorgelarach
 */

#include "stepper.h"
#include "board_pins.h"

#include "FreeRTOS.h"
#include "task.h"

#define FULL_STEPS_PER_REV 200
#define MICROSTEPS 16
#define STEPS_90_DEG ((FULL_STEPS_PER_REV * MICROSTEPS) / 4)

stepper_t steppers[MOTOR_COUNT]; // Define steppers array
volatile motor_id_t active_motor = MOTOR_U;

void stepper_init_all(void){ // Fill steppers array with initialized data

	steppers[MOTOR_U] = (stepper_t){
		.step_port = U_STEP_GPIO_Port,
		.step_pin  = U_STEP_Pin,
		.dir_port  = U_DIR_GPIO_Port,
		.dir_pin   = U_DIR_Pin
	};

}

void stepper_move_90(motor_id_t motor, turn_dir_t dir){
	stepper_t *m = &steppers[motor]; // Pointer to motor in question

	// Set/Reset DIR pin
	HAL_GPIO_WritePin(
			m->dir_port,
			m->dir_pin,
			(dir == TURN_CW) ? GPIO_PIN_SET : GPIO_PIN_RESET);

	// Critical section: assigning steps
	taskENTER_CRITICAL();
	m->steps_remaining = STEPS_90_DEG;
	taskEXIT_CRITICAL();

	stepper_tim3_start();
}
