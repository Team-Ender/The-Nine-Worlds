cmake_minimum_required(VERSION 3.20)

project(TheNineWorlds
    VERSION 0.1.0
    LANGUAGES CXX
)

list(APPEND CMAKE_MODULE_PATH
    "${CMAKE_SOURCE_DIR}/cmake"
)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# =========================================================
# Dependencies
# =========================================================

set(DEPS_DIR "${CMAKE_SOURCE_DIR}/deps")

function(clone_dependency NAME URL PATH)
    if(NOT EXISTS "${PATH}")
        message(STATUS "${NAME} not found. Cloning...")

        execute_process(
            COMMAND git clone --recurse-submodules "${URL}" "${PATH}"
            RESULT_VARIABLE CLONE_RESULT
        )

        if(NOT CLONE_RESULT EQUAL 0)
            message(FATAL_ERROR "Failed to clone ${NAME}.")
        endif()
    endif()
endfunction()

# =========================================================
# raylib
# =========================================================

set(RAYLIB_DIR "${DEPS_DIR}/raylib")

clone_dependency(
    "raylib"
    "https://github.com/raysan5/raylib.git"
    "${RAYLIB_DIR}"
)

set(BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)

add_subdirectory(
    "${RAYLIB_DIR}"
    "${CMAKE_BINARY_DIR}/raylib"
)

# =========================================================
# GLM
# =========================================================

set(GLM_DIR "${DEPS_DIR}/glm")

clone_dependency(
    "GLM"
    "https://github.com/g-truc/glm.git"
    "${GLM_DIR}"
)

add_library(glm INTERFACE)

target_include_directories(glm INTERFACE
    "${GLM_DIR}"
)

# =========================================================
# Jolt Physics
# =========================================================

set(JOLT_DIR "${DEPS_DIR}/JoltPhysics")

clone_dependency(
    "Jolt Physics"
    "https://github.com/jrouwe/JoltPhysics.git"
    "${JOLT_DIR}"
)

set(DOUBLE_PRECISION OFF CACHE BOOL "" FORCE)
set(GENERATE_DEBUG_SYMBOLS ON CACHE BOOL "" FORCE)
set(OVERRIDE_CXX_FLAGS OFF CACHE BOOL "" FORCE)
set(INTERPROCEDURAL_OPTIMIZATION OFF CACHE BOOL "" FORCE)
set(USE_STATIC_MSVC_RUNTIME_LIBRARY OFF CACHE BOOL "" FORCE)

add_subdirectory(
    "${JOLT_DIR}/Build"
    "${CMAKE_BINARY_DIR}/Jolt"
)

# =========================================================
# Assimp
# =========================================================

clone_dependency(
    assimp
    https://github.com/assimp/assimp.git
    "${DEPS_DIR}/assimp"
)
set(ASSIMP_BUILD_TESTS OFF CACHE BOOL "" FORCE)
set(ASSIMP_BUILD_ASSIMP_TOOLS OFF CACHE BOOL "" FORCE)
set(ASSIMP_BUILD_SAMPLES OFF CACHE BOOL "" FORCE)

set(ASSIMP_WARNINGS_AS_ERRORS OFF CACHE BOOL "" FORCE)

add_subdirectory(
    "${DEPS_DIR}/assimp"
    "${CMAKE_BINARY_DIR}/assimp"
)

if(CMAKE_CXX_COMPILER_ID STREQUAL "Clang" AND CMAKE_RC_COMPILER MATCHES "llvm-rc")
    get_target_property(ASSIMP_SOURCES assimp SOURCES)
    list(FILTER ASSIMP_SOURCES EXCLUDE REGEX "assimp\\.rc$")
    set_property(TARGET assimp PROPERTY SOURCES "${ASSIMP_SOURCES}")
endif()


# ========================================================= 
# R3D   
# ========================================================

clone_dependency(
    r3d
    https://github.com/Bigfoot71/r3d.git
    "${DEPS_DIR}/r3d"
)

set(R3D_WARNINGS_AS_ERRORS OFF CACHE BOOL "" FORCE)

add_subdirectory(
    "${DEPS_DIR}/r3d"
    "${CMAKE_BINARY_DIR}/r3d"
)




# =========================================================
# Game Sources
# =========================================================

file(GLOB_RECURSE SOURCES CONFIGURE_DEPENDS
    "${CMAKE_SOURCE_DIR}/src/*.cpp"
)

list(FILTER SOURCES EXCLUDE REGEX
    "${DEPS_DIR}/.*"
)



# =========================================================
# Game
# =========================================================

add_executable(TheNineWorlds
    ${SOURCES}
)

set_target_properties(TheNineWorlds PROPERTIES
    WIN32_EXECUTABLE FALSE
)

target_include_directories(TheNineWorlds PRIVATE
    "${CMAKE_SOURCE_DIR}/src"
)

target_include_directories(TheNineWorlds PRIVATE
    "${CMAKE_SOURCE_DIR}/worlds"
)

target_include_directories(TheNineWorlds PRIVATE
    "${DEPS_DIR}/r3d/include"
)

target_link_libraries(TheNineWorlds PRIVATE
    raylib
    glm
    Jolt
    assimp
    r3d
)



# =========================================================
# Assets
# =========================================================

add_custom_command(TARGET TheNineWorlds POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_directory
        "${CMAKE_SOURCE_DIR}/assets"
        "$<TARGET_FILE_DIR:TheNineWorlds>/assets"
)

# =========================================================
# Runtime dependencies
# =========================================================

if(WIN32)
    add_custom_command(TARGET TheNineWorlds POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different
            "$<TARGET_FILE:raylib>"
            "$<TARGET_FILE_DIR:TheNineWorlds>"
    )
endif()