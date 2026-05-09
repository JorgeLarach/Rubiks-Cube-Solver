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
<<<<<<< Updated upstream
<<<<<<< Updated upstream
#define U_DIR_GPIO_Port  GPIOC
#define U_DIR_Pin   GPIO_PIN_2
=======
#define U_DIR_GPIO_Port  GPIOA
#define U_DIR_Pin   GPIO_PIN_1
>>>>>>> Stashed changes
=======
#define U_DIR_GPIO_Port  GPIOA
#define U_DIR_Pin   GPIO_PIN_1
>>>>>>> Stashed changes

#define L_STEP_GPIO_Port GPIOA
#define L_STEP_Pin  GPIO_PIN_4
#define L_DIR_GPIO_Port  GPIOB
#define L_DIR_Pin   GPIO_PIN_0

#define B_STEP_GPIO_Port GPIOC
#define B_STEP_Pin  GPIO_PIN_1
#define B_DIR_GPIO_Port  GPIOC
#define B_DIR_Pin   GPIO_PIN_0

#define D_STEP_GPIO_Port GPIOB
#define D_STEP_Pin  GPIO_PIN_3
#define D_DIR_GPIO_Port  GPIOA
#define D_DIR_Pin  GPIO_PIN_10

#define R_STEP_GPIO_Port GPIOB
#define R_STEP_Pin  GPIO_PIN_4
#define R_DIR_GPIO_Port  GPIOB
#define R_DIR_Pin   GPIO_PIN_5

#define F_STEP_GPIO_Port GPIOA
#define F_STEP_Pin  GPIO_PIN_8
#define F_DIR_GPIO_Port  GPIOB
#define F_DIR_Pin  GPIO_PIN_10

#define EN_GPIO_Port     GPIOC
#define EN_Pin      GPIO_PIN_7

#define SOLVE_BUTTON_GPIO_Port      GPIOA
#define SOLVE_BUTTON_Pin       GPIO_PIN_6

#define EXECUTE_BUTTON_GPIO_Port    GPIOA
#define EXECUTE_BUTTON_Pin     GPIO_PIN_7

//wiring order:
//U, L, B, D, R, F

#endif /* BOARD_PINS_H */
