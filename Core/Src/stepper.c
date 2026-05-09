/*
 * stepper.c
 *
 *  Created on: Jan 20, 2026
 *      Author: jorgelarach
 */

#include "stepper.h"
#include "stepper_timer.h"
#include "board_pins.h"

#include "FreeRTOS.h"
#include "task.h"

#define FULL_STEPS_PER_REV 200
#define MICROSTEPS 16
#define STEPS_90_DEG ((FULL_STEPS_PER_REV * MICROSTEPS) / 4)
#define STEPS_180_DEG STEPS_90_DEG * 2

stepper_t steppers[MOTOR_COUNT]; // Define steppers array
volatile motor_id_t active_motor = MOTOR_U;

void stepper_init_all(void){ // Fill steppers array with initialized data

	steppers[MOTOR_U] = (stepper_t){
		.step_port = U_STEP_GPIO_Port,
		.step_pin  = U_STEP_Pin,
		.dir_port  = U_DIR_GPIO_Port,
		.dir_pin   = U_DIR_Pin
	};
	steppers[MOTOR_D] = (stepper_t){
		.step_port = D_STEP_GPIO_Port,
		.step_pin  = D_STEP_Pin,
		.dir_port  = D_DIR_GPIO_Port,
		.dir_pin   = D_DIR_Pin
	};
	steppers[MOTOR_L] = (stepper_t){
		.step_port = L_STEP_GPIO_Port,
		.step_pin  = L_STEP_Pin,
		.dir_port  = L_DIR_GPIO_Port,
		.dir_pin   = L_DIR_Pin
	};
	steppers[MOTOR_R] = (stepper_t){
		.step_port = R_STEP_GPIO_Port,
		.step_pin  = R_STEP_Pin,
		.dir_port  = R_DIR_GPIO_Port,
		.dir_pin   = R_DIR_Pin
	};
	steppers[MOTOR_F] = (stepper_t){
		.step_port = F_STEP_GPIO_Port,
		.step_pin  = F_STEP_Pin,
		.dir_port  = F_DIR_GPIO_Port,
		.dir_pin   = F_DIR_Pin
	};
	steppers[MOTOR_B] = (stepper_t){
		.step_port = B_STEP_GPIO_Port,
		.step_pin  = B_STEP_Pin,
		.dir_port  = B_DIR_GPIO_Port,
		.dir_pin   = B_DIR_Pin
	};
}

void stepper_move(motor_id_t motor, turn_dir_t dir, turn_degrees_t deg){
	stepper_t *m = &steppers[motor]; // Pointer to motor in question
	active_motor = motor;


	// Set/Reset DIR pin
	HAL_GPIO_WritePin(
			m->dir_port,
			m->dir_pin,
			(dir == TURN_CW) ? GPIO_PIN_SET : GPIO_PIN_RESET);

	// Critical section: assigning steps
	taskENTER_CRITICAL();
	m->steps_remaining = (deg == TURN_90_DEG) ? STEPS_90_DEG : STEPS_180_DEG;
	taskEXIT_CRITICAL();

	stepper_tim3_start();
}

uint8_t stepper_is_busy(motor_id_t motor){
	return steppers[motor].steps_remaining > 0;
}
