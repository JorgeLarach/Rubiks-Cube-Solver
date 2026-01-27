/*
 * uart_cube.c
 *
 *  Created on: Jan 26, 2026
 *      Author: jorgelarach
 */

#include "uart_cube.h"

uint8_t rx_buffer[PACKET_SIZE] = {0};
volatile uint8_t rx_ready = 0;
uint8_t rx_index = 0;


void uart_start_reception(UART_HandleTypeDef *huart){
	HAL_UART_Receive_IT(huart, &rx_buffer[0], 1);
}

// called automatically when data arrives
void HAL_UART_RxCpltCallback(UART_HandleTypeDef *huart) {
    if (huart->Instance == USART2) {
        rx_index++;

        if (rx_index >= PACKET_SIZE) {
            rx_ready = 1;
            rx_index = 0;
        }

        // Listen for next byte
        HAL_UART_Receive_IT(huart, &rx_buffer[rx_index], 1);
    }
}

void HAL_UART_ErrorCallback(UART_HandleTypeDef *huart) {
    if (huart->Instance == USART2) {
        rx_index = 0;
        HAL_UART_Receive_IT(huart, &rx_buffer[rx_index], 1);
    }
}
