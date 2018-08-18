# ksrl - Kobold Slayer Roguelike
A roguelike idea I initially prototyped in C#. I wanted to re-make it in Rust to get to help get to grips with the language. The pain points in the prototype were the lack of planned architecture and the method by which the game interacted with the display, particularly concerning particles.

# Build Instructions
1. Install Cargo using rustup https://rustup.rs/
2. Download the repo
3. Download a libtcod terminal.png (the one at https://bitbucket.org/libtcod/libtcod/src works) and place it in the root folder
4. navigate to directory and call ```Cargo run```

# Gameplay Overview
A lone adventurer travelling through kobold warrens. Watch out, for you are big and the tunnels are small, you are one and the kobolds are endless. Keep your wits about you, for the kobolds are cunning and have many traps.

- Weapons have swing arcs and can get caught in terrain, choose the right weapon for the situation
- Kobolds do not mindlessly run at you, but flank and direct to smaller passageways
- Perform abilities dependent on the surrounds (spring off a wall, pin an enemy to a wall, manoeuvre around enemies in wider spaces)

# Plan

- Get a particle system that is functional implemented, and allow for two display modes:
  - Awaiting execution of certain particles before input can be performed (e.g. an arrow being fired down a hallway, you must wait until it finishes before you can move
  - Continuous particles that display and update whilst input is awaited (a fire will burn continuously and not block input)
- Prototype visuals / ui
- Wrapper around the current input system to allow for meaning to be assigned to inputs
- Menuing model that allows for nested UI, scrollable menus, and info boxes.
- Map generation / representation
- Gameobject data model for enemies, objects and the player
- Weapon system
- AI
