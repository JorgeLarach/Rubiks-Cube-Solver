/*
 * stepper_timer.h
 *
 *  Created on: Jan 19, 2026
 *      Author: jorgelarach
 */

#ifndef STEPPER_TIMER_H
#define STEPPER_TIMER_H

void stepper_tim3_irqhandler(void);
void stepper_tim3_start(void);
void stepper_tim3_stop(void);
void stepper_tim3_enable_ir(void);

#endif /* STEPPER_TIMER_H */
