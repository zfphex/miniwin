use winmac::event_loop::EventLoop;
use winmac::window::{Window, WindowStyle, FullscreenMode};

fn main() {
    println!("Initializing EventLoop...");
    let event_loop = EventLoop::new();
    
    // Logical sizing
    let width = 800;
    let height = 600;
    
    println!("Creating window...");
    let mut window = Window::new(
        "Stage 5 Retina & Coordinate Test",
        width as f64,
        height as f64,
        WindowStyle::Standard,
        FullscreenMode::None,
    );
    
    // Query DPI backing scale factor
    let scale = window.backing_scale_factor();
    println!("Window backing scale factor (Retina multiplier): {}", scale);
    
    // Calculate physical buffer size
    let mut physical_width = (width as f64 * scale) as usize;
    let mut physical_height = (height as f64 * scale) as usize;
    println!("Physical buffer size: {}x{}", physical_width, physical_height);
    
    println!("Showing window...");
    window.make_key_and_order_front();
    
    println!("Starting render loop. Move mouse, resize window, press keys, press Cmd+Q/Close to exit.");
    
    let mut pixels = vec![0u32; physical_width * physical_height];
    let mut frame_count = 0;
    let mut running = true;
    
    while running {
        let events = event_loop.poll_events();
        for event in events {
            println!("Event: {:?}", event);
            match event {
                winmac::event::Event::CloseRequested => {
                    println!("Close requested! Exiting loop...");
                    running = false;
                }
                winmac::event::Event::Resized { width: _w, height: _h, physical_width: pw, physical_height: ph } => {
                    physical_width = pw;
                    physical_height = ph;
                    pixels.resize(physical_width * physical_height, 0);
                    println!("Buffer resized to physical dimensions: {}x{}", physical_width, physical_height);
                }
                _ => {}
            }
        }
        
        // Render a moving color pattern scaled to the physical buffer size
        for y in 0..physical_height {
            for x in 0..physical_width {
                let r = ((x + frame_count) & 0xFF) as u32;
                let g = ((y + frame_count) & 0xFF) as u32;
                let b = (frame_count & 0xFF) as u32;
                pixels[y * physical_width + x] = 0xFF000000 | (r << 16) | (g << 8) | b;
            }
        }
        
        window.update_buffer(&pixels, physical_width, physical_height);
        frame_count += 2;
        
        std::thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS
    }
    
    println!("Clean exit.");
}
