/*
 * stepper.h
 *
 *  Created on: Jan 20, 2026
 *      Author: jorgelarach
 */

#ifndef STEPPER_H
#define STEPPER_H

#include <stepper_primitives.h>
#include "stm32f4xx_hal.h"

typedef struct {
	GPIO_TypeDef *step_port;
	uint16_t step_pin;

	GPIO_TypeDef *dir_port;
	uint16_t dir_pin;

	volatile int32_t steps_remaining;
} stepper_t;

void stepper_init_all(void);
void stepper_move(motor_id_t motor, turn_dir_t dir, turn_degrees_t deg);
uint8_t stepper_is_busy(motor_id_t motor);

extern stepper_t steppers[MOTOR_COUNT];
extern volatile motor_id_t active_motor; // SHOULD EVENTUALLY LIVE IN MOTORTASK.c

#endif /* STEPPER_H */
