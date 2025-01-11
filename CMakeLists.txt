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

target_compile_definitions(${PROJECT_NAME} PRIVATE
    $<$<CONFIG:Debug>:DEBUG=1> # Define DEBUG in debug mode
    IMGUI_DEFINE_MATH_OPERATORS
)

# -------------------
# ---Add libraries---
# -------------------

# GLFW (Must be before WebGPU, otherwise Dawn uses its own version of glfw)
if(NOT EMSCRIPTEN)
    add_subdirectory(lib/glfw)
    set(GLFW_BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)
    set(GLFW_BUILD_TESTS OFF CACHE BOOL "" FORCE)
    set(GLFW_BUILD_DOCS OFF CACHE BOOL "" FORCE)
    set(GLFW_INSTALL OFF CACHE BOOL "" FORCE)
    target_link_libraries(${PROJECT_NAME} PRIVATE glfw)
else()
    # Emscripten has built-in support for GLFW but requires the `-sUSE_GLFW=3` link option:
    add_library(glfw INTERFACE)
    target_link_options(glfw INTERFACE -sUSE_GLFW=3)
endif()

# WebGPU (Must be after GLFW, otherwise Dawn uses its own version of glfw)
set(WEBGPU_BACKEND "DAWN" CACHE STRING "" FORCE)
add_subdirectory(lib/WebGPU-distribution)
target_link_libraries(${PROJECT_NAME} PRIVATE webgpu)
target_copy_webgpu_binaries(${PROJECT_NAME})

add_subdirectory(lib/glfw3webgpu)
target_link_libraries(${PROJECT_NAME} PRIVATE glfw3webgpu)