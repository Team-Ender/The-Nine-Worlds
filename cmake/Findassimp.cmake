set(assimp_FOUND TRUE)

if(TARGET assimp)
    get_target_property(ASSIMP_INCLUDE_DIRS assimp INTERFACE_INCLUDE_DIRECTORIES)

    if(NOT ASSIMP_INCLUDE_DIRS)
        set(ASSIMP_INCLUDE_DIRS "")
    endif()

    set(ASSIMP_LIBRARIES assimp)
else()
    set(assimp_FOUND FALSE)
endif()