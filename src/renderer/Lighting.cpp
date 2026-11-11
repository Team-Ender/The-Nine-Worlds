#include "Lighting.hpp"

void addOmniLight(
    Vector3 position,
    float range,
    Color color,
    float energy
)
{
    R3D_Light light = R3D_CreateOmniLight(
        position,
        range,
        color,
        energy
    );

    R3D_PushLight(light);
}