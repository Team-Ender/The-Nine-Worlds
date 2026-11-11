#include "play.hpp"
#include "game.hpp"

#include "worlds/WorldManager.hpp"

namespace {
    WorldMgr worldMgr;
}

bool Play::init(Game* game) {
    m_game = game;
    return worldMgr.initMgr();
}   

void Play::update() {
    worldMgr.updateMgr();
}

void Play::render() {
    worldMgr.renderMgr();
    

}

void Play::shutdown() {
    worldMgr.shutdownMgr();
}

