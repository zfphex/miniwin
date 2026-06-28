use winmac::event_loop::EventLoop;
use winmac::window::{Window, WindowStyle, FullscreenMode};

fn main() {
    println!("Initializing EventLoop...");
    let event_loop = EventLoop::new();
    
    println!("Creating window...");
    let window = Window::new(
        "Stage 3 Event Loop Test",
        800.0,
        600.0,
        WindowStyle::Standard,
        FullscreenMode::None,
    );
    
    println!("Showing window...");
    window.make_key_and_order_front();
    
    println!("Starting event loop. Close the window or press Cmd+Q to exit.");
    
    let mut running = true;
    while running {
        let events = event_loop.poll_events();
        for event in events {
            println!("Received Event: {:?}", event);
            if event == winmac::event::Event::CloseRequested {
                println!("Close requested! Exiting loop...");
                running = false;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS
    }
    
    println!("Clean exit.");
}
