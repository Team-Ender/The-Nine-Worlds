#include <r3d/r3d.h>

class Library
{
public:
    bool init();
    void update();
    void render(float x, float y, float z);
    void shutdown();

private:
    R3D_Model room{};
};