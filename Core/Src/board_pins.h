/*
 * board_pins.h
 *
 *  Created on: Jan 20, 2026
 *      Author: jorgelarach
 */

#ifndef BOARD_PINS_H
#define BOARD_PINS_H

#include "stm32f4xx_hal.h"

#define U_STEP_GPIO_Port GPIOA
#define U_STEP_Pin  GPIO_PIN_0
#define U_DIR_GPIO_Port  GPIOA
#define U_DIR_Pin   GPIO_PIN_1

#define D_STEP_GPIO_Port GPIOA
#define D_STEP_Pin  GPIO_PIN_4
#define D_DIR_GPIO_Port  GPIOB
#define D_DIR_Pin   GPIO_PIN_0

#define L_STEP_GPIO_Port GPIOC
#define L_STEP_Pin  GPIO_PIN_1
#define L_DIR_GPIO_Port  GPIOC
#define L_DIR_Pin   GPIO_PIN_0

#define R_STEP_GPIO_Port GPIOB
#define R_STEP_Pin  GPIO_PIN_3
#define R_DIR_GPIO_Port  GPIOA
#define R_DIR_Pin  GPIO_PIN_10

#define F_STEP_GPIO_Port GPIOB
#define F_STEP_Pin  GPIO_PIN_4
#define F_DIR_GPIO_Port  GPIOB
#define F_DIR_Pin   GPIO_PIN_5

#define B_STEP_GPIO_Port GPIOA
#define B_STEP_Pin  GPIO_PIN_8
#define B_DIR_GPIO_Port  GPIOB
#define B_DIR_Pin  GPIO_PIN_10

#define EN_GPIO_Port     GPIOA
#define EN_GPIO_PIN GPIO_PIN_9

#endif /* BOARD_PINS_H */
