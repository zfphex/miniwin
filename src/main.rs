use winmac::window::{FullscreenMode, Window, WindowStyle};

fn main() {
    // Logical sizing
    let width = 800;
    let height = 600;

    println!("Creating window...");
    let mut window = Window::new(
        "Stage 5/6 Live Native Resize Test",
        width as f64,
        height as f64,
        WindowStyle::Standard,
        FullscreenMode::None,
    );

    // Query DPI backing scale factor
    let scale = window.backing_scale_factor();
    println!("Window backing scale factor (Retina multiplier): {}", scale);

    // Calculate physical buffer size
    let physical_width = (width as f64 * scale) as usize;
    let physical_height = (height as f64 * scale) as usize;
    println!(
        "Physical buffer size: {}x{}",
        physical_width, physical_height
    );

    println!("Showing window...");
    window.make_key_and_order_front();

    println!(
        "Starting render loop. Move mouse, resize window, press keys, press Cmd+Q/Close to exit."
    );

    let mut pixels = vec![0u32; physical_width * physical_height];
    let mut frame_count = 0;
    let mut running = true;

    // We define our draw logic as a closure. It captures the local environment variables.
    // During Cocoa's blocking live-resize loop, this closure is executed synchronously
    // inside the delegate callbacks. During normal loop execution, it is called at the bottom.
    let mut draw = |win: &mut Window, w: usize, h: usize| {
        if pixels.len() != w * h {
            pixels.resize(w * h, 0);
        }

        // Render a moving color pattern scaled to the physical buffer size
        for y in 0..h {
            for x in 0..w {
                let r = ((x + frame_count) & 0xFF) as u32;
                let g = ((y + frame_count) & 0xFF) as u32;
                let b = (frame_count & 0xFF) as u32;
                pixels[y * w + x] = 0xFF000000 | (r << 16) | (g << 8) | b;
            }
        }

        win.update_buffer(&pixels, w, h);
        frame_count += 2;
    };

    while running {
        let events = window.poll_events(&mut draw);
        for event in events {
            println!("Event: {:?}", event);
            match event {
                winmac::event::Event::CloseRequested => {
                    println!("Close requested! Exiting loop...");
                    running = false;
                }
                _ => {}
            }
        }

        // Draw regular frames (when not modal resizing)
        let scale = window.backing_scale_factor();
        let (w, h) = window.content_size();
        let pw = (w * scale) as usize;
        let ph = (h * scale) as usize;
        draw(&mut window, pw, ph);

        window.wait_for_vsync();
    }

    println!("Clean exit.");
}
