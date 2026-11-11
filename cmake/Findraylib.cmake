set(raylib_FOUND TRUE)

if(TARGET raylib)
    get_target_property(RAYLIB_INCLUDE_DIRS raylib INTERFACE_INCLUDE_DIRECTORIES)

    if(NOT RAYLIB_INCLUDE_DIRS)
        set(RAYLIB_INCLUDE_DIRS "")
    endif()

    set(RAYLIB_LIBRARIES raylib)
else()
    set(raylib_FOUND FALSE)
endif()