use winmac::event_loop::EventLoop;
use winmac::window::{Window, WindowStyle, FullscreenMode};

fn main() {
    println!("Initializing EventLoop...");
    let event_loop = EventLoop::new();
    
    let width = 800;
    let height = 600;
    
    println!("Creating window...");
    let mut window = Window::new(
        "Stage 4 Graphics Buffer Test",
        width as f64,
        height as f64,
        WindowStyle::Standard,
        FullscreenMode::None,
    );
    
    println!("Showing window...");
    window.make_key_and_order_front();
    
    println!("Starting render loop. Close the window or press Cmd+Q to exit.");
    
    let mut pixels = vec![0u32; width * height];
    let mut frame_count = 0;
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
        
        // Render a moving color pattern
        for y in 0..height {
            for x in 0..width {
                let r = ((x + frame_count) & 0xFF) as u32;
                let g = ((y + frame_count) & 0xFF) as u32;
                let b = (frame_count & 0xFF) as u32;
                // standard ARGB/XRGB format: 0xFFRRGGBB
                pixels[y * width + x] = 0xFF000000 | (r << 16) | (g << 8) | b;
            }
        }
        
        window.update_buffer(&pixels, width, height);
        frame_count += 2;
        
        std::thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS
    }
    
    println!("Clean exit.");
}
