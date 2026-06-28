# macOS Zero-Dependency Windowing Library Implementation Plan

This document outlines the clear, separated stages for implementing a zero-dependency, macro-free macOS windowing library in Rust.

## Stage 1: The FFI Foundation (Objective-C Runtime)

**Goal:** Establish the unsafe bindings to macOS `libobjc` and define the raw C types required for communication.

* **Types:** Define opaque pointers: `id`, `SEL`, `Class`.
* **Linkage:** Link `libobjc.A.dylib`.
* **Core Functions:** Bind `objc_getClass`, `sel_registerName`, `objc_msgSend` (which will require extensive use of `std::mem::transmute`), `objc_allocateClassPair`, `class_addMethod`, and `objc_registerClassPair`.
* **C-Structs:** Define `#[repr(C)]` geometry structs (`NSPoint`, `NSSize`, `NSRect`) and map necessary constants (`NSWindowStyleMask`, `NSEventType`, `NSEventMask`).

## Stage 2: Core Architecture & Windowing

**Goal:** Bootstrap the application and instantiate a window on the screen with full capability support.

* **Application Init:** Retrieve `NSApplication sharedApplication`.
* **Activation:** Set activation policy to `NSApplicationActivationPolicyRegular` so the app appears in the Dock and receives input.
* **Menu Bar:** Programmatically create a minimal `NSMenu` and `NSMenuItem` so the app behaves like a first-class macOS citizen (fixes focus issues).
* **Window Creation & Capabilities:** Provide a configuration API to support all window styles:
  * Standard titled and resizable windows.
  * Borderless and transparent windows (custom drawn).
  * Fullscreen (Workspace): Native macOS fullscreen that transitions the window to a dedicated workspace.
  * Fullscreen (Monitor Fit): A borderless window that scales to cover the exact dimensions of the current monitor without transitioning spaces.
* **View Hierarchy:** Allocate an `NSView`, enable CoreAnimation (`setWantsLayer: YES`), and attach it to the window (`setContentView:`).

## Stage 3: Event Handling & Run Loop

**Goal:** Intercept OS events and pipe them into a Rust-friendly enum without yielding thread control to Apple.

* **Delegate Class:** Dynamically generate an Objective-C class (`RustWindowDelegate`) at runtime using `objc_allocateClassPair`.
* **Method Injection:** Bind Rust `extern "C"` functions to `windowShouldClose:` and `windowDidResize:` using `class_addMethod`.
* **Event Polling (`poll_events`):** Implement a non-blocking `poll_events()` method on the `EventLoop` so the user can control their own main `while` loop. Utilize `nextEventMatchingMask:untilDate:inMode:dequeue:` with `[NSDate distantPast]`.
* **Automatic Memory Management:** The `poll_events()` method will internally allocate and drain an `NSAutoreleasePool` every frame to prevent temporary macOS objects (like `NSEvent`) from leaking.
* **Event Dispatch & Scope:** Support a comprehensive suite of events. Parse all hardware input (keyboard, mouse clicks, movement, scrolling) and window lifecycle events, translating them into a robust safe Rust `enum Event`.

## Stage 4: The Graphics Pipeline (CPU Pixel Buffer)

**Goal:** Efficiently blit a raw `&[u32]` array to the screen using CoreGraphics and CoreAnimation.

* **Linkage:** Link `CoreGraphics` and `QuartzCore` (for `CALayer`).
* **Update Method:** Implement `Window::update_buffer(&mut self, pixels: &[u32], width: usize, height: usize)`.
* **CoreGraphics Mapping:** Use `CGDataProviderCreateWithData` and `CGImageCreate` to wrap the Rust slice into a GPU-ready image without copying memory.
* **Compositing:** Apply the `CGImage` to the window's `CALayer` via the `setContents:` message.
* **CF Memory Management:** Strictly enforce `CFRelease` rules for the `CGImage`, provider, and colorspace to prevent memory leaks.

## Stage 5: Polish & High DPI (Retina) Scaling

**Goal:** Ensure the window behaves natively on modern Apple displays and maps inputs correctly.

* **Retina Scaling:** Interrogate `backingScaleFactor`. Ensure that logical coordinates requested by the user are properly scaled to physical pixel buffers.
* **Resize Handling:** Recalculate dimensions on `windowDidResize:` and issue resize events to the user.
* **Coordinate Mapping:** Flip the Y-axis for mouse coordinates (macOS uses bottom-left origin) so they are presented as top-left to the user.
* **Input Parsing:** Parse `keyCode` and `modifierFlags` from `NSEvent` to map hardware keys and modifiers correctly.

## Stage 6: The Safe Rust Wrapper

**Goal:** Encapsulate all unsafe FFI behind a clean, idiomatic Rust API.

* **Structs:** `Window` (holds the `id`), `EventLoop` (manages the event queue and autorelease pool).
* **Memory Management (Drop):** Implement `Drop` for `Window` to call `[window release]`.
* **Thread Safety:** Implement `!Send` and `!Sync` (via `PhantomData`) for `Window` to prevent AppKit crashes caused by background thread UI manipulation.
* **Main Thread Guards:** Panic if window creation or event polling is attempted off the main thread.
