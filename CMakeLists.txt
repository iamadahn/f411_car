cmake_minimum_required(VERSION 3.17)
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_VERSION 1)

set(CMAKE_C_COMPILER arm-none-eabi-gcc)
set(CMAKE_CXX_COMPILER arm-none-eabi-g++)
set(CMAKE_ASM_COMPILER arm-none-eabi-gcc)
set(CMAKE_AR arm-none-eabi-ar)
set(CMAKE_OBJCOPY arm-none-eabi-objcopy)
set(CMAKE_OBJDUMP arm-none-eabi-objdump)
set(SIZE arm-none-eabi-size)
set(CMAKE_TRY_COMPILE_TARGET_TYPE STATIC_LIBRARY)

# Add you project name
string(REGEX MATCH "[^\/]+$" BUFF_FOR_PROJECT_NAME ${CMAKE_CURRENT_SOURCE_DIR})
project(${BUFF_FOR_PROJECT_NAME} C CXX ASM)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_C_STANDARD 11)

add_compile_options(-mcpu=cortex-m4 -mfloat-abi=hard -mfpu=fpv4-sp-d16 -mthumb -mthumb-interwork)
add_compile_options(-ffunction-sections -fdata-sections -fno-common -fmessage-length=0)
add_compile_options(-lc -lm -lnosys -specs=nano.specs -specs=nosys.specs)

# Uncomment to mitigate c++17 absolute addresses warnings
#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wno-register")

if ("${CMAKE_BUILD_TYPE}" STREQUAL "Release")
    message(STATUS "Maximum optimization for speed")
    add_compile_options(-Ofast)
elseif ("${CMAKE_BUILD_TYPE}" STREQUAL "RelWithDebInfo")
    message(STATUS "Maximum optimization for speed, debug info included")
    add_compile_options(-Ofast -g)
elseif ("${CMAKE_BUILD_TYPE}" STREQUAL "MinSizeRel")
    message(STATUS "Maximum optimization for size")
    add_compile_options(-Os)
else ()
    message(STATUS "Minimal optimization, debug info included")
    add_compile_options(-O0 -g)
endif ()

set(DEFINES
    -DSTM32F411xE
    -DUSE_FULL_LL_DRIVER
    -DCFG_TUSB_MCU=OPT_MCU_STM32F4
    -DLOGGER_BUF_SIZE=128
)

add_definitions(${DEFINES})

# Add Include directories
include_directories(
    ${CMAKE_SOURCE_DIR}
    ${CMAKE_CURRENT_SOURCE_DIR}/core/inc
    ${CMAKE_CURRENT_SOURCE_DIR}/drivers/cmsis/inc
    ${CMAKE_CURRENT_SOURCE_DIR}/drivers/ll/inc
    ${CMAKE_CURRENT_SOURCE_DIR}/rtos
)

# Add you source file
file(GLOB SOURCES
    startup_stm32f411xe.s
    "${CMAKE_CURRENT_SOURCE_DIR}/core/src/*.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/drivers/cmsis/src/*.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/drivers/ll/src/*.c"
    "${CMAKE_CURRENT_SOURCE_DIR}/rtos/*.c"
)


# Add lincer file
file(GLOB LINKER_SCRIPT "STM32*.ld")
message(STATUS "Found linker script - ${LINKER_SCRIPT}")

# this options for C++
add_link_options(-specs=nano.specs -specs=nosys.specs -lc -lm)
add_link_options(-Wl,-gc-sections,--print-memory-usage,-Map=${PROJECT_BINARY_DIR}/${PROJECT_NAME}.map)
add_link_options(-mcpu=cortex-m4 -mfloat-abi=hard -mfpu=fpv4-sp-d16 -mthumb -mthumb-interwork)
add_link_options(-T ${LINKER_SCRIPT})

add_executable(${PROJECT_NAME}.elf ${SOURCES} ${LINKER_SCRIPT})

add_library(root INTERFACE)
target_include_directories(root INTERFACE ${CMAKE_SOURCE_DIR})
target_compile_definitions(root INTERFACE ${DEFINES})

add_subdirectory(${CMAKE_SOURCE_DIR}/drivers/led)
add_subdirectory(${CMAKE_SOURCE_DIR}/drivers/nrf24l01)
add_subdirectory(${CMAKE_SOURCE_DIR}/drivers/logger)

target_link_libraries(${PROJECT_NAME}.elf
    led_driver
    nrf24l01_driver
    logger_driver
)

add_subdirectory(${CMAKE_SOURCE_DIR}/modules/led_controller)
add_subdirectory(${CMAKE_SOURCE_DIR}/modules/rc_controller)
add_subdirectory(${CMAKE_SOURCE_DIR}/modules/usbd_controller)
add_subdirectory(${CMAKE_SOURCE_DIR}/modules/motor_controller)

target_link_libraries(${PROJECT_NAME}.elf
    led_controller_module
    rc_controller_module
    usbd_controller_module
    motor_controller_module
)

add_library(freertos_config INTERFACE)
target_include_directories(freertos_config SYSTEM
    INTERFACE
        ${CMAKE_SOURCE_DIR}/core/inc)
target_compile_definitions(freertos_config
    INTERFACE
        projCOVERAGE_TEST=0
)

set( FREERTOS_HEAP "4" CACHE STRING "" FORCE)
# Select the cross-compile PORT
set(FREERTOS_PORT "GCC_ARM_CM4F" CACHE STRING "" FORCE)

add_subdirectory(${CMAKE_SOURCE_DIR}/FreeRTOS-Kernel)

target_link_libraries(${PROJECT_NAME}.elf
    freertos_kernel
)

add_library(tinyusb_config)
target_include_directories(tinyusb_config PRIVATE
	${CMAKE_SOURCE_DIR}/tinyusb/src
	${CMAKE_SOURCE_DIR}/core/src
	${CMAKE_SOURCE_DIR}/core/inc
	${CMAKE_SOURCE_DIR}/modules/usbd_controller
	)                            
target_sources(tinyusb_config PRIVATE
	${CMAKE_SOURCE_DIR}/tinyusb/src/portable/synopsys/dwc2/dcd_dwc2.c
   )

add_subdirectory(tinyusb/src)

target_link_libraries(${PROJECT_NAME}.elf
    tinyusb
    tinyusb_config
)

target_compile_definitions(freertos_config INTERFACE ${definitions})
target_compile_options(freertos_config INTERFACE ${options})

set(HEX_FILE ${PROJECT_BINARY_DIR}/${PROJECT_NAME}.hex)
set(BIN_FILE ${PROJECT_BINARY_DIR}/${PROJECT_NAME}.bin)

add_custom_command(TARGET ${PROJECT_NAME}.elf POST_BUILD
        COMMAND ${CMAKE_OBJCOPY} -Oihex $<TARGET_FILE:${PROJECT_NAME}.elf> ${HEX_FILE}
        COMMAND ${CMAKE_OBJCOPY} -Obinary $<TARGET_FILE:${PROJECT_NAME}.elf> ${BIN_FILE}
        COMMENT "Building ${HEX_FILE}
Building ${BIN_FILE}")
