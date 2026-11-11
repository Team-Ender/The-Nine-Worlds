#include "worlds/commons/library/library.hpp"

class Midgard
{
public:
    bool init();
    void update();
    void render();
    void shutdown();

private:
    // terrain
    // world geometry
    // buildings
    // NPCs
    // etc.

    Library library;
};