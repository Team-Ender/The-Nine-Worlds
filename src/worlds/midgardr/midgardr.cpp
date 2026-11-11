#include "midgardr.hpp"

bool Midgard::init()
{
    if (!library.init()) {
        return false;
    }
    

    return true;
}   

void Midgard::update() {
    library.update();
}

void Midgard::render() {
    library.render(0.f, -5.f, 0.f);


    DrawPlane(
        { 0.0f, -10.0f, 0.0f },
        { 50.0f, 50.0f },
        GREEN   
        );
}

void Midgard::shutdown() {
    library.shutdown();
}

