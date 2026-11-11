#pragma once

#include <r3d/r3d_lighting.h>


class Game
{
public:
    bool init();
    void run();
    void shutdown();
    void update();
    void render();

    enum class Scene {
        Intro,
        Play
    };
    Scene currentScene = Scene::Intro;

    void changeScene(Scene scene);

private:

    bool running = false;
    
};