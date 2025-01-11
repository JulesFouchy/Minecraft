cmake_minimum_required(VERSION 3.20)

project(Minecraft)
add_executable(${PROJECT_NAME})

target_compile_features(${PROJECT_NAME} PRIVATE cxx_std_23)

# Set warning level
if(MSVC)
    target_compile_options(${PROJECT_NAME} INTERFACE /W4)
else()
    target_compile_options(${PROJECT_NAME} INTERFACE -Wall -Wextra -Wpedantic -pedantic-errors -Wconversion -Wsign-conversion -Wimplicit-fallthrough)
endif()

# Maybe enable warnings as errors
set(WARNINGS_AS_ERRORS_FOR_MINECRAFT OFF CACHE BOOL "ON iff you want to treat warnings as errors")

if(WARNINGS_AS_ERRORS_FOR_MINECRAFT)
    if(MSVC)
        target_compile_options(${PROJECT_NAME} INTERFACE /WX)
    else()
        target_compile_options(${PROJECT_NAME} INTERFACE -Werror)
    endif()
endif()

# Set the folder where the executable is created
set_target_properties(${PROJECT_NAME} PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/bin/${CMAKE_BUILD_TYPE})

# Add all the source files
file(GLOB_RECURSE SOURCES CONFIGURE_DEPENDS src/*)
target_sources(${PROJECT_NAME} PRIVATE ${SOURCES})
target_include_directories(${PROJECT_NAME} PRIVATE src)