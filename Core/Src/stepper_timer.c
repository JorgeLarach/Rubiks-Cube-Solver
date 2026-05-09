/*
 * stepper_timer.c
 *
 *  Created on: Jan 19, 2026
 *      Author: jorgelarach
 */

#include "stepper_timer.h"
#include "stepper.h"

extern volatile motor_id_t active_motor;

// For TIM3 configuration info, check MX_TIM3_Init() in main.c

void stepper_tim3_irqhandler(void){
	static uint8_t step_level = 0; // Tracks rising or falling STEP edge
	if (TIM3->SR & TIM_SR_UIF){ // If Update Interrupt Flag (UIF) bit is set in the status register, then interrupt requested
		TIM3->SR &= ~TIM_SR_UIF; // clears UIF

		stepper_t *m = &steppers[active_motor]; // Pointer to active motor

		if(m->steps_remaining > 0){ // If the active motor hasn't finished its 90 degree turn
			/* Toggle STEP pin */
			HAL_GPIO_WritePin(
				m->step_port,
				m->step_pin,
				step_level ? GPIO_PIN_RESET : GPIO_PIN_SET // If step is high, transition low
			);

			// If step is HIGH, decrement. Do nothing when step is low (this is why step rate is 1/2 TIM3 frequency)
			if(step_level) m->steps_remaining--;

			step_level ^= 1; // Invert step_level
		} else {
			// When motor has finished turning, set STEP pin LOW and stop timer
			HAL_GPIO_WritePin(
				m->step_port,
				m->step_pin,
				GPIO_PIN_RESET
			);

			stepper_tim3_stop();
		}
	}
}

void stepper_tim3_start(void){
    TIM3->CNT = 0;
    TIM3->CR1 |= TIM_CR1_CEN;
}

void stepper_tim3_stop(void){
	TIM3->CR1 &= ~TIM_CR1_CEN;
}

void stepper_tim3_enable_ir(void){
	TIM3->DIER |= TIM_DIER_UIE;  // enable update interrupt
}
