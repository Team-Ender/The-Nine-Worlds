#include "library.hpp"

#include <renderer/Lighting.hpp>

bool Library::init()
{
    room = R3D_LoadModel("assets/models/library.glb");

    if (room.meshCount <= 0)
    {
        TraceLog(LOG_ERROR, "Failed to load library model");
        return false;
    }

    return true;
}

void Library::update()
{
}

void Library::render(float x, float y, float z)
{
    Vector3 position = { x, y, z };
    float scale = 1.0f;

        
        addOmniLight(
            { -0.0f, 5.0f, 5.0f },
            20.0f,
            WHITE,
            5.0f
        );
        addOmniLight(
            { 20.0f, 5.0f, 5.0f },
            20.0f,
            WHITE,
            5.0f
        );  
        addOmniLight(
            { 40.0f, 5.0f, 5.0f },
            20.0f,
            WHITE,
            5.0f
        );  
        
    R3D_DrawModel(room, position, scale);
}

void Library::shutdown()
{
    R3D_UnloadModel(room, true);
}