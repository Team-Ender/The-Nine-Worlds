# The Nine Worlds --- Complete Game TODO

> **Project:** The Nine Worlds\
> **Genre:** Large-scale 3D solo/indie action-adventure / exploration
> RPG\
> **Setting:** Norse mythology-inspired\
> **Primary language:** C++\
> **Graphics/game library:** raylib\
> **Build system:** CMake\
> **Target:** Windows initially
>
> This is a living development roadmap, not a promise that every item
> must be implemented exactly as written. Early prototypes should become
> real parts of the final game whenever practical.

# 1. Project Foundation

## 1.1 Repository

-   [x] Create project repository.
-   [x] Establish source directory.
-   [x] Establish dependency directory.
-   [x] Establish build directory.
-   [x] Establish CMake project.
-   [x] Configure raylib dependency.
-   [x] Automatically use a local raylib checkout when available.
-   [x] Fetch/clone raylib when it is missing.
-   [x] Glob source files under `src/` rather than manually listing
    every `.cpp`.
-   [x] Configure C++ compiler.
-   [x] Configure clang/LLVM toolchain.
-   [x] Configure Debug build.
-   [x] Configure Release build.
-   [x] Make Debug retain the console for logs.
-   [x] Make Release use the Windows GUI subsystem.
-   [x] Add `.gitignore`.
-   [x] Add README.
-   [x] Add license. (GPLV3)
-   [x] Add CI build.

## 1.2 Initial directory architecture

Possible long-term structure:
The Nine Worlds/
├── CMakeLists.txt
├── README.md
├── LICENSE
├── todo.md
├── assets/
│   ├── textures/
│   ├── models/
│   ├── audio/
│   ├── fonts/
│   ├── shaders/
│   └── data/
├── dependencies/
├── docs/
├── saves/
├── src/
│   ├── main.cpp
│   ├── game.cpp
│   ├── game.hpp
│   ├── core/
│   ├── graphics/
│   ├── input/
│   ├── world/
│   ├── entities/
│   ├── player/
│   ├── combat/
│   ├── interaction/
│   ├── inventory/
│   ├── runes/
│   ├── quests/
│   ├── dialogue/
│   ├── factions/
│   ├── ai/
│   ├── audio/
│   ├── save/
│   ├── streaming/
│   ├── networking/
│   └── ui/
└── tools/
```

-   [ ] Create only directories when they become useful.
-   [ ] Avoid creating hundreds of empty files before the systems exist.

------------------------------------------------------------------------

# 2. Game Loop

-   [x] Create `Game`.
-   [x] Create constructor.
-   [x] Create destructor.
-   [x] Initialize raylib.
-   [x] Create window.
-   [x] Set target FPS.
-   [x] Create `run()`.
-   [x] Create `update()`.
-   [x] Create `draw()`.
-   [x] Handle `WindowShouldClose()`.
-   [ ] Separate fixed-step simulation from rendering if needed.
-   [ ] Track `deltaTime`.
-   [ ] Add frame timing.
-   [ ] Add FPS display in debug mode.
-   [ ] Add frame-time graph/debug overlay.
-   [ ] Add global pause state.
-   [ ] Add game states.
-   [ ] Add loading state.
-   [ ] Add main menu state.
-   [ ] Add gameplay state.
-   [ ] Add pause state.
-   [ ] Add save/load state.
-   [ ] Add transition state.

------------------------------------------------------------------------

# 3. 3D Rendering Basics

-   [x] Initialize 3D camera.
-   [x] Enter `BeginMode3D()`.
-   [x] Draw a 3D object.
-   [x] Exit `EndMode3D()`.
-   [x] Understand `position`, `target`, `up`, `fovy`, and projection.
-   [x] Understand world coordinates.
-   [x] Understand X/Y/Z.
-   [x] Understand camera orientation.
-   [x] Understand perspective projection.
-   [ ] Understand model transforms.
-   [ ] Understand local vs world coordinates.
-   [ ] Understand matrices.
-   [ ] Understand vectors.
-   [ ] Understand quaternions well enough to use them when appropriate.
-   [ ] Add basic lighting.
-   [ ] Add shadows.
-   [ ] Add materials.
-   [ ] Add textures.
-   [ ] Add sky/background.
-   [ ] Add fog.
-   [ ] Add atmospheric effects.
-   [ ] Add water rendering.
-   [ ] Add particles.
-   [ ] Add post-processing where useful.

------------------------------------------------------------------------

# 4. Camera

## 4.1 Free camera

-   [x] Disable cursor.
-   [x] Read mouse delta.
-   [x] Convert mouse movement into yaw/pitch.
-   [x] Prevent pitch from flipping over.
-   [x] Calculate forward direction.
-   [x] Update camera target.
-   [ ] Move camera using keyboard.
-   [ ] Add configurable movement speed.
-   [ ] Add acceleration/deceleration.
-   [ ] Add sprint.
-   [ ] Add vertical movement.
-   [ ] Add fast debug movement.
-   [ ] Add slow precision movement.
-   [ ] Add camera sensitivity setting.
-   [ ] Add optional inverted Y.
-   [ ] Add optional inverted X.
-   [ ] Add cursor lock/unlock.

## 4.2 Final player camera

-   [ ] Decide first-person vs third-person.
-   [ ] Implement player-relative camera.
-   [ ] Add camera collision.
-   [ ] Add camera smoothing.
-   [ ] Add camera shake.
-   [ ] Add camera effects.
-   [ ] Add underwater camera behavior.
-   [ ] Add extreme-temperature visual effects.
-   [ ] Add realm-specific atmosphere.
-   [ ] Add cinematic camera controller.

------------------------------------------------------------------------

# 5. Tiny Miðgarðr Prototype

> **First major gameplay milestone: make a tiny piece of Miðgarðr that
> can actually be walked around.**

-   [ ] Create terrain.
-   [ ] Create ground.
-   [ ] Add basic height variation.
-   [ ] Add trees/rocks.
-   [ ] Add a small path.
-   [ ] Add one small settlement.
-   [ ] Add a few buildings.
-   [ ] Add collision.
-   [ ] Add player spawn.
-   [ ] Add player movement.
-   [ ] Add basic interaction.
-   [ ] Add one NPC.
-   [ ] Add one interactable object.
-   [ ] Add one door.
-   [ ] Add one altar.
-   [ ] Add one rune interaction.
-   [ ] Add basic save/load.
-   [ ] Make this prototype part of the actual Miðgarðr map.

------------------------------------------------------------------------

# 6. Player

-   [ ] Player entity.
-   [ ] Transform.
-   [ ] Velocity.
-   [ ] Acceleration.
-   [ ] Movement.
-   [ ] Gravity.
-   [ ] Jumping.
-   [ ] Ground detection.
-   [ ] Slopes.
-   [ ] Step handling.
-   [ ] Collision.
-   [ ] Swimming.
-   [ ] Climbing where appropriate.
-   [ ] Falling.
-   [ ] Environmental hazards.
-   [ ] Stamina if the final design needs it.
-   [ ] Health.
-   [ ] Status effects.
-   [ ] Death state.
-   [ ] Respawn.
-   [ ] Animation state.
-   [ ] Equipment state.
-   [ ] Inventory state.
-   [ ] Knowledge state.
-   [ ] Quest state.
-   [ ] Faction relationships.
-   [ ] Origin information.

------------------------------------------------------------------------

# 7. Interaction System

Create a general system rather than hardcoding every interaction.

-   [ ] Raycast/interaction detection.
-   [ ] Interaction range.
-   [ ] Interactable interface/component.
-   [ ] Interaction prompt.
-   [ ] Context-sensitive actions.
-   [ ] Doors.
-   [ ] Chests.
-   [ ] NPCs.
-   [ ] Altars.
-   [ ] Books.
-   [ ] Rune inscriptions.
-   [ ] Levers.
-   [ ] Environmental objects.
-   [ ] Hidden objects.
-   [ ] World transitions.
-   [ ] Dialogue interaction.
-   [ ] Pickups.
-   [ ] Examine interactions.
-   [ ] Multi-stage interactions.
-   [ ] Interaction requirements.
-   [ ] Knowledge-based interactions.

------------------------------------------------------------------------

# 8. World Coordinate Architecture

-   [ ] Establish a consistent world coordinate convention.
-   [ ] Decide units.
-   [ ] Decide player height/unit scale.
-   [ ] Decide chunk size.
-   [ ] Decide sector size.
-   [ ] Decide realm coordinate ranges.
-   [ ] Decide whether each realm has its own coordinate origin.
-   [ ] Implement world-space transforms.
-   [ ] Implement realm-space transforms.
-   [ ] Implement chunk coordinates.
-   [ ] Implement deterministic world coordinates.
-   [ ] Add coordinate debugging tools.

------------------------------------------------------------------------

# 9. World Representation

-   [ ] World.
-   [ ] Realm.
-   [ ] Region.
-   [ ] Zone.
-   [ ] Chunk.
-   [ ] Cell/sector.
-   [ ] Terrain.
-   [ ] Static objects.
-   [ ] Dynamic entities.
-   [ ] Spawn points.
-   [ ] Points of interest.
-   [ ] Routes.
-   [ ] Doors.
-   [ ] Altars.
-   [ ] Hidden locations.
-   [ ] Quest locations.
-   [ ] Dungeon locations.

Possible conceptual hierarchy:

``` text
World
└── Realm
    └── Region
        └── Zone
            └── Chunk
                ├── Terrain
                ├── Static objects
                ├── Entities
                ├── POIs
                └── Spawn data
```

------------------------------------------------------------------------

# 10. The Nine Worlds

The game interpretation uses:

1.  [ ] Ásgarðr --- Æsir
2.  [ ] Miðgarðr --- humans
3.  [ ] Vanaheimr --- Vanir
4.  [ ] Jötunheimr --- jötnar
5.  [ ] Álfheimr --- álfr/elves
6.  [ ] Svartálfaheimr / Niðavellir --- dwarfs
7.  [ ] Niflheimr --- cold/primordial realm
8.  [ ] Múspellheimr --- fire realm
9.  [ ] Hel --- realm associated with the dead

> Historical note: surviving medieval sources do not provide one
> universally agreed numbered map of "the Nine Worlds". The game's
> geography is therefore an intentional interpretation.

For every realm:

-   [ ] Establish visual identity.
-   [ ] Establish geography.
-   [ ] Establish climate/environment.
-   [ ] Establish inhabitants.
-   [ ] Establish factions.
-   [ ] Establish settlements.
-   [ ] Establish hazards.
-   [ ] Establish wildlife/creatures.
-   [ ] Establish major landmarks.
-   [ ] Establish traversal rules.
-   [ ] Establish realm-specific mechanics.
-   [ ] Establish music/audio identity.
-   [ ] Establish quests.
-   [ ] Establish secrets.
-   [ ] Establish relationship to Yggdrasil.
-   [ ] Establish relationship to other realms.
-   [ ] Establish lore.
-   [ ] Establish streaming boundaries.

------------------------------------------------------------------------

# 11. Miðgarðr

-   [ ] Main human realm.
-   [ ] Mountains.
-   [ ] Forests.
-   [ ] Rivers.
-   [ ] Lakes.
-   [ ] Coastlines.
-   [ ] Villages.
-   [ ] Larger settlements.
-   [ ] Farms.
-   [ ] Roads.
-   [ ] Wilderness.
-   [ ] Ruins.
-   [ ] Sacred sites.
-   [ ] Hidden caves.
-   [ ] Human factions.
-   [ ] Human cultures.
-   [ ] Traders.
-   [ ] Warriors.
-   [ ] Scholars.
-   [ ] Religious/lore institutions.
-   [ ] Realm-specific quests.
-   [ ] Major cities.
-   [ ] Regional economies.
-   [ ] Wildlife.
-   [ ] Monsters.
-   [ ] Weather.
-   [ ] Day/night cycle.

------------------------------------------------------------------------

# 12. Ásgarðr

-   [ ] Design realm geography.
-   [ ] Establish access requirements.
-   [ ] Build major divine settlements.
-   [ ] Build divine sanctuaries.
-   [ ] Add gods as NPCs/entities.
-   [ ] Add divine factions.
-   [ ] Add realm-specific rules.
-   [ ] Build the route toward Yggdrasil.
-   [ ] Implement first Yggdrasil entry.
-   [ ] Add divine doors.
-   [ ] Add divine altars.
-   [ ] Add lore/library content.
-   [ ] Add major landmarks.

------------------------------------------------------------------------

# 13. Vanaheimr

-   [ ] Define geography.
-   [ ] Define Vanir culture.
-   [ ] Define settlements.
-   [ ] Define magic/nature systems.
-   [ ] Define political relationships.
-   [ ] Define NPCs.
-   [ ] Define factions.
-   [ ] Define quests.
-   [ ] Define major locations.
-   [ ] Connect to wider realm network.

------------------------------------------------------------------------

# 14. Jötunheimr

-   [ ] Build enormous-scale environmental design.
-   [ ] Create jötunn settlements.
-   [ ] Implement giant-scale architecture.
-   [ ] Implement scale differences where useful.
-   [ ] Add jötunn factions.
-   [ ] Add wilderness.
-   [ ] Add major landmarks.
-   [ ] Add mythology-inspired encounters.
-   [ ] Add unique traversal.
-   [ ] Add quests and politics.

------------------------------------------------------------------------

# 15. Álfheimr

-   [ ] Define álfr society.
-   [ ] Build settlements.
-   [ ] Build landscapes.
-   [ ] Add unique visual identity.
-   [ ] Add faction relationships.
-   [ ] Add lore.
-   [ ] Add quests.
-   [ ] Add unique environmental mechanics.

------------------------------------------------------------------------

# 16. Svartálfaheimr / Niðavellir

-   [ ] Define dwarf culture.
-   [ ] Build underground areas.
-   [ ] Build mines.
-   [ ] Build workshops.
-   [ ] Build enormous mechanical/industrial spaces where appropriate.
-   [ ] Add crafting-related systems.
-   [ ] Add dwarf factions.
-   [ ] Add unique materials.
-   [ ] Add lore.
-   [ ] Add quests.
-   [ ] Add hidden tunnels.

------------------------------------------------------------------------

# 17. Niflheimr

-   [ ] Build cold/primordial environment.
-   [ ] Implement extreme visibility conditions.
-   [ ] Add ice.
-   [ ] Add fog.
-   [ ] Add dangerous terrain.
-   [ ] Add environmental survival mechanics if appropriate.
-   [ ] Build central Niflheim route.
-   [ ] Create hidden route toward Yggdrasil.
-   [ ] Make reaching the center a meaningful exploration challenge.
-   [ ] Add unique inhabitants.
-   [ ] Add lore.
-   [ ] Add secrets.

------------------------------------------------------------------------

# 18. Múspellheimr

> Intentional exception to the usual giant explorable-world rule.

-   [ ] Create extreme fire environment.
-   [ ] Create the main accessible chamber/sanctuary.
-   [ ] Make the surrounding environment effectively
    inaccessible/dangerous.
-   [ ] Build Múspell-origin content.
-   [ ] Build fire-related mechanics.
-   [ ] Build Surtr-related content if appropriate.
-   [ ] Build Ragnarök-related progression.
-   [ ] Make the realm feel enormous despite limited safe exploration.
-   [ ] Add unique audiovisual presentation.

------------------------------------------------------------------------

# 19. Hel

-   [ ] Define realm geography.
-   [ ] Define relationship to the dead.
-   [ ] Build major landmarks.
-   [ ] Define inhabitants.
-   [ ] Define traversal.
-   [ ] Define death/return mechanics.
-   [ ] Define quests.
-   [ ] Define lore.
-   [ ] Define relationship to Miðgarðr.
-   [ ] Define relationship to Ragnarök.

------------------------------------------------------------------------

# 20. Yggdrasil

> **Yggdrasil is an actual physical explorable world-space, not a
> loading screen.**

-   [ ] Create enormous Yggdrasil environment.
-   [ ] Establish trunk.
-   [ ] Establish roots.
-   [ ] Establish branches.
-   [ ] Establish vertical traversal.
-   [ ] Establish realm connections.
-   [ ] Add environmental hazards.
-   [ ] Add unique flora/fauna.
-   [ ] Add major landmarks.
-   [ ] Add divine access points.
-   [ ] Add altars.
-   [ ] Add hidden paths.
-   [ ] Add secrets.
-   [ ] Add Ratatoskr.
-   [ ] Add Níðhöggr.
-   [ ] Add great eagle.
-   [ ] Add Veðrfölnir.
-   [ ] Decide how other source-attested creatures/entities appear.

------------------------------------------------------------------------

# 21. Yggdrasil Access

## Hidden Niflheim route

-   [ ] Allow player to reach central Niflheimr.
-   [ ] Discover hidden route.
-   [ ] Make discovery difficult.
-   [ ] Make it possible without the intended progression if desired.

## Intended route

Requirements:

-   [ ] Gain access to Ásgarðr.
-   [ ] Obtain special cape.
-   [ ] Implement Ratatoskr stealth mechanic.
-   [ ] Obtain/use Dagaz.
-   [ ] Reach Yggdrasil.
-   [ ] Establish first successful entry.

## Post-entry access

-   [ ] Unlock Yggdrasil altars.
-   [ ] Allow later access through altars.
-   [ ] Persist access state in save data.

------------------------------------------------------------------------

# 22. Ratatoskr

-   [ ] Create Ratatoskr entity.
-   [ ] Make Ratatoskr a serious threat.
-   [ ] Create detection system.
-   [ ] Create player visibility/concealment state.
-   [ ] Create stealth behavior.
-   [ ] Create patrol/navigation behavior.
-   [ ] Create pursuit behavior.
-   [ ] Create escape behavior.
-   [ ] Create encounter feedback.
-   [ ] Create cape interaction.
-   [ ] Make Ratatoskr behavior feel distinct from a normal combat
    enemy.
-   [ ] Add appropriate sound cues.
-   [ ] Add environmental warning signs.
-   [ ] Balance the encounter around hiding rather than simply killing
    the creature.

------------------------------------------------------------------------

# 23. Cape

-   [ ] Create cape item/equipment.
-   [ ] Create concealment effect.
-   [ ] Integrate with Ratatoskr detection.
-   [ ] Define limitations.
-   [ ] Define visual/audio feedback.
-   [ ] Add lore.
-   [ ] Make acquisition meaningful.

------------------------------------------------------------------------

# 24. Dagaz

> Dagaz is a major traversal/world-network rune.

-   [ ] Implement Dagaz rune.
-   [ ] Define activation conditions.
-   [ ] Require Yggdrasil or a god's sanctuary where appropriate.
-   [ ] Create threshold/door network.
-   [ ] Create destination registration.
-   [ ] Create destination discovery.
-   [ ] Create door-to-door traversal.
-   [ ] Create altar access.
-   [ ] Persist discovered destinations.
-   [ ] Handle inaccessible destinations.
-   [ ] Handle locked destinations.
-   [ ] Handle world-state-dependent destinations.
-   [ ] Handle Ragnarök-changing destinations.
-   [ ] Make doors meaningful destinations rather than generic loading
    screens.
-   [ ] Add UI for discovered destinations.
-   [ ] Add lore explanation.
-   [ ] Add visual/audio activation.

------------------------------------------------------------------------

# 25. Runes

Use the 24 Elder Futhark runes as the starting set.

-   [ ] Fehu ᚠ
-   [ ] Uruz ᚢ
-   [ ] Thurisaz ᚦ
-   [ ] Ansuz ᚨ
-   [ ] Raidō ᚱ
-   [ ] Kaunan/Kenaz ᚲ
-   [ ] Gebo ᚷ
-   [ ] Wunjō ᚹ
-   [ ] Hagalaz ᚺ
-   [ ] Nauthiz ᚾ
-   [ ] Isaz ᛁ
-   [ ] Jēra ᛃ
-   [ ] Eihwaz ᛇ
-   [ ] Perthro ᛈ
-   [ ] Algiz ᛉ
-   [ ] Sowilō ᛊ
-   [ ] Tiwaz ᛏ
-   [ ] Berkanan ᛒ
-   [ ] Ehwaz ᛖ
-   [ ] Mannaz ᛗ
-   [ ] Laguz ᛚ
-   [ ] Ingwaz ᛜ
-   [ ] Dagaz ᛞ
-   [ ] Othala ᛟ

## Rune architecture

-   [ ] Create rune ID/type.
-   [ ] Create rune metadata.
-   [ ] Create display name.
-   [ ] Create rune symbol.
-   [ ] Create lore description.
-   [ ] Create unlock/discovery state.
-   [ ] Create effect definitions.
-   [ ] Create activation rules.
-   [ ] Create cooldown/cost rules where appropriate.
-   [ ] Create rune combinations.
-   [ ] Create rune knowledge.
-   [ ] Create rune progression.
-   [ ] Create save serialization.
-   [ ] Avoid creating huge duplicated implementation files for every
    rune.
-   [ ] Use shared systems for common behavior.
-   [ ] Keep individual rune data declarative where possible.
-   [ ] Allow unique rune code only when genuinely necessary.

------------------------------------------------------------------------

# 26. Rune Discovery / Knowledge

-   [ ] Discover rune inscriptions.
-   [ ] Learn rune names.
-   [ ] Learn rune pronunciation/display where appropriate.
-   [ ] Learn lore.
-   [ ] Discover mechanical properties.
-   [ ] Discover combinations.
-   [ ] Store knowledge in player state.
-   [ ] Allow NPCs/books to teach rune knowledge.
-   [ ] Allow experimentation.
-   [ ] Add unknown-rune UI.
-   [ ] Add known-rune UI.
-   [ ] Add lore library integration.
-   [ ] Avoid assuming historical magical meanings are canonical.
-   [ ] Clearly frame invented mechanics as game fiction.

------------------------------------------------------------------------

# 27. Origins / Peoples

Potential playable origins:

-   [ ] Human
-   [ ] Álfr
-   [ ] Dvergr
-   [ ] Jötunn
-   [ ] Æsir
-   [ ] Vanir
-   [ ] Múspell being

Origins should not simply be "race = class".

For every origin:

-   [ ] Starting location.
-   [ ] Culture.
-   [ ] Knowledge.
-   [ ] Social reactions.
-   [ ] Relationships.
-   [ ] Unique abilities.
-   [ ] Starting equipment.
-   [ ] Starting lore.
-   [ ] Faction relationships.
-   [ ] Unique dialogue.
-   [ ] Unique quests.
-   [ ] Unique access routes.
-   [ ] Avoid completely locking the player's playstyle.

------------------------------------------------------------------------

# 28. Character Creation

-   [ ] Choose origin.
-   [ ] Appearance system.
-   [ ] Name.
-   [ ] Pronouns if desired.
-   [ ] Starting background if desired.
-   [ ] Starting knowledge.
-   [ ] Starting location.
-   [ ] Starting relationships.
-   [ ] Starting equipment.
-   [ ] Save character state.
-   [ ] Ensure origin affects the world without reducing the player to a
    class.

------------------------------------------------------------------------

# 29. NPC System

-   [ ] NPC entity.
-   [ ] Identity.
-   [ ] Origin/people.
-   [ ] Faction.
-   [ ] Location.
-   [ ] Schedule.
-   [ ] Needs.
-   [ ] Relationships.
-   [ ] Knowledge.
-   [ ] Memory.
-   [ ] Dialogue state.
-   [ ] Quest state.
-   [ ] Inventory.
-   [ ] Occupation.
-   [ ] Combat capability.
-   [ ] Travel behavior.
-   [ ] Death/injury behavior if appropriate.
-   [ ] World-state reactions.
-   [ ] Persistent NPC state.
-   [ ] Distance-based simulation.

------------------------------------------------------------------------

# 30. AI / Simulation

## High detail

Nearby NPCs:

-   [ ] Full navigation.
-   [ ] Full schedules.
-   [ ] Full perception.
-   [ ] Full combat.
-   [ ] Full dialogue state.
-   [ ] Full interactions.

## Medium detail

Distant NPCs:

-   [ ] Simplified movement.
-   [ ] Simplified schedules.
-   [ ] Simplified interactions.
-   [ ] Reduced update rate.

## Low detail

Very distant NPCs:

-   [ ] Abstract simulation.
-   [ ] Population changes.
-   [ ] Resource changes.
-   [ ] Faction changes.
-   [ ] Major event processing.

## Far/unloaded

-   [ ] Save persistent state.
-   [ ] Simulate only world-level consequences.
-   [ ] Spawn/reconstruct entities when necessary.

------------------------------------------------------------------------

# 31. Factions

-   [ ] Faction data.
-   [ ] Reputation.
-   [ ] Alliances.
-   [ ] Hostility.
-   [ ] Trade relationships.
-   [ ] Political relationships.
-   [ ] Territory.
-   [ ] Faction leaders.
-   [ ] Internal factions.
-   [ ] Dynamic relationships.
-   [ ] World-state changes.
-   [ ] Ragnarök reactions.

------------------------------------------------------------------------

# 32. Dialogue

-   [ ] Dialogue data format.
-   [ ] Dialogue UI.
-   [ ] Branching dialogue.
-   [ ] Player choices.
-   [ ] Knowledge requirements.
-   [ ] Origin requirements.
-   [ ] Faction requirements.
-   [ ] Quest requirements.
-   [ ] World-state requirements.
-   [ ] Relationship requirements.
-   [ ] Consequences.
-   [ ] Persistent dialogue state.
-   [ ] NPC memory.
-   [ ] Lore references.

------------------------------------------------------------------------

# 33. Quest System

-   [ ] Quest IDs.
-   [ ] Objectives.
-   [ ] Conditions.
-   [ ] Rewards.
-   [ ] Dialogue integration.
-   [ ] World-state effects.
-   [ ] Optional objectives.
-   [ ] Hidden objectives.
-   [ ] Failure conditions.
-   [ ] Quest chains.
-   [ ] Branching quests.
-   [ ] Faction quests.
-   [ ] Origin-specific quests.
-   [ ] Rune quests.
-   [ ] Yggdrasil quests.
-   [ ] Ragnarök quests.
-   [ ] Endgame quests.
-   [ ] Quest journal.
-   [ ] Quest persistence.

------------------------------------------------------------------------

# 34. Inventory

-   [ ] Item IDs.
-   [ ] Item definitions.
-   [ ] Stackable items.
-   [ ] Equipment.
-   [ ] Consumables.
-   [ ] Quest items.
-   [ ] Rune items.
-   [ ] Lore items.
-   [ ] Materials.
-   [ ] Currency.
-   [ ] Inventory UI.
-   [ ] Item descriptions.
-   [ ] Weight/capacity only if useful.
-   [ ] Save/load.

------------------------------------------------------------------------

# 35. Equipment

-   [ ] Weapons.
-   [ ] Armor.
-   [ ] Clothing.
-   [ ] Accessories.
-   [ ] Rune equipment.
-   [ ] Cape.
-   [ ] Tools.
-   [ ] Equipment stats.
-   [ ] Visual representation.
-   [ ] Equipment effects.
-   [ ] Durability only if desired.
-   [ ] Equipment requirements.
-   [ ] Origin interactions.

------------------------------------------------------------------------

# 36. Combat

-   [ ] Decide combat philosophy.
-   [ ] Basic attack.
-   [ ] Heavy attack.
-   [ ] Defensive action.
-   [ ] Dodge/block/parry as appropriate.
-   [ ] Hit detection.
-   [ ] Damage.
-   [ ] Health.
-   [ ] Stagger.
-   [ ] Status effects.
-   [ ] Enemy behavior.
-   [ ] Boss behavior.
-   [ ] Rune integration.
-   [ ] Weapon types.
-   [ ] Environmental interactions.
-   [ ] Death.
-   [ ] Respawn.
-   [ ] Difficulty balancing.
-   [ ] Combat feedback.
-   [ ] Audio feedback.
-   [ ] VFX.

------------------------------------------------------------------------

# 37. Creatures / Enemies

-   [ ] Wildlife.
-   [ ] Mundane hostile creatures.
-   [ ] Mythological creatures.
-   [ ] Realm-specific creatures.
-   [ ] Elite enemies.
-   [ ] Bosses.
-   [ ] Unique named entities.
-   [ ] AI archetypes.
-   [ ] Spawn rules.
-   [ ] Loot.
-   [ ] Faction relationships.
-   [ ] Environmental behavior.

------------------------------------------------------------------------

# 38. Gods

Build a substantial roster of important gods and a smaller roster of
minor divine beings.

-   [ ] Odin
-   [ ] Frigg
-   [ ] Thor
-   [ ] Baldr
-   [ ] Höðr
-   [ ] Váli
-   [ ] Víðarr
-   [ ] Heimdallr
-   [ ] Týr
-   [ ] Bragi
-   [ ] Iðunn
-   [ ] Sif
-   [ ] Ullr
-   [ ] Forseti
-   [ ] Njörðr
-   [ ] Freyr
-   [ ] Freyja
-   [ ] Skaði
-   [ ] Loki
-   [ ] Ægir
-   [ ] Rán
-   [ ] Gefjon
-   [ ] Fulla
-   [ ] Sjöfn
-   [ ] Lofn
-   [ ] Vár
-   [ ] Vör
-   [ ] Syn
-   [ ] Hlín
-   [ ] Snotra
-   [ ] Gná
-   [ ] Eir
-   [ ] Nanna
-   [ ] Sigyn
-   [ ] Hel
-   [ ] Surtr
-   [ ] Mímir
-   [ ] Kvasir
-   [ ] Rindr
-   [ ] Sága
-   [ ] Óðr
-   [ ] Ullr
-   [ ] Andvari
-   [ ] Dellingr
-   [ ] Máni
-   [ ] Sól
-   [ ] Nótt
-   [ ] Dagr
-   [ ] Mundilfari
-   [ ] Borr
-   [ ] Bestla
-   [ ] Ymir
-   [ ] Auðumbla
-   [ ] Ask
-   [ ] Embla

> Verify names, relationships, and classifications against
> primary/academic sources during lore implementation. Do not treat
> every later popular-culture classification as historical fact.

For each major god:

-   [ ] Character profile.
-   [ ] Appearance.
-   [ ] Personality.
-   [ ] Domain/associations.
-   [ ] Relationships.
-   [ ] Faction.
-   [ ] Sanctuary.
-   [ ] Dialogue.
-   [ ] Quests.
-   [ ] World-state reactions.
-   [ ] Ragnarök behavior.
-   [ ] Lore entries.
-   [ ] Voice/audio identity if voice is eventually used.

------------------------------------------------------------------------

# 39. Divine Relationships

-   [ ] Æsir relationships.
-   [ ] Vanir relationships.
-   [ ] Æsir--Vanir history.
-   [ ] God/family relationships.
-   [ ] God/jötunn relationships.
-   [ ] Loki's relationships.
-   [ ] Human/divine relationships.
-   [ ] Dynamic relationships.
-   [ ] Relationship consequences.
-   [ ] World-state consequences.

------------------------------------------------------------------------

# 40. Lore Library --- Valhalla

> **Major feature:** a library inside Valhalla containing lore.

-   [ ] Build Valhalla library.
-   [ ] Create librarian NPC.
-   [ ] Create books.
-   [ ] Create shelves/categories.
-   [ ] Create readable-book UI.
-   [ ] Create search.
-   [ ] Create index.
-   [ ] Create cross-references.
-   [ ] Create mythology section.
-   [ ] Create world-history section.
-   [ ] Create gods section.
-   [ ] Create realms section.
-   [ ] Create creatures section.
-   [ ] Create runes section.
-   [ ] Create Yggdrasil section.
-   [ ] Create Ragnarök section.
-   [ ] Create character section.
-   [ ] Create player-discovered lore section.
-   [ ] Track unread/read books.
-   [ ] Track discovered information.
-   [ ] Allow books to unlock as the player learns things.

------------------------------------------------------------------------

# 41. The Community-Maintained Tome

> In-universe humor: the librarian describes one particular book as
> being maintained by the godly "Reddit".

The joke is that it is **not actually Reddit**.

-   [ ] Create a special community-maintained tome.
-   [ ] Make it a real living documentation project.
-   [ ] Do not assign normal in-world authorship.
-   [ ] Show authorship/history only when the player accesses the
    corresponding webpage/repository concept.
-   [ ] Make the tome continuously updateable outside the game.
-   [ ] Build a web documentation version.
-   [ ] Build a Git-based repository.
-   [ ] Create pull-request-based contributions.
-   [ ] Require review/checking before changes are accepted.
-   [ ] Generate/build the in-game book from the accepted data.
-   [ ] Preserve version/history information.
-   [ ] Add humorous librarian commentary.
-   [ ] Keep the in-game book itself presented as community-maintained.
-   [ ] Make this a genuine community contribution system rather than
    merely a fake UI.

------------------------------------------------------------------------

# 42. "Wikipedia" Section

-   [ ] Create an in-game encyclopedia-style section covering the entire
    game/world.
-   [ ] Include realms.
-   [ ] Include characters.
-   [ ] Include factions.
-   [ ] Include creatures.
-   [ ] Include items.
-   [ ] Include locations.
-   [ ] Include quests.
-   [ ] Include events.
-   [ ] Include mythology.
-   [ ] Include player-discovered information.
-   [ ] Include links/cross-references.
-   [ ] Track unknown entries.
-   [ ] Track incomplete entries.
-   [ ] Distinguish player-known facts from unknown information.
-   [ ] Integrate the community-maintained tome where appropriate.

------------------------------------------------------------------------

# 43. Wyrda

> **Wyrda is the game's one major addition to the mythological
> setting.**

-   [ ] Define Wyrda.
-   [ ] Establish Wyrda as associated with Odin's birds.
-   [ ] Present Huginn, Muninn, and Wyrda as the trio.
-   [ ] Wyrda represents destiny/fate.
-   [ ] Establish her personality.
-   [ ] Establish her gameplay role.
-   [ ] Establish her relationship to Odin.
-   [ ] Establish her relationship to Huginn.
-   [ ] Establish her relationship to Muninn.
-   [ ] Establish her relationship to the player.
-   [ ] Decide whether she appears physically, spiritually, or both.
-   [ ] Create Wyrda-related quests.
-   [ ] Create Wyrda-related lore.
-   [ ] Integrate fate/destiny themes without turning the game into a
    simple predetermined storyline.

------------------------------------------------------------------------

# 44. Fate / Choice System

-   [ ] Decide how much fate is predetermined.
-   [ ] Allow meaningful player choice.
-   [ ] Allow player choices to alter world state.
-   [ ] Avoid a simplistic good/evil meter.
-   [ ] Track major decisions.
-   [ ] Track faction consequences.
-   [ ] Track divine relationships.
-   [ ] Track realm consequences.
-   [ ] Track Ragnarök decisions.
-   [ ] Track irreversible events.
-   [ ] Allow ambiguity.
-   [ ] Build lore around fate vs agency.

------------------------------------------------------------------------

# 45. Ragnarök

> Ragnarök is a major world-state transition, not a button.

## Trigger route

-   [ ] Create Múspell-origin route.
-   [ ] Establish prerequisites.
-   [ ] Establish knowledge required.
-   [ ] Establish locations required.
-   [ ] Establish relationships required.
-   [ ] Establish narrative discovery.
-   [ ] Do not make "start Ragnarök" an arbitrary menu option.

## World transition

-   [ ] Create global Ragnarök state.
-   [ ] Change timeline/world state.
-   [ ] Change god behavior.
-   [ ] Change settlements.
-   [ ] Change factions.
-   [ ] Change travel routes.
-   [ ] Change Yggdrasil.
-   [ ] Change quests.
-   [ ] Change NPC behavior.
-   [ ] Change environmental conditions.
-   [ ] Change accessible/inaccessible areas.
-   [ ] Change enemy populations.
-   [ ] Change divine relationships.
-   [ ] Change dialogue.
-   [ ] Change music/audio.
-   [ ] Change lore.
-   [ ] Persist the transition permanently or according to the final
    narrative design.

## Philosophical routes

-   [ ] Ragnarök is necessary.
-   [ ] Ragnarök should be prevented.
-   [ ] Ragnarök can be controlled.
-   [ ] Player triggers it regardless.
-   [ ] Allow the player to resist.
-   [ ] Allow the player to embrace it.
-   [ ] Allow the player to manipulate it.
-   [ ] Avoid reducing these choices to "good" and "evil".

## Narrative realization

-   [ ] Establish why Ragnarök may be necessary.
-   [ ] Seed evidence throughout the game.
-   [ ] Make the player initially question why anyone would trigger it.
-   [ ] Reveal the larger cycle/reset problem.
-   [ ] Allow the player to reinterpret earlier events.
-   [ ] Make the final choice emotionally and philosophically
    meaningful.

------------------------------------------------------------------------

# 46. World State System

-   [ ] Global flags.
-   [ ] Realm flags.
-   [ ] Regional flags.
-   [ ] Faction flags.
-   [ ] NPC flags.
-   [ ] Quest flags.
-   [ ] Divine relationship flags.
-   [ ] Rune knowledge flags.
-   [ ] Yggdrasil access flags.
-   [ ] Ragnarök state.
-   [ ] Time state.
-   [ ] Weather state where persistent.
-   [ ] Settlement state.
-   [ ] Door network state.
-   [ ] Major-event state.
-   [ ] Save all persistent consequences.

------------------------------------------------------------------------

# 47. Time

-   [ ] Day/night cycle.
-   [ ] Time-of-day.
-   [ ] Calendar.
-   [ ] Days.
-   [ ] Seasons if useful.
-   [ ] NPC schedules.
-   [ ] World events.
-   [ ] Divine schedules.
-   [ ] Realm-specific time behavior.
-   [ ] Time progression during travel.
-   [ ] Save/load time.
-   [ ] Time-dependent quests.
-   [ ] Time-dependent world states.

------------------------------------------------------------------------

# 48. Weather

-   [ ] Clear weather.
-   [ ] Clouds.
-   [ ] Rain.
-   [ ] Snow.
-   [ ] Fog.
-   [ ] Storms.
-   [ ] Wind.
-   [ ] Lightning.
-   [ ] Realm-specific weather.
-   [ ] Weather transitions.
-   [ ] Gameplay effects where appropriate.
-   [ ] Audio.
-   [ ] Visual effects.

------------------------------------------------------------------------

# 49. World Streaming

> Critical for the planned huge worlds.

-   [ ] Choose chunk size.
-   [ ] Implement chunk coordinates.
-   [ ] Load nearby chunks.
-   [ ] Unload distant chunks.
-   [ ] Async loading.
-   [ ] Async unloading.
-   [ ] Loading priorities.
-   [ ] Streaming radius.
-   [ ] Hysteresis to prevent load/unload thrashing.
-   [ ] Persistent chunk state.
-   [ ] Dynamic entity serialization.
-   [ ] Static object generation.
-   [ ] Procedural/seeded generation if used.
-   [ ] Debug chunk visualization.
-   [ ] Loading profiler.
-   [ ] Memory budgeting.
-   [ ] Disk I/O budgeting.
-   [ ] Realm boundary streaming.
-   [ ] Yggdrasil streaming.
-   [ ] Save-safe streaming.

------------------------------------------------------------------------

# 50. Level of Detail

-   [ ] LOD 0: fully detailed nearby geometry.
-   [ ] LOD 1: reduced geometry.
-   [ ] LOD 2: simplified geometry.
-   [ ] LOD 3: extremely simplified representation.
-   [ ] Impostors/billboards where appropriate.
-   [ ] Distance-based animation reduction.
-   [ ] Distance-based AI reduction.
-   [ ] Distance-based physics reduction.
-   [ ] Distance-based audio reduction.
-   [ ] Distance-based particle reduction.
-   [ ] LOD debugging.
-   [ ] Smooth LOD transitions.

------------------------------------------------------------------------

# 51. Spatial Partitioning

-   [ ] Research spatial structures.
-   [ ] Start with a simple grid/chunk system.
-   [ ] Add broad-phase collision queries.
-   [ ] Add entity lookup by region.
-   [ ] Add proximity queries.
-   [ ] Add AI perception queries.
-   [ ] Add rendering culling.
-   [ ] Add streaming queries.
-   [ ] Consider BVH/octree/quadtree only if profiling justifies it.
-   [ ] Avoid overengineering before the world needs it.

------------------------------------------------------------------------

# 52. Instancing

-   [ ] Identify repeated assets.
-   [ ] Instance vegetation.
-   [ ] Instance rocks.
-   [ ] Instance buildings where possible.
-   [ ] Instance repeated decorations.
-   [ ] Batch compatible rendering.
-   [ ] Reduce draw calls.
-   [ ] Profile draw-call counts.
-   [ ] Add instance LOD.

------------------------------------------------------------------------

# 53. Simulation Levels

Implement different simulation detail based on distance.

-   [ ] Level 0 --- player-near full simulation.
-   [ ] Level 1 --- nearby full-ish simulation.
-   [ ] Level 2 --- regional simplified simulation.
-   [ ] Level 3 --- world-level abstract simulation.
-   [ ] Level 4 --- unloaded/persistent state only.
-   [ ] Transition entities between simulation levels.
-   [ ] Preserve important state.
-   [ ] Prevent obvious simulation discontinuities.
-   [ ] Profile CPU usage by simulation level.

------------------------------------------------------------------------

# 54. Physics / Collision

-   [ ] Player collision.
-   [ ] World collision.
-   [ ] Static object collision.
-   [ ] Terrain collision.
-   [ ] Trigger volumes.
-   [ ] Interaction volumes.
-   [ ] Damage volumes.
-   [ ] Environmental hazards.
-   [ ] Projectile collision if required.
-   [ ] Character collision.
-   [ ] Decide whether raylib collision primitives are sufficient.
-   [ ] Introduce a dedicated physics library only if the game's needs
    justify it.

------------------------------------------------------------------------

# 55. Terrain

-   [ ] Terrain representation.
-   [ ] Heightmaps or procedural terrain.
-   [ ] Terrain chunks.
-   [ ] Terrain LOD.
-   [ ] Terrain collision.
-   [ ] Terrain materials.
-   [ ] Grass.
-   [ ] Rocks.
-   [ ] Trees.
-   [ ] Rivers.
-   [ ] Lakes.
-   [ ] Mountains.
-   [ ] Caves.
-   [ ] Cliffs.
-   [ ] Roads.
-   [ ] Terrain streaming.

------------------------------------------------------------------------

# 56. Buildings

-   [ ] Building data format.
-   [ ] Modular building pieces.
-   [ ] Exterior.
-   [ ] Interior.
-   [ ] Doors.
-   [ ] Windows.
-   [ ] Furniture.
-   [ ] Collision.
-   [ ] Interior streaming.
-   [ ] NPC schedules inside buildings.
-   [ ] Ownership.
-   [ ] Faction association.
-   [ ] Destruction/state changes where required.
-   [ ] Realm-specific architecture.

------------------------------------------------------------------------

# 57. Doors

Doors are important because of Dagaz.

-   [ ] Door entity.
-   [ ] Door ID.
-   [ ] Destination ID.
-   [ ] Local destination.
-   [ ] Realm destination.
-   [ ] Yggdrasil destination.
-   [ ] Dagaz-compatible destination.
-   [ ] Locked state.
-   [ ] Quest state.
-   [ ] Faction access.
-   [ ] World-state access.
-   [ ] Door animation.
-   [ ] Door interaction.
-   [ ] Destination UI.
-   [ ] Persistence.

------------------------------------------------------------------------

# 58. Altars

-   [ ] Altar entity.
-   [ ] Altar activation.
-   [ ] Realm association.
-   [ ] Destination association.
-   [ ] Yggdrasil access.
-   [ ] Dagaz integration.
-   [ ] Save state.
-   [ ] Visual effect.
-   [ ] Audio effect.
-   [ ] Discovery state.
-   [ ] Fast-travel/threshold UI if appropriate.

------------------------------------------------------------------------

# 59. Audio

-   [ ] Music system.
-   [ ] Ambient sound.
-   [ ] Footsteps.
-   [ ] Combat sounds.
-   [ ] UI sounds.
-   [ ] Environmental sounds.
-   [ ] Weather sounds.
-   [ ] Realm themes.
-   [ ] Dynamic music.
-   [ ] Threat music.
-   [ ] Ratatoskr audio.
-   [ ] Yggdrasil audio.
-   [ ] Ragnarök audio.
-   [ ] Spatial audio if supported/needed.
-   [ ] Audio streaming.
-   [ ] Volume settings.
-   [ ] Mute controls.

------------------------------------------------------------------------

# 60. Graphics / Visual Identity

-   [ ] Establish visual style.
-   [ ] Establish lighting style.
-   [ ] Establish color language per realm.
-   [ ] Establish material language.
-   [ ] Establish architecture language.
-   [ ] Establish creature silhouettes.
-   [ ] Establish divine visual language.
-   [ ] Establish rune visual language.
-   [ ] Establish Yggdrasil visual language.
-   [ ] Establish Ragnarök visual transition.
-   [ ] Create shaders where useful.
-   [ ] Create sky systems.
-   [ ] Create fog.
-   [ ] Create volumetric-like effects where practical.
-   [ ] Create water.
-   [ ] Create fire.
-   [ ] Create ice.
-   [ ] Create magic/rune effects.

------------------------------------------------------------------------

# 61. UI

-   [ ] Main menu.
-   [ ] Pause menu.
-   [ ] Settings.
-   [ ] Inventory.
-   [ ] Equipment.
-   [ ] Rune screen.
-   [ ] Knowledge/lore.
-   [ ] Quest journal.
-   [ ] Map.
-   [ ] World/realm map.
-   [ ] Door destination interface.
-   [ ] Altar interface.
-   [ ] Dialogue.
-   [ ] Interaction prompts.
-   [ ] Notifications.
-   [ ] Loading screen.
-   [ ] Save/load UI.
-   [ ] Debug UI.
-   [ ] Accessibility settings.

------------------------------------------------------------------------

# 62. Maps

-   [ ] Local map.
-   [ ] Regional map.
-   [ ] Realm map.
-   [ ] Yggdrasil map.
-   [ ] Realm network map.
-   [ ] POIs.
-   [ ] Player marker.
-   [ ] NPC markers only where appropriate.
-   [ ] Quest markers.
-   [ ] Discovered locations.
-   [ ] Hidden locations.
-   [ ] Dagaz destinations.
-   [ ] Map fog/unexplored regions.
-   [ ] Ragnarök map changes.

------------------------------------------------------------------------

# 63. Save System

-   [ ] Save file format.
-   [ ] Versioned save format.
-   [ ] Player state.
-   [ ] World state.
-   [ ] Quest state.
-   [ ] NPC state.
-   [ ] Faction state.
-   [ ] Inventory.
-   [ ] Equipment.
-   [ ] Rune knowledge.
-   [ ] Door network.
-   [ ] Altar discovery.
-   [ ] Yggdrasil access.
-   [ ] World changes.
-   [ ] Ragnarök state.
-   [ ] Time.
-   [ ] Important dynamic objects.
-   [ ] Multiple save slots.
-   [ ] Autosave.
-   [ ] Manual save.
-   [ ] Save corruption protection.
-   [ ] Atomic save writes.
-   [ ] Backups.
-   [ ] Migration between save versions.

------------------------------------------------------------------------

# 64. Data Architecture

Prefer data-driven definitions for content.

-   [ ] Item definitions.
-   [ ] Rune definitions.
-   [ ] NPC definitions.
-   [ ] Creature definitions.
-   [ ] Faction definitions.
-   [ ] Quest definitions.
-   [ ] Dialogue definitions.
-   [ ] Building definitions.
-   [ ] World/realm definitions.
-   [ ] POI definitions.
-   [ ] Door definitions.
-   [ ] Altar definitions.
-   [ ] Lore definitions.
-   [ ] Book definitions.
-   [ ] God definitions.
-   [ ] World-state definitions.
-   [ ] Spawn definitions.

Potential data formats:

-   [ ] JSON.
-   [ ] TOML.
-   [ ] Custom binary format later if needed.
-   [ ] Decide based on editing convenience and runtime requirements.

------------------------------------------------------------------------

# 65. Asset Pipeline

-   [ ] Decide model format.
-   [ ] Decide texture formats.
-   [ ] Decide audio formats.
-   [ ] Decide shader pipeline.
-   [ ] Create asset registry.
-   [ ] Create asset IDs.
-   [ ] Create asset validation.
-   [ ] Create missing-asset handling.
-   [ ] Create asset loading.
-   [ ] Create asset unloading.
-   [ ] Create cache.
-   [ ] Create debug asset browser if useful.
-   [ ] Avoid requiring a 3D editor for ordinary world placement.

------------------------------------------------------------------------

# 66. Networking / Rust Exploration

> Networking can be a separate subsystem/project if useful. The goal is
> to learn Rust without forcing the entire game to be Rust.

-   [ ] Decide whether networking is actually needed for the final game.
-   [ ] Create a small Rust networking component if appropriate.
-   [ ] Define C++ ↔ Rust boundary only if needed.
-   [ ] Decide IPC vs static/shared library vs C ABI.
-   [ ] Keep the boundary narrow.
-   [ ] Define serialization format.
-   [ ] Define message types.
-   [ ] Handle connection state.
-   [ ] Handle errors.
-   [ ] Handle versioning.
-   [ ] Test latency.
-   [ ] Test packet loss.
-   [ ] Test reconnect.
-   [ ] Test malformed data.
-   [ ] Keep gameplay code independent of networking unless multiplayer
    becomes a real feature.
-   [ ] Do not add multiplayer merely because networking is interesting.

------------------------------------------------------------------------

# 67. Performance

-   [ ] Establish performance targets.
-   [ ] Establish target FPS.
-   [ ] Establish memory budget.
-   [ ] Establish loading-time target.
-   [ ] Profile CPU.
-   [ ] Profile GPU.
-   [ ] Profile memory.
-   [ ] Profile asset loading.
-   [ ] Profile streaming.
-   [ ] Profile AI.
-   [ ] Profile physics.
-   [ ] Profile rendering.
-   [ ] Profile draw calls.
-   [ ] Profile allocations.
-   [ ] Add frame-time graph.
-   [ ] Add subsystem timers.
-   [ ] Add entity count.
-   [ ] Add chunk count.
-   [ ] Add memory counters.
-   [ ] Add debug culling view.
-   [ ] Add LOD visualization.

------------------------------------------------------------------------

# 68. Debug Tools

-   [ ] FPS counter.
-   [ ] Frame time.
-   [ ] Camera position.
-   [ ] Camera orientation.
-   [ ] Player coordinates.
-   [ ] Current realm.
-   [ ] Current chunk.
-   [ ] Loaded chunks.
-   [ ] Entity count.
-   [ ] NPC simulation level.
-   [ ] Draw-call count.
-   [ ] Memory usage.
-   [ ] Collision visualization.
-   [ ] Raycast visualization.
-   [ ] Navigation visualization.
-   [ ] Chunk boundaries.
-   [ ] Streaming radius.
-   [ ] LOD visualization.
-   [ ] World-state inspector.
-   [ ] Quest inspector.
-   [ ] Inventory inspector.
-   [ ] Rune inspector.
-   [ ] Teleport/debug travel.
-   [ ] Spawn entity command.
-   [ ] Time control.
-   [ ] Weather control.
-   [ ] Ragnarök debug toggle.
-   [ ] Save/load debug tools.

------------------------------------------------------------------------

# 69. Testing

## Unit tests

-   [ ] Vector/math helpers where applicable.
-   [ ] Serialization.
-   [ ] Save/load.
-   [ ] Quest conditions.
-   [ ] Rune logic.
-   [ ] World-state logic.
-   [ ] Inventory.
-   [ ] Faction relationships.
-   [ ] Door routing.
-   [ ] Dagaz network.
-   [ ] Data validation.

## Integration tests

-   [ ] Load world.
-   [ ] Enter/leave realm.
-   [ ] Stream chunk.
-   [ ] Save while streaming.
-   [ ] Load save.
-   [ ] Trigger quest.
-   [ ] Change faction state.
-   [ ] Activate rune.
-   [ ] Use Dagaz.
-   [ ] Enter Yggdrasil.
-   [ ] Trigger major world-state transition.

## Gameplay tests

-   [ ] Start game.
-   [ ] Walk around Miðgarðr.
-   [ ] Interact.
-   [ ] Discover rune.
-   [ ] Save.
-   [ ] Quit.
-   [ ] Reload.
-   [ ] Continue.
-   [ ] Reach settlement.
-   [ ] Reach other realm.
-   [ ] Use Yggdrasil.
-   [ ] Experience major world-state changes.

------------------------------------------------------------------------

# 70. Accessibility / Quality of Life

-   [ ] Key rebinding.
-   [ ] Mouse sensitivity.
-   [ ] FOV setting.
-   [ ] Audio volume controls.
-   [ ] Subtitle controls.
-   [ ] UI scaling.
-   [ ] Text size.
-   [ ] Color/visual accessibility options where practical.
-   [ ] Toggle/hold controls.
-   [ ] Camera options.
-   [ ] Motion-effect options.
-   [ ] Save recovery.
-   [ ] Clear interaction prompts.

------------------------------------------------------------------------

# 71. Localization

-   [ ] Separate text from code.
-   [ ] Localization IDs.
-   [ ] Translation table.
-   [ ] Font fallback.
-   [ ] Unicode support.
-   [ ] Rune symbols.
-   [ ] Special characters.
-   [ ] Dynamic text.
-   [ ] Pluralization.
-   [ ] Date/time formatting.
-   [ ] Test long translated strings.

------------------------------------------------------------------------

# 72. Mythology Research

For each mythology-inspired element:

-   [ ] Record source.
-   [ ] Record source date/tradition.
-   [ ] Record variant spellings.
-   [ ] Record uncertainty.
-   [ ] Record conflicting accounts.
-   [ ] Record whether an element is directly attested.
-   [ ] Record whether the game interpretation is invented.
-   [ ] Avoid presenting uncertain scholarship as absolute fact.
-   [ ] Maintain a lore research database.
-   [ ] Keep game canon separate from historical scholarship.

Special research areas:

-   [ ] Poetic Edda.
-   [ ] Prose Edda.
-   [ ] Heimskringla where relevant.
-   [ ] Other medieval sources.
-   [ ] Modern scholarship.
-   [ ] Rune scholarship.
-   [ ] Place-name/language research where useful.

------------------------------------------------------------------------

# 73. Lore Accuracy / Game Canon Separation

-   [ ] Define "historical/source-attested".
-   [ ] Define "reasonable interpretation".
-   [ ] Define "game-original".
-   [ ] Document intentional deviations.
-   [ ] Avoid claiming that invented mechanics are ancient Norse
    beliefs.
-   [ ] Make the game's mythology internally coherent.

------------------------------------------------------------------------

# 74. Content Pipeline

For each major content item:

``` text
Concept
→ Research
→ Design
→ Data definition
→ Implementation
→ Prototype
→ Playtest
→ Polish
→ Lore entry
→ Save integration
→ Streaming integration
→ Performance test
```

-   [ ] Create reusable content workflow.
-   [ ] Create content validation.
-   [ ] Create naming conventions.
-   [ ] Create ID conventions.
-   [ ] Create versioning.

------------------------------------------------------------------------

# 75. Development Milestones

## Milestone 0 --- Project

-   [x] CMake.
-   [x] raylib.
-   [x] Compiler.
-   [x] Debug build.
-   [x] Release build.
-   [x] Source globbing.
-   [x] Basic game class.

## Milestone 1 --- 3D Prototype

-   [x] Window.
-   [x] Game loop.
-   [x] 3D camera.
-   [x] Free camera.
-   [x] Mouse look.
-   [x] Pitch clamp.
-   [ ] Camera movement.
-   [ ] Cube.
-   [ ] Ground.
-   [ ] Lighting.

## Milestone 2 --- Miðgarðr Prototype

-   [ ] Tiny terrain.
-   [ ] Player.
-   [ ] Collision.
-   [ ] Basic interaction.
-   [ ] Tiny settlement.
-   [ ] NPC.
-   [ ] Door.
-   [ ] Altar.
-   [ ] First rune.
-   [ ] Save.

## Milestone 3 --- Core Systems

-   [ ] Entity architecture.
-   [ ] Components/data.
-   [ ] Inventory.
-   [ ] Equipment.
-   [ ] Dialogue.
-   [ ] Quests.
-   [ ] Factions.
-   [ ] Knowledge.
-   [ ] World state.

## Milestone 4 --- First Real Realm

-   [ ] Expand Miðgarðr.
-   [ ] Multiple settlements.
-   [ ] NPC schedules.
-   [ ] Factions.
-   [ ] Wildlife.
-   [ ] Combat.
-   [ ] Exploration.
-   [ ] First major questline.

## Milestone 5 --- Realm Architecture

-   [ ] Realm abstraction.
-   [ ] Multiple realms.
-   [ ] Realm transitions.
-   [ ] Realm-specific data.
-   [ ] Streaming.
-   [ ] Persistence.

## Milestone 6 --- Ásgarðr / Divine Systems

-   [ ] Ásgarðr.
-   [ ] Major gods.
-   [ ] Divine sanctuaries.
-   [ ] Divine quests.
-   [ ] Yggdrasil access.

## Milestone 7 --- Yggdrasil

-   [ ] Physical Yggdrasil.
-   [ ] Vertical traversal.
-   [ ] Ratatoskr.
-   [ ] Cape.
-   [ ] Níðhöggr.
-   [ ] Eagle/Veðrfölnir.
-   [ ] Realm connections.
-   [ ] Altars.

## Milestone 8 --- Dagaz Network/ all runes work

-   [ ] Door network.
-   [ ] Destination discovery.
-   [ ] Dagaz activation.
-   [ ] Altars.
-   [ ] Persistent destinations.
-   [ ] World-state-aware routing.

## Milestone 9 --- Large World

-   [ ] Chunk streaming.
-   [ ] LOD.
-   [ ] Spatial partitioning.
-   [ ] Instancing.
-   [ ] Simulation levels.
-   [ ] Performance profiling.
-   [ ] Large-scale Miðgarðr.
-   [ ] Additional realms.

## Milestone 10 --- Narrative Systems

-   [ ] Major factions.
-   [ ] Divine politics.
-   [ ] Origin-specific routes.
-   [ ] Major branching quests.
-   [ ] Fate system.
-   [ ] Wyrda.
-   [ ] Lore library.

## Milestone 11 --- Ragnarök

-   [ ] Múspell route.
-   [ ] Ragnarök prerequisites.
-   [ ] Global state transition.
-   [ ] Changed world.
-   [ ] Changed NPCs.
-   [ ] Changed gods.
-   [ ] Changed quests.
-   [ ] Changed Yggdrasil.
-   [ ] Narrative revelation.
-   [ ] Multiple philosophical outcomes.

## Milestone 12 --- Content Expansion

-   [ ] All realms.
-   [ ] Major gods.
-   [ ] Minor gods/figures.
-   [ ] Creatures.
-   [ ] Major settlements.
-   [ ] Major quests.
-   [ ] Secrets.
-   [ ] Library.
-   [ ] Community tome.

## Milestone 13 --- Polish

-   [ ] Visual polish.
-   [ ] Audio polish.
-   [ ] UI polish.
-   [ ] Combat polish.
-   [ ] NPC polish.
-   [ ] Streaming polish.
-   [ ] Save reliability.
-   [ ] Performance.
-   [ ] Accessibility.
-   [ ] Localization.
-   [ ] Bug fixing.

## Milestone 14 --- Release

-   [ ] Release candidate.
-   [ ] Clean install test.
-   [ ] Save migration test.
-   [ ] Performance test.
-   [ ] Hardware compatibility.
-   [ ] Crash handling.
-   [ ] Final lore review.
-   [ ] Final content review.
-   [ ] Final credits.
-   [ ] Packaging.
-   [ ] Release build.
-   [ ] Post-release patch pipeline.
-   [ ] Add Godmode (fly, invinvibility, no rune cooldown for singleplayer)

-----------------------------------------------------------------------
