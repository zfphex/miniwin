use winmac::event_loop::EventLoop;
use winmac::window::{FullscreenMode, Window, WindowStyle};

fn main() {
    println!("Initializing EventLoop...");
    let _el = EventLoop::new();

    println!("Creating window...");
    let window = Window::new(
        "Stage 2 Test Window",
        800.0,
        600.0,
        WindowStyle::Standard,
        FullscreenMode::None,
    );

    println!("Showing window...");
    window.make_key_and_order_front();

    println!("Window initialized successfully! Sleeping for 2 seconds...");
    std::thread::sleep(std::time::Duration::from_secs(2));
}
