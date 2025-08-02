# Smart Intersection Simulation

![Demo](example.gif)

## Overview

This project simulates a smart intersection management system for autonomous vehicles (AVs) without using traditional traffic lights. The simulation demonstrates how AVs can safely and efficiently navigate a cross intersection using a custom algorithm that minimizes congestion and prevents collisions, inspired by real-world research into next-generation traffic control for self-driving cars.

## Features
- **No Traffic Lights:** Vehicles are managed by a smart intersection algorithm, not by signals.
- **Autonomous Vehicle Physics:** Each AV has multiple velocities, respects a safety distance, and cannot change lanes mid-intersection.
- **Keyboard Controls:** Spawn vehicles from any direction using arrow keys.
- **Random Vehicle Generation:** Press `R` to auto-generate random vehicles for 60 seconds.
- **Statistics Window:** On exit, see stats like max/min time to cross, total cars, etc.
- **Fun Extras:** Press `P` to spawn a plane!

## How It Works
- The intersection is a standard cross with three lanes per direction: left, straight, right.
- Each vehicle is assigned a random route (left, straight, right) and enters from a chosen direction.
- Vehicles follow their lane and route, maintaining a safe distance from others.
- The smart intersection algorithm gives priority to vehicles already in the intersection and prevents conflicting movements, ensuring no collisions.
- Vehicles are animated as they move and turn, with their orientation changing as needed.
- When you exit (Esc), a stats window summarizes the simulation.

## Controls
- **Arrow Up:** Spawn vehicle from south to north
- **Arrow Down:** Spawn vehicle from north to south
- **Arrow Right:** Spawn vehicle from west to east
- **Arrow Left:** Spawn vehicle from east to west
- **R:** Auto-generate random vehicles for 60 seconds
- **Esc:** Exit simulation and show statistics
- **P:** Spawn a plane (for fun)

## Technologies Used
- **Rust**
- **SDL2** (with `image` and `ttf` features)
- **rand** crate

## Installation & Running
1. **Install Rust:** [https://rustup.rs/](https://rustup.rs/)
2. **Install SDL2:**
   - On Windows: Use [vcpkg](https://github.com/microsoft/vcpkg) or download from [libsdl.org](https://www.libsdl.org/download-2.0.php)
   - On macOS: `brew install sdl2 sdl2_image sdl2_ttf`
   - On Linux: `sudo apt-get install libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev`
3. **Clone this repo and enter the directory:**
   ```sh
   git clone https://github.com/yourusername/smart-road.git
   cd smart-road
   ```
4. **Run the simulation:**
   ```sh
   cargo run --release
   ```
5. **Assets:**
   - Ensure the `assets/` folder is present with all required images and fonts.

## Project Structure
- `src/main.rs` — Main simulation loop, rendering, event handling, statistics
- `src/car.rs` — Car/vehicle logic, physics, collision avoidance, animation
- `src/spawn_cars.rs` — Spawning vehicles based on user input and random generation
- `assets/` — Images for cars, planes, roads, and font (Roboto.ttf)
- `example.gif` — Demo animation of the simulation
- `Cargo.toml` — Rust dependencies

