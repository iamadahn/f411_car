#include "rc_controller.h"
#include "drivers/nrf24l01/nrf24l01.h"
#include "rtos.h"

void
rc_controller_task(void* argument) {
    nrf24_t nrf24 = {
        .spi = SPI1,
        .ce_port = GPIOB,
        .ce_pin =  LL_GPIO_PIN_0,
        .cs_port = GPIOB,
        .cs_pin = LL_GPIO_PIN_1,
    };
    uint8_t nrf24_connect_state = nrf24_is_connected(&nrf24);
    
    nrf24_connect_state = nrf24_init(&nrf24);
    xQueueOverwrite(nrf24_state_queue, &nrf24_connect_state);

    while (1) {
        vTaskDelay(pdMS_TO_TICKS(5));
    }
}
