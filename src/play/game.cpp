#include <math.h>
#include <r3d/r3d.h>


#include "game.hpp"
#include "controls.hpp" 
#include "intro/intro.hpp"
#include "play/play.hpp"

namespace {
    Controls controls;
    Intro intro;
    Play play;
}




bool Game::init()
{
    SetConfigFlags(FLAG_WINDOW_RESIZABLE);
    InitWindow(1920, 1080, "The Nine Worlds");
    SetWindowMinSize(1280, 720);
    SetWindowMaxSize(5120, 2880);
    Image icon = LoadImage("assets/icon2.png");
    ImageFormat(&icon, PIXELFORMAT_UNCOMPRESSED_R8G8B8A8);
    SetWindowIcon(icon);
    UnloadImage(icon);

    if (!intro.init(this)) {
        return false;
    }
    
    R3D_Init(1920, 1080);
    if (!play.init(this)) {
        R3D_Close();
        return false;
    }
    controls.init();
    return true;
}   

void Game::update() {
    if (currentScene == Scene::Intro) {
        intro.update();
    } 
    
    if (currentScene == Scene::Play) {
        controls.update();
        play.update();
    }   
}

void Game::render() {
        BeginDrawing();
        ClearBackground(RAYWHITE);

        if (currentScene == Scene::Intro) {
            intro.render();
        }
        if (currentScene == Scene::Play) {
            R3D_Begin(controls.camera);
            
            DisableCursor();
            play.render();
        
            R3D_End();           
        }
       
        EndDrawing();
}

void Game::run()
{
    while (!WindowShouldClose())
    {
        update();
        render();
    }
}

void Game::changeScene(Scene scene) {
    currentScene = scene;
}

void Game::shutdown()
{
    intro.shutdown();
    play.shutdown();
    R3D_Close();
    CloseWindow();
}   