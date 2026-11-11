class Game;

class Play
{
public:
    void update();
    void render();
    void shutdown();

    bool init(Game* game);

private:
    Game* m_game = nullptr;

    // Play scene

    // all that shenanigans
};

