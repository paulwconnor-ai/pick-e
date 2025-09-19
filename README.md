# 🤖 Pick.e

*A minimalist warehouse robot simulation built in Rust using Bevy and Rapier.*

![screenshot_placeholder](./screenshot.png)

---

## Overview

**Pick.e** is a small, autonomous warehouse robot with a big job:  
Pick up parcels, dodge hazards, and explore a world he can only *partially* see — using simulated **LiDAR**, **odometry**, and a growing memory of the environment around him.

This project was built over a single weekend as a personal game-jam to:

- Learn Rust and Bevy for real-time simulation
- Explore ECS-based sim architecture
- Model perception and memory from a robot’s point of view
- Lay the groundwork for future AI or RL experiments

---

## Features (in progress)

- ✅ 2D top-down warehouse layout
- ✅ Circular robot with WASD control
- ✅ Raycast-based LiDAR simulation
- ✅ Fog of war and map memory
- 🚧 Package pickup and delivery
- 🚧 Autonomous mode toggle
- 🚧 Hazards (forklifts) with patrol logic
- 🚧 Scoring system and simple HUD
- 🚧 WASM/WebGL build for browser play

---

## Tech Stack

- 🦀 Rust
- 🎮 [Bevy](https://bevyengine.org/) — ECS game engine
- 🧩 [Rapier](https://rapier.rs/) — 2D physics and raycasting
- 🌐 WebAssembly (WASM) + GitHub Pages (planned)

---

## Try It

> Coming soon: playable browser demo via GitHub Pages

You'll be able to try Pick.e right in your browser — no install required.

---

## License

Apache-2.0

---

## Screenshots

> Visuals coming soon — robot, fog-of-war, LiDAR rays, and delivery mechanics in action.
