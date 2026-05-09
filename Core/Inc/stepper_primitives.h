/*
 * stepper_primitives.h
 *
 *  Created on: Jan 20, 2026
 *      Author: jorgelarach
 */

#ifndef STEPPER_PRIMITIVES_H
#define STEPPER_PRIMITIVES_H

#include <stdint.h>

#define MOTOR_COUNT 6

typedef enum {
	MOTOR_U,
	MOTOR_D,
	MOTOR_L,
	MOTOR_R,
	MOTOR_F,
	MOTOR_B,
} motor_id_t;

typedef enum {
	TURN_CW,
	TURN_CCW
} turn_dir_t;

typedef enum {
	TURN_90_DEG,
	TURN_180_DEG
} turn_degrees_t;

typedef struct {
	motor_id_t motor;
	turn_dir_t dir;
	turn_degrees_t degrees;
} stepper_move_t;

#endif /* STEPPER_PRIMITIVES_H */
