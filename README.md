# Mimir-Engine - Game Engine implementation in Rust

A custom 2D/3D game engine built from scratch in **Rust**, designed to master low-level systems programming, graphics pipelines (`wgpu`), data-oriented design, and engine architecture.

---

## 🛠️ Tech Stack & Dependencies
*   **Language:** Rust (Edition 2021)
*   **Windowing & Events:** `winit`
*   **Graphics API:** `wgpu` (WebGPU implementation for Vulkan, Metal, DX12, OpenGL)
*   **Linear Algebra:** `glam` (Vectors, Matrices, Quaternions)
*   **Entity Component System (ECS):** `hecs`
*   **Asset Decoding:** `image`
*   **Utilities:** `pollster`, `bytemuck`, `env_logger`, `log`

---

## 🚀 Progress Tracking & Roadmap

### Phase 1: The Foundation & Windowing ✅ (Completed)
- [x] **Step 1.1:** Setup Cargo workspace and configuration.
- [x] **Step 1.2:** Cross-platform OS window creation using `winit`.
- [x] **Step 1.3:** Custom fixed/variable game loop with Delta Time ($\Delta t$) calculation for frame-rate independent movement.

### Phase 2: Graphics & Rendering Basics ✅ (Completed)
- [x] **Step 2.1:** Hardware graphics initialization and device/queue handling via `wgpu`.
- [x] **Step 2.2:** Surface configuration and swapchain management with dynamic window resizing support.
- [x] **Step 2.3 & 2.4:** GPU vertex buffers, WGSL (WebGPU Shading Language) shaders, and rendering geometry primitives.

### Phase 3: Architecture & The Engine Core ✅ (Completed)
- [x] **Step 3.1 & 3.2:** Modular architecture layout (`main.rs`, `engine.rs`, `components.rs`, `input.rs`).
- [x] **Step 3.3:** Data-oriented ECS setup using `hecs` for entity management.
- [x] **Step 3.4:** Linear algebra matrix transformations via `glam` combined with GPU Uniform Buffers.

### Phase 4: Expanding Subsystems 🔄 (In Progress)
- [x] **Step 4.1:** Keyboard input mapping system (WASD / Arrow Keys controlling entities).
- [x] **Step 4.2:** Multi-entity rendering with independent GPU bind groups and uniform buffers.
- [x] **Step 4.3:** Texture and Asset Management (Procedural and image-based texture decoding with UV coordinate mapping and sampler binding).
- [x] **Step 4.4:** Camera & Projection Matrices (View-Projection matrix scaling and world panning/zooming).
- [x] **Step 4.5:** Physics & Collision Detection (Axis-Aligned Bounding Box / AABB collision systems or `rapier` integration).
- [ ] **Step 4.6:** Audio Subsystem (Sound effects and background music via `rodio` or `kira`).

### Phase 5: The Proof-of-Concept Game ⏳ (Pending)
- [ ] Game state machine (Menu, Gameplay, Game Over).
- [ ] Build a playable mini-game (e.g., Pong, Breakout, or a top-down shooter) to stress-test the engine.
- [ ] Performance profiling and optimization.

---

## 🏃 Getting Started & Running

1. **Clone or initialize the repository:**
   ```bash
   git clone https://github.com/ramajd/mimir-engine.git
   cd mimir-engine
   ```

2. **Run the engine:**
   ```bash
   cargo run
   ```

3. **Controls:**
   * Use **W, A, S, D** or **Arrow Keys** to move the player entity.
   * Press **Escape** to exit the application.
