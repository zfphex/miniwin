### Phase 1: The FFI Foundation (Objective-C Runtime)

Because you are avoiding all dependencies (including `libc` and `objc2`), your first step is to declare the raw C types and external functions required to communicate with macOS.

You will need to manually define:

- **Opaque Types:** `id` (pointer to an object), `SEL` (method selector), and `Class` (pointer to an Objective-C class). In Rust, these are typically represented as `*mut std::ffi::c_void`.
- **Core Runtime Functions:** You must link against `libobjc.A.dylib` and declare:

- `objc_getClass`: To retrieve class pointers (e.g., `NSWindow`).
- `sel_registerName`: To convert C-strings into method selectors.
- `objc_msgSend`: The workhorse of your library. Because `objc_msgSend` is variadic and its signature changes based on the method being called, you will have to use `std::mem::transmute` to cast it to the exact function signature every time you use it.
- `objc_allocateClassPair`, `class_addMethod`, and `objc_registerClassPair`: Necessary for creating custom delegate classes at runtime to handle events.

### Phase 2: Core macOS Types & Structs

You will need to recreate Apple's C-structs in Rust using `#[repr(C)]`.

- **Geometry:** `NSPoint` (x, y), `NSSize` (width, height), and `NSRect` (origin, size). These require `f64` values.
- **Enums:** You will need to manually port specific constants from Apple's headers:

- `NSWindowStyleMask` (Titled, Closable, Resizable, Miniaturizable).
- `NSBackingStoreType` (Buffered).
- `NSEventType` (KeyDown, MouseUp, MouseMoved, etc.).
- `NSEventMask` (For filtering the event loop).

### Phase 3: Porting AppKit & Window Management

To get a window on the screen, you must send messages to specific AppKit classes. You will need to look up and call methods on the following:

- **NSApplication:** \* `sharedApplication`: To initialize the app.

- `setActivationPolicy:`: To ensure your app appears in the Dock and can receive input (using `NSApplicationActivationPolicyRegular`).
- `finishLaunching`: To tell macOS your app is ready.
- **NSWindow:**

- `alloc` and `initWithContentRect:styleMask:backing:defer:`: To create the window.
- `makeKeyAndOrderFront:`: To show the window.
- `setDelegate:`: To attach your custom event handler.
- **NSView:**

- You will likely need a custom `NSView` subclass (created at runtime) attached to `window.setContentView:` if you plan to integrate a graphics API (like Metal or OpenGL) later.

### Phase 4: Input and Event Handling

Since you cannot use macros or Objective-C protocols directly, handling input requires dynamically building an Objective-C class in Rust at runtime.

1. **The Delegate Class:** Use `objc_allocateClassPair` to create a new class (e.g., `RustWindowDelegate`) inheriting from `NSObject`.
2. **Method Injection:** Use `class_addMethod` to inject Rust `extern "C"` functions into this Objective-C class. You will need to bind methods like:

- `windowShouldClose:`
- `windowDidResize:`

3. **The Event Loop:** You have two choices here, but for a Rust library, the **manual pump** is usually preferred over `[NSApp run]` so you don't yield control of the main thread to Apple.

- You will continuously call `nextEventMatchingMask:untilDate:inMode:dequeue:` on `NSApplication`.
- If an event exists, call `sendEvent:` to dispatch it.
- Map the resulting `NSEvent` types into your own safe Rust `enum Event { Resized, KeyPressed(Code), ... }`.

### Phase 5: The Safe Rust Wrapper

Once the raw FFI is working, you will wrap this unsafe, verbose code into a clean, idiomatic Rust API.

- **`Window`:** A struct holding the `id` of the `NSWindow`. It will implement `Drop` to ensure `[window release]` is called.
- **`EventLoop`:** A struct that yields events to the user, likely through a `.poll()` or `.run(callback)` method.

**The "No Macro" Reality Check**
To prepare you for the syntax, because you are not using `msg_send!`, a simple call to allocate a window will look something like this in your library:

```
let ns_window_class = objc_getClass(b"NSWindow\0".as_ptr() as *const i8);
let alloc_sel = sel_registerName(b"alloc\0".as_ptr() as *const i8);

// You must manually define the function signature for THIS specific call
let msg_send_id_id: extern "C" fn(id, SEL) -> id = std::mem::transmute(objc_msgSend);
let allocated_window = msg_send_id_id(ns_window_class, alloc_sel);

```

You will be writing this boilerplate for _every single interaction_ with macOS. It is highly recommended to write a few safe, generic inline Rust functions to wrap `transmute` for common method signatures (e.g., methods that take one `f64`, methods that return an `id`, etc.) to keep your codebase readable without relying on macros.

How do you want to handle the main application loop—do you prefer a blocking `run()` function that takes a closure, or a non-blocking `poll_events()` method that allows the user to manage their own while-loop?

### Phase 6: The `Window::new` Constructor

Since we are skipping the builder pattern, your `Window::new` function will take all essential parameters upfront.

To make this work, you will need to port strings and geometry to Objective-C over FFI:

- **Arguments:** `pub fn new(title: &str, width: f64, height: f64) -> Self`
- **Porting `NSString`:** You cannot pass a Rust `&str` directly to macOS. You must manually bridge it.

1. Look up the `NSString` class.
2. Call `alloc`.
3. Call `initWithBytes:length:encoding:` (UTF-8 encoding is usually `4`).
4. Call `window.setTitle:` with this string pointer.

- **Porting Geometry:** You will instantiate your `#[repr(C)] NSRect` using the provided width and height.
- **Initialization:** Inside `new`, you will chain the `NSWindow` allocation, set the title, call `makeKeyAndOrderFront:`, and return the `Window` struct containing the raw `id` pointer.

### Phase 7: The Non-Blocking Event Loop

Because you are not yielding control to `[NSApplication run]`, you are entirely responsible for keeping the macOS event queue moving. Without this, your window will appear as a frozen, unresponsive spinning beachball.

You will need to implement a `poll_events` method that drains the event queue per frame:

- **The Core Method:** You must manually bind and call `nextEventMatchingMask:untilDate:inMode:dequeue:` on the `NSApplication` singleton.
- **The Parameters:**

- `mask`: Pass `NSAnyEventMask` (usually `std::u64::MAX`) to catch everything.
- `untilDate`: This is the secret to making it **non-blocking**. You must look up the `NSDate` class and call the `distantPast` class method. Passing this date tells the OS to return immediately, even if the queue is empty.
- `inMode`: You will need to port the `NSDefaultRunLoopMode` string constant.
- `dequeue`: Pass `YES` (a C boolean `1`) to remove the event from the queue.
- **Dispatching:** If an event is returned (the pointer is not null), you must manually call `sendEvent:` on the `NSApplication` instance so macOS can handle native behaviors (like dragging the window by the title bar).
- **Translation:** Before dispatching, read the event's `type` and translate it into your safe Rust `Event` enum to return to the user.

### Phase 8: Manual Memory Management (The ARC-less Void)

Because you are using pure FFI without Rust's `objc` crates, you do not have Automatic Reference Counting (ARC). You are writing raw C-style Objective-C.

- **NSAutoreleasePool:** This is the most critical component of a non-blocking loop. macOS generates temporary objects (like `NSEvent` and `NSDate`) every single frame. If you do not clean them up, your app will rapidly leak memory.

- Inside your user's main `while` loop (or wrapped inside your `poll_events` function), you **must**:

1. Allocate and initialize an `NSAutoreleasePool` (`[NSAutoreleasePool alloc] init]`).
2. Poll your events.
3. Call `drain` or `release` on the pool at the end of the frame.

- **Window Deallocation:** In your `Window` struct's `Drop` implementation, you must explicitly send a `release` or `close` message to the `NSWindow` pointer.

### Phase 9: Linker Configuration

Since you have zero dependencies, you must tell the Rust compiler to link against the required Apple frameworks directly.

You will need to add these exact directives to the top of your FFI module or in a build script so the linker can find `objc_msgSend` and the AppKit symbols:

```
#[link(name = "AppKit", kind = "framework")]
#[link(name = "Foundation", kind = "framework")]
#[link(name = "objc")]
extern "C" {}

```

Now that we have the window initialized, memory managed, and the event loop manually ticking, how are you planning to handle graphics rendering inside this window—will you eventually want to bind a Metal layer to it, or just use a raw CPU pixel buffer for now?

### Phase 10: The Raw Pixel Pipeline (CoreGraphics & CoreAnimation)

To display a CPU buffer efficiently on modern macOS, you will wrap your raw bytes in a `CGImage` (CoreGraphics Image) and assign it directly to the window's underlying `CALayer` (CoreAnimation Layer). The OS will then use the GPU to composite your CPU-rendered image onto the screen automatically.

Here is how to architect this pipeline:

#### 1. Linking Additional Frameworks

You will need to update your linker directives to include the C-based graphics frameworks:

```
#[link(name = "CoreGraphics", kind = "framework")]
#[link(name = "QuartzCore", kind = "framework")] // For CALayer

```

#### 2. Modifying `Window::new` (Layer-Backed Views)

By default, an `NSWindow` does not have a high-performance animation layer. You must create an `NSView`, enable its layer, and attach it to the window.

- **Allocate `NSView`:** Call `alloc` and `initWithFrame:` using the same `NSRect` you used for the window.
- **Enable CoreAnimation:** Send the `setWantsLayer:` message to the view, passing `YES`.
- **Attach View:** Send the `setContentView:` message to your `NSWindow`, passing the view.
- **Store the Layer:** Retrieve the layer using `[view layer]` and store this `id` in your Rust `Window` struct. This prevents you from having to look it up every frame.

#### 3. Creating the `update_buffer` Method

You will expose a method on your `Window` struct that takes a slice of pixels: `pub fn update_buffer(&mut self, pixels: &[u32], width: usize, height: usize)`.

Inside this method, you will map your Rust slice directly into a macOS CoreGraphics image without copying the data:

- **Color Space:** Call `CGColorSpaceCreateDeviceRGB()` to tell macOS how to interpret your color channels.
- **Data Provider:** Call `CGDataProviderCreateWithData()`. This C-function takes a pointer to your pixel data (`pixels.as_ptr()`). It creates a CoreGraphics wrapper around your Rust memory.
- **Image Creation:** Call `CGImageCreate()`. This is a massive function signature. You will pass it:

- `width` and `height`
- `bitsPerComponent` (8 for standard color)
- `bitsPerPixel` (32 for standard color)
- `bytesPerRow` (width \* 4)
- The color space
- Bitmap info flags (e.g., `kCGImageAlphaNoneSkipFirst` to tell macOS it's an ARGB or XRGB buffer).
- The data provider.

#### 4. Pushing to the Screen

Once `CGImageCreate` returns a `CGImageRef`, you cross back over into Objective-C FFI for one single call:

- Use `objc_msgSend` to send the `setContents:` message to the `CALayer` you stored during window creation, passing the `CGImageRef` as the argument.

#### 5. Core Foundation Memory Management (The CFRelease Rule)

Unlike Objective-C objects which require `release` or an `NSAutoreleasePool`, CoreGraphics uses Core Foundation (CF) memory rules. Any C-function that contains the word "Create" returns an object you must manually release.

- At the end of your `update_buffer` function, after setting the layer contents, you **must** call:

- `CGImageRelease(cg_image)`
- `CGDataProviderRelease(provider)`
- `CGColorSpaceRelease(color_space)`

If you miss these, your application will hemorrhage memory every single frame.

This approach gives you incredibly fast CPU-to-screen blitting with zero external dependencies.

### Phase 11: Input Translation and Coordinate Mapping

You need to extract useful data from the raw `NSEvent` pointer in your non-blocking loop and translate it into safe Rust types.

- **Keyboard Mapping:** Call the `keyCode` method on the event. You will need to write a `match` statement that maps Apple's raw hardware keycodes (e.g., `0x00` for 'A', `0x31` for Space) to a clean Rust enum.
- **Modifier Keys:** Call `modifierFlags` to detect if Shift, Command, or Control are being held down during a keypress or mouse click.
- **Coordinate Flipping:** Call `locationInWindow` for mouse events. macOS uses a bottom-left origin for its coordinate system. You must subtract the Y-coordinate from the window's height to provide standard top-left coordinates to your users.

### Phase 12: High DPI (Retina) Scaling

If you ignore DPI scaling, your `&[u32]` pixel buffer will appear blurry or only fill a quarter of the window on modern Mac displays.

- **The Scale Factor:** You must look up and call the `backingScaleFactor` method on your `NSWindow`. This returns an `f64` (typically `1.0` for standard displays and `2.0` for Retina displays).
- **Buffer Sizing:** When the user calls `Window::new(width, height)`, treat those as logical points. The actual size of the `&[u32]` slice they need to provide must be `(width * scale_factor) * (height * scale_factor)`.
- **Resize Handling:** When a `windowDidResize:` event fires, you must recalculate the physical pixel dimensions and notify the user to resize their `&[u32]` buffer.

### Phase 13: The Application Menu and Activation

macOS applications without a menu bar behave erratically. They often cannot receive keyboard focus or intercept certain shortcuts.

- **The Main Menu:** You must instantiate an `NSMenu`, create an `NSMenuItem` for your application, and attach it via `[NSApp setMainMenu:]`.
- **Activation Policy:** Before showing the window, ensure you have called `setActivationPolicy:` with `NSApplicationActivationPolicyRegular`. This forces macOS to treat your raw binary as a first-class desktop application.

### Phase 14: Thread Safety and Safe Abstractions

Because you are interacting with UI components, you must strictly enforce thread safety rules in Rust.

- **Main Thread Requirement:** AppKit will crash if you attempt to create windows or poll events from a background thread. You should implement a runtime check inside `Window::new` to panic if it is not called on the main thread.
- **Struct Traits:** You must explicitly define your `Window` struct as `!Send` and `!Sync` (usually by including a `PhantomData<*mut ()>`) to prevent the Rust compiler from allowing the user to move the window pointer across threads.

This completes the architectural plan for a zero-dependency macOS windowing library.
