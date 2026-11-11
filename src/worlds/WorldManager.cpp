#include "WorldManager.hpp"

#include "midgardr/midgardr.hpp"

namespace {
    Midgard midgard;
}

bool WorldMgr::initMgr() {

    // worlds init
    midgard.init();
    
    return true;
}

void WorldMgr::updateMgr() {

    midgard.update();

}

void WorldMgr::renderMgr() {
    midgard.render();

}

void WorldMgr::shutdownMgr() {
    midgard.shutdown();

}