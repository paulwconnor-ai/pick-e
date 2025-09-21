# 🚀 Locus Robotics Simulation Showcase – Top 5 Features

This list outlines the 5 high-impact features I plan to complete to demonstrate my ability to build real-time, sensor-based robot simulations using Rust, Bevy ECS, and Rapier2D — all aligned with the Locus Robotics Simulation Engineer role.

---

## ✅ 1. `CmdVel` + `DiffDrive` Components

Create ECS components to separate control intent (`CmdVel`) from robot movement (`DiffDrive`).  
This mirrors real-world robot APIs (like ROS2) and shows architectural maturity beyond direct-input motion.

---

## ✅ 2. LiDAR Sensor (Raycast-Based)

Simulate a forward-facing LiDAR by casting rays in an arc and storing hit distances.  
This demonstrates accurate sensor modeling and real-time perception of obstacles using physics-based raycasts.

---

## ✅ 3. Occupancy Grid Mapping

Convert LiDAR hits into a basic 2D gridmap of seen obstacles (drawn as tiles, dots, or lines).  
Foundational for SLAM and digital twins — it proves your simulation can generate useful world state from raw sensor data.

---

## ✅ 4. Reactive Navigation

Use LiDAR readings to steer the robot toward open space and avoid walls.  
Simple real-time autonomy based on perception, without requiring global maps — visually effective and robotics-grounded.

---

## ✅ 5. Simulation Metrics Logging

Track key metrics like `time_to_goal`, number of wall collisions, and total steps taken.  
Fulfills Locus's requirement for evaluating simulation accuracy and performance.

---

## 🧩 Bonus (Stretch Goals)

- Load map from file (`assets/levels/maze01.txt`)
- Goal-tile marker and auto-drive toward it
- **Browser-deployable build using WebAssembly**
- **Live deployment via AWS S3 + CloudFront for public access**
- look at generating collision-cache instead from high-res "beauty" image - using "blueness" to detect the walls - might give better results than low-res mask
- tidy up presentation of beauty image

---

## 📦 Final Deliverables (Application Assets)

- `README.txt` (final write-up explaining the project, tech choices, and alignment with role)
- CV (updated to highlight simulation, ECS, Bevy/Rust, and robotics experience)
- Covering letter (focused on Locus Robotics, this sim project, and future potential)
- Submission of job application with live demo link + GitHub repo (optional)

---

_All features scoped for completion in ~9 hours total. Designed to match the Locus Robotics job description by demonstrating ECS architecture, sensor modeling, autonomous behavior, and performance metrics — with a clean, testable sim that runs both natively and in the browser._
