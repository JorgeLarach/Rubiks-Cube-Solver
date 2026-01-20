/*
 * stepper_timer.c
 *
 *  Created on: Jan 19, 2026
 *      Author: jorgelarach
 */
#include "stm32f4xx.h"
#include <stdint.h>

#include "stepper_timer.h"
//#include "stepper.h"
#include "FreeRTOS.h"
#include "task.h"

void Stepper_TIM3_IRQHandler(void){
	 if (TIM3->SR & TIM_SR_UIF){
		 TIM3->SR &= ~TIM_SR_UIF;
//		 static uint8_t step_level = 0;

		 HAL_GPIO_TogglePin(GPIOA, GPIO_PIN_0);
//		 HAL_GPIO_TogglePin(LD2_GPIO_Port, LD2_Pin);

	 }

}
