#include "intro.hpp"
#include "play/game.hpp"

namespace {
    Game game;
};


bool Intro::init(Game* game)
{
    m_game = game;

    m_titleScreen = LoadTexture("assets/TitleScreen.png");

    EnableCursor();

    return true;
}


void Intro::update()
{
}


void Intro::render()
{
    Vector2 winSize = {
        (float)GetScreenWidth(),
        (float)GetScreenHeight()
    };

    float scaleX = winSize.x / m_titleScreen.width;
    float scaleY = winSize.y / m_titleScreen.height;
    float scale = fmaxf(scaleX, scaleY);

    // Calculate how big the texture will actually be after scaling.
    float width = m_titleScreen.width * scale;
    float height = m_titleScreen.height * scale;

    DrawTextureEx(
        m_titleScreen,
        {
            // Center the scaled image on the screen.
            (winSize.x - width) / 2.0f,
            (winSize.y - height) / 2.0f
        },
        0.0f,
        scale,
        WHITE
    );


    Rectangle playButton = {
        winSize.x * 0.45f,   // x position
        winSize.y * 0.75f,   // y position
        winSize.x * 0.1f,    // width
        winSize.y * 0.03f    // height
    };

    DrawRectangleRec(playButton, GREEN);

    Vector2 mouse = GetMousePosition();


    if (CheckCollisionPointRec(mouse, playButton))
    {
        if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT))
        {
            m_game->changeScene(Game::Scene::Play);
            
        }
    }
}


void Intro::shutdown()
{
    UnloadTexture(m_titleScreen);
}