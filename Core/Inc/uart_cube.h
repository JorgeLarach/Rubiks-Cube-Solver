/*
 * uart_cube.h
 *
 *  Created on: Jan 26, 2026
 *      Author: jorgelarach
 */

#ifndef UART_CUBE_H
#define UART_CUBE_H

#include <stdint.h>
#include <string.h>
#include "stm32f4xx_hal.h"

#define PACKET_SIZE 54

extern uint8_t rx_buffer[PACKET_SIZE];
extern volatile uint8_t rx_ready;
extern uint8_t rx_index;

void uart_start_reception(UART_HandleTypeDef *huart);
void uart_process_packet(uint8_t *cube);
#endif /* UART_CUBE_H */
