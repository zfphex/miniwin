use miniwin::*;

fn main() {
    println!("Initializing Window...");
    // Unified window creation for both platforms
    let mut window = create_window(
        "Demo",
        0,
        0, // Default position
        800,
        600,
        WindowStyle::Standard,
    );

    let scale = window.scale_factor();
    let physical_width = (800.0 * scale) as usize;
    let physical_height = (600.0 * scale) as usize;

    println!("Demo Controls:");
    println!("  [C] - Cycle cursor icon (Arrow -> IBeam -> PointingHand -> Crosshair)");
    println!("  [V] - Toggle cursor visibility");
    println!("  [G] - Toggle cursor grab/lock");
    println!("  [P] - Copy 'Hello from miniwin!' to clipboard");
    println!("  [O] - Read and print clipboard contents");
    println!("  [Drop Files] - Drop files onto the window to print their paths");

    let mut pixels = vec![0u32; physical_width * physical_height];
    let mut frame_count = 0;
    let mut running = true;

    let mut cursor_index = 0;
    let cursor_icons = [
        CursorIcon::Arrow,
        CursorIcon::IBeam,
        CursorIcon::PointingHand,
        CursorIcon::Crosshair,
        CursorIcon::ResizeLeftRight,
        CursorIcon::ResizeUpDown,
    ];

    let mut cursor_visible = true;
    let mut cursor_grabbed = false;

    // Platform-agnostic drawing logic
    let draw = |pixels: &mut Vec<u32>, w: usize, h: usize, frame: &mut usize| {
        if pixels.len() != w * h {
            pixels.resize(w * h, 0);
        }
        for y in 0..h {
            for x in 0..w {
                let r = ((x + *frame) & 0xFF) as u32;
                let g = ((y + *frame) & 0xFF) as u32;
                let b = (*frame & 0xFF) as u32;
                pixels[y * w + x] = 0xFF000000 | (r << 16) | (g << 8) | b;
            }
        }
        *frame += 2;
    };

    while running {
        // Unified event polling and resize-rendering hook
        window.draw(|win| {
            let scale = win.scale_factor();
            let (w, h) = win.content_size();
            let pw = (w as f64 * scale) as usize;
            let ph = (h as f64 * scale) as usize;
            draw(&mut pixels, pw, ph, &mut frame_count);
            win.update_buffer(&pixels, pw, ph);
        });

        // Unified event retrieval
        let mut events = Vec::new();
        while let Some(evt) = window.event() {
            events.push(evt);
        }

        for event in events {
            match event {
                Event::Quit | Event::CloseRequested => {
                    running = false;
                }
                Event::ReceivedCharacter(c) => {
                    println!("Character Received: {:?}", c);

                    match c {
                        'c' | 'C' => {
                            cursor_index = (cursor_index + 1) % cursor_icons.len();
                            let icon = cursor_icons[cursor_index];
                            window.set_cursor_icon(icon);
                            println!("Set cursor shape to {:?}", icon);
                        }
                        'v' | 'V' => {
                            cursor_visible = !cursor_visible;
                            window.set_cursor_visible(cursor_visible);
                            println!("Set cursor visibility: {}", cursor_visible);
                        }
                        'g' | 'G' => {
                            cursor_grabbed = !cursor_grabbed;
                            window.set_cursor_grab(cursor_grabbed);
                            println!("Set cursor grab: {}", cursor_grabbed);
                        }
                        'p' | 'P' => {
                            let text = "Hello from miniwin Clipboard!";
                            window.set_clipboard_text(text);
                            println!("Copied text to clipboard: {:?}", text);
                        }
                        'o' | 'O' => {
                            if let Some(text) = window.get_clipboard_text() {
                                println!("Read text from clipboard: {:?}", text);
                            }
                        }
                        _ => {}
                    }
                }
                Event::DroppedFiles(files) => {
                    println!("Dropped Files:");
                    for file in files {
                        println!("  - {:?}", file);
                    }
                }
                _ => {}
            }
        }

        window.wait_for_vsync();
    }
}
