# Migration Plan: Immediate Mode Input API

## Objective

Transition `miniwin` from an Enum-stream based event system (`while let Some(evt) = window.event()`) to a purely **Immediate Mode Input** architecture. This will simplify the main loop, eliminate the massive `match` boilerplate, and perfectly align event processing with the existing closure-based drawing model.

## Core Architectural Changes

### 1. Removal of the `Event` Enum

The `Event` enum (and the `window.event()` queue drainer) will be completely removed. Instead, the `Window` struct will internally track the state of input devices and expose them via queryable methods.

### 2. Window Lifecycle (Quit & Close)

- **Problem**: Catching `Event::Quit` or `Event::CloseRequested` requires manual polling and a queue drain.
- **Solution**: The window will manage its own `open` boolean flag.
- **API**:
  - `window.open() -> bool`
  - `window.close()`
- **Usage**: The main loop simplifies to `while window.open() { ... }`.

### 3. Continuous Input Tracking

- **Problem**: Tracking WASD movement or mouse holds with an event stream forces the user to maintain their own external booleans for every key they care about.
- **Solution**: The `Window` will track two states per frame: `current_keys` and `previous_keys` (and similarly for mouse buttons).
- **API**:
  - `window.input.is_down(Key)` (currently held)
  - `window.input.pressed(Key)` (just pressed this frame)
  - `window.input.released(Key)` (just released this frame)
  - `window.input.mouse_down(MouseButton)`
  - `window.input.mouse_pos() -> (f64, f64)`

### 4. Discrete Actions & Pattern Matching

- **Problem**: Immediate mode is notoriously bad for hotkeys/commands (e.g., "Press I for GI, O for AO"), usually forcing users into massive `if / else if` chains because pattern matching isn't naturally supported on boolean state queries.
- **Solution**: Expose an iterable slice of all keys that transitioned to "pressed" this specific frame. This brings the power of `match` back to immediate mode.
- **API**: `window.input.pressed_keys() -> &[Key]`
- **Usage**:
  ```rust
  for &key in window.input.pressed_keys() {
      match key {
          Key::Char('i') => toggle_gi(),
          Key::Char('o') => toggle_ao(),
          _ => {}
      }
  }
  ```

### 5. Stream-Based Data (Text, Scroll, & File Drops)

- **Problem**: Text input and drag-and-drop actions are inherently stream-based. A user can type multiple characters in one frame, meaning a single state boolean (`is_down`) cannot accurately capture typed text.
- **Solution**: The `Window` will accumulate these stream-based events into internal `Vec`s over the course of the frame, and expose them as read-only slices.
- **API**:
  - `window.input.text_input() -> &[char]` (or `&str`)
  - `window.input.dropped_files() -> &[PathBuf]`
  - `window.input.scroll_delta() -> (f64, f64)`

### 6. Frame Boundaries (The `draw` closure)

- **Problem**: Immediate mode input requires a strict concept of a "frame boundary" so the system knows exactly when to cycle `previous_keys = current_keys` and when to clear the `text_input` and `dropped_files` vectors.
- **Solution**: The existing `window.draw(|win| { ... })` method already pumps OS events to support drawing during resizing. It will now officially act as the frame boundary.
- **Internal Flow inside `draw`**:
  1. Cycle input states (`prev = current`).
  2. Clear temporary queues (`text_input`, `dropped_files`, deltas).
  3. Pump OS events (updating `current` state and populating temporary queues).
  4. Execute the user's render closure.

## Implementation Steps

1. **State Container**: Create a new `InputState` struct to house the HashSets/Bitsets for key tracking, mouse tracking, and vectors for text/files.
2. **Integration**: Embed `InputState` (or expose it via an `.input()` accessor) into the platform-specific `Window` structs (`macos/window.rs`, `windows/window.rs`).
3. **OS Pumping**: Modify the OS event pump (`TranslateMessage`/`DispatchMessage` on Windows, `nextEventMatchingMask` on macOS) to update the `InputState` instead of pushing to an `Event` queue.
4. **Trait Updates**: Add the public accessor methods (`pressed_keys()`, `text_input()`, `should_close()`, etc.) to the `Window` trait in `lib.rs`.
5. **Cleanup**: Delete the `Event` enum completely.
6. **Example Migration**: Rewrite `examples/demo.rs` to demonstrate the new, clean immediate-mode loop.
