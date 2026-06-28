use winmac::event::Event;
use winmac::window::{CursorIcon, FullscreenMode, Window, WindowStyle};

fn main() {
    println!("Initializing Window...");
    let mut window = Window::new(
        "Stage 7 Feature Demo",
        800.0,
        600.0,
        WindowStyle::Standard,
        FullscreenMode::None,
    );

    let scale = window.backing_scale_factor();
    let physical_width = (800.0 * scale) as usize;
    let physical_height = (600.0 * scale) as usize;

    window.make_key_and_order_front();

    println!("Demo Controls:");
    println!("  [C] - Cycle cursor icon (Arrow -> IBeam -> PointingHand -> Crosshair)");
    println!("  [V] - Toggle cursor visibility");
    println!("  [G] - Toggle cursor grab/lock");
    println!("  [P] - Copy 'Hello from winmac!' to clipboard");
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

    let mut draw = |win: &mut Window, w: usize, h: usize| {
        if pixels.len() != w * h {
            pixels.resize(w * h, 0);
        }
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
            match event {
                Event::CloseRequested => {
                    running = false;
                }
                Event::ReceivedCharacter(c) => {
                    println!("Character Received: {:?}", c);

                    // Handle control keys
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
                            let text = "Hello from winmac Clipboard!";
                            window.set_clipboard_text(text);
                            println!("Copied text to clipboard: {:?}", text);
                        }
                        'o' | 'O' => {
                            let text = window.get_clipboard_text();
                            println!("Read text from clipboard: {:?}", text);
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
                Event::Resized { .. } => {}
                _ => {}
            }
        }

        let scale = window.backing_scale_factor();
        let (w, h) = window.content_size();
        let pw = (w * scale) as usize;
        let ph = (h * scale) as usize;
        draw(&mut window, pw, ph);

        window.wait_for_vsync();
    }
}
