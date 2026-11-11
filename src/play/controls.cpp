#include "controls.hpp"
#include "raymath.h"   // Required for: cosf(), sinf()

void Controls::init(){

    camera.position = { 0.0f, 2.0f, 0.0f };
    camera.target = { 0.0f, 0.0f, 0.0f };
    camera.up = { 0.0f, 1.0f, 0.0f };
    camera.fovy = 45.0f;
    camera.projection = CAMERA_PERSPECTIVE;
}

void Controls::update()
{
    float dt = GetFrameTime();

    // -------------------------
    // Camera rotation
    // -------------------------

    Vector2 mouseDelta = GetMouseDelta();

    yaw -= mouseDelta.x * 0.001f;
    pitch -= mouseDelta.y * 0.001f;

    pitch = Clamp(
        pitch,
        -PI / 2.0f + 0.01f,
        PI / 2.0f - 0.01f
    );

    Vector3 direction = {
        cosf(pitch) * sinf(yaw),
        sinf(pitch),
        cosf(pitch) * cosf(yaw)
    };

    Vector3 forward = {
    sinf(yaw),
    0.0f,
    cosf(yaw)
    };

    Vector3 right = Vector3CrossProduct(
        direction,
        camera.up
    );

    // -------------------------
    // Horizontal movement
    // -------------------------

    float speed = 5.0f;

    if (IsKeyDown(KEY_W))
    {
        camera.position = Vector3Add(
            camera.position,
            Vector3Scale(forward, speed * dt)
        );
    }

    if (IsKeyDown(KEY_S))
    {
        camera.position = Vector3Subtract(
            camera.position,
            Vector3Scale(forward, speed * dt)
        );
    }

    if (IsKeyDown(KEY_A))
    {
        camera.position = Vector3Subtract(
            camera.position,
            Vector3Scale(right, speed * dt)
        );
    }

    if (IsKeyDown(KEY_D))
    {
        camera.position = Vector3Add(
            camera.position,
            Vector3Scale(right, speed * dt)
        );
    }

    verticalVelocity -= gravity * dt;

    camera.position.y += verticalVelocity * dt;


    if (camera.position.y <= 0.0f)
    {
        camera.position.y = 0.0f;
        verticalVelocity = 0.0f;

        if (IsKeyDown(KEY_SPACE))
        {
            verticalVelocity = jumpStrength;
        }
    }

    // -------------------------
    // Camera target
    // -------------------------

    camera.target = Vector3Add(
        camera.position,
        direction
    );
}