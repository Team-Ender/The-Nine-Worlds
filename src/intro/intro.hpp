#include <raylib.h>

class Game;

class Intro
{
public:
    bool init(Game* game);
    void update();
    void render();
    void shutdown();

private:
    Texture2D m_titleScreen;
    Game* m_game = nullptr;
};