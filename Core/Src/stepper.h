/*
 * stepper.h
 *
 *  Created on: Jan 20, 2026
 *      Author: jorgelarach
 */

#ifndef STEPPER_H
#define STEPPER_H

#include <stdint.h>
#include "stm32f4xx_hal.h"
#include "cube_primitives.h"
#include "stepper_timer.h"

typedef struct {
	GPIO_TypeDef *step_port;
	uint16_t step_pin;

	GPIO_TypeDef *dir_port;
	uint16_t dir_pin;

	volatile int32_t steps_remaining;
} stepper_t;

void stepper_init_all(void);
void stepper_move_90(motor_id_t motor, turn_dir_t dir);

extern stepper_t steppers[MOTOR_COUNT];
extern volatile motor_id_t active_motor; // SHOULD EVENTUALLY LIVE IN MOTORTASK.c

#endif /* STEPPER_H */
