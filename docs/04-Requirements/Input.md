# Mimir Engine — Input Requirements

**Status:** Draft
**Version:** 0.1
**Area:** Input
**Scope:** MVP
**Related Documents:**

* `01-Vision.md`
* `02-PRD.md`
* `03-Feature-List.md`
* `04-Requirements/README.md`
* `04-Requirements/Engine.md`
* `04-Requirements/Window.md`

---

# 1. Purpose

This document defines the product requirements for the Mimir input system.

The input system provides applications with access to basic user input from the keyboard and mouse.

---

# 2. Scope

The MVP includes:

* Keyboard input
* Mouse button input
* Mouse position
* Mouse movement
* Input state
* Pressed/released state detection
* Frame-based input access

---

# 3. Requirements

## REQ-INP-001 — Keyboard Input

**Priority:** Must
**Scope:** MVP

The engine must provide access to keyboard input.

### Acceptance Criteria

* Keyboard input can be detected while the application is running.
* Individual supported keys can be identified.
* Keyboard state can be queried by application logic.
* Keyboard input updates as the user interacts with the keyboard.

---

## REQ-INP-002 — Keyboard State

**Priority:** Must
**Scope:** MVP

The input system must provide the current state of supported keyboard keys.

### Acceptance Criteria

* The application can determine whether a key is currently pressed.
* The application can determine whether a key is currently released.
* State reflects the latest processed input events.

---

## REQ-INP-003 — Keyboard Press and Release

**Priority:** Must
**Scope:** MVP

The input system must distinguish between key press and key release events.

### Acceptance Criteria

* A key press can be detected.
* A key release can be detected.
* Press and release states are distinguishable during runtime.

---

## REQ-INP-004 — Mouse Button Input

**Priority:** Must
**Scope:** MVP

The engine must provide access to mouse button input.

### Acceptance Criteria

* Supported mouse buttons can be identified.
* The application can determine whether a mouse button is pressed.
* The application can determine whether a mouse button is released.

---

## REQ-INP-005 — Mouse Position

**Priority:** Must
**Scope:** MVP

The input system must provide the current mouse position.

### Acceptance Criteria

* The current mouse position can be queried.
* The position is updated as the mouse moves.
* The coordinate representation is consistent during runtime.

---

## REQ-INP-006 — Mouse Movement

**Priority:** Must
**Scope:** MVP

The engine must detect mouse movement.

### Acceptance Criteria

* Mouse movement can be detected.
* Movement information can be made available to application logic.
* Movement is updated during runtime.

---

## REQ-INP-007 — Frame-Based Input State

**Priority:** Must
**Scope:** MVP

The input system must provide a stable input state for each engine frame.

### Acceptance Criteria

* Input events are processed during runtime.
* The current input state can be queried during an update cycle.
* Input state advances consistently between frames.
* Input state does not depend on rendering implementation.

---

## REQ-INP-008 — Input Lifecycle

**Priority:** Must
**Scope:** MVP

The input system must operate within the engine lifecycle.

### Acceptance Criteria

* Input is initialized before application input can be queried.
* Input events are processed while the engine is running.
* Input processing stops when the engine shuts down.
* Input resources are released during shutdown.

---

# 4. Dependencies

```text
Engine Foundation
       │
       ▼
Window & Platform
       │
       ▼
Input
```

The input system depends on the window/platform layer for receiving relevant operating-system input events.

---

# 5. Out of Scope

The MVP does not require:

* Gamepad support
* Joystick support
* Touch input
* Gesture recognition
* Input rebinding
* Action mapping
* Text-input/IME systems
* Input recording
* Input playback

These may be added later.

---

# 6. Traceability

The requirements originate from:

* Read keyboard input
* Read mouse input
* Track keyboard state
* Track mouse button state
* Track mouse position
* Track mouse movement
* Detect input events
* Distinguish pressed/released states
* Provide frame-based input state

---

# 7. Design Boundary

This document does not specify:

* Input library
* Event queue implementation
* Key-code representation
* Input buffering strategy
* Input polling strategy
* Platform-specific input APIs

Those decisions belong to Architecture and Subsystem Design.

---

# 8. Completion Criteria

The Input MVP is complete when an application can:

1. Detect keyboard input.
2. Query keyboard state.
3. Detect key presses and releases.
4. Detect mouse button input.
5. Query mouse position.
6. Detect mouse movement.
7. Access a consistent input state during each frame.
8. Operate input correctly throughout the engine lifecycle.
