# Pick.e

_A 2-day Rust + Bevy + Rapier learning project by Paul Connor_

---

## What is Pick.e?

**Pick.e** is a minimal 2D robot simulation created over a single weekend to explore game-style robotics logic using modern Rust tools.  
It features a curious little warehouse bot who can explore his environment using **LiDAR**, build a memory of what he’s seen, and collect objects scattered around the map.

This was my **first time using Rust, Bevy, Rapier, and WebAssembly**, and I built everything from scratch in just 2 days — including all code, visuals, and ECS architecture.

_ChatGPT provided help with learning Rust syntax, Bevy usage and file-structure practices, and code fixes — but all logic and integration was mine._

---

## 🚀 Project Goals

- Learn and use Rust in a simified robotics simulation context
- Understand Bevy's ECS model and real-time UI/rendering
- Use raycasts for simulating LiDAR in a 2D world
- Implement simple memory and fog-of-war
- Add collectible parcels and score tracking
- Explore auto-navigation behaviour

This isn't a polished game — it's a hands-on **robotics sandbox** and learning exercise.

---

## 🔧 Tech Stack

- **Rust**
- [Bevy](https://bevyengine.org/) — Entity Component System (ECS) game engine
- [Rapier](https://rapier.rs/) — Physics + Raycasting for LiDAR
- **WebAssembly (WASM)** — Targeting browser play (via GitHub Pages)

---

## 🎮 Current Features

- Top-down 2D map with walkable and blocked areas
- Manual WASD robot control
- Raycast-based simulated LiDAR sensor
- Real-time occupancy-grid-based mapping from LiDAR
- UI overlay with stats and performance info
- Pickups that disappear when touched
- Autonomous nav mode using frontier exploration
- Web demo hosted via GitHub Pages

---

## Planned Features

- Further develop navigation to allow all pickups to be collected

---

## 🖼️ Screenshots

<img src="./screenshot.png" alt="screenshot" style="width: 50%;"/>

---

## 📦 Try It (coming soon)

Click here to run **Pick.e** in your browser!

---

## 👨‍🔬 Author & Acknowledgements

Built in a single weekend by **Paul Connor** ([@paulwconnor-ai](https://github.com/paulwconnor-ai))  
with help and code suggestions from **ChatGPT**.

All design, debugging, and architectural choices were mine.
ChatGPT helped accelerate things but didn't write the project.

---

## ⚖️ License

Apache 2.0
