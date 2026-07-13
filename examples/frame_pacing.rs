//! Real-window frame-pacing comparison: legacy AlwaysSignal vs current ArmGated.
//!
//! Opens an NSWindow, drives CVDisplayLink waits under both strategies, idles
//! while the display keeps ticking, then prints wall-clock wait samples.
//!
//!   cargo run --example frame_pacing

use miniwin::macos::vsync::{VsyncMode, VsyncTracker};
use miniwin::{create_window, PlatformWindow, WindowStyle};
use std::process::ExitCode;
use std::time::{Duration, Instant};

struct PaceReport {
    label: &'static str,
    mode: VsyncMode,
    phase: &'static str,
    samples_ms: Vec<f64>,
}

impl PaceReport {
    fn from_samples(
        label: &'static str,
        mode: VsyncMode,
        phase: &'static str,
        samples: &[Duration],
    ) -> Self {
        Self {
            label,
            mode,
            phase,
            samples_ms: samples.iter().map(|d| d.as_secs_f64() * 1000.0).collect(),
        }
    }

    fn min_ms(&self) -> f64 {
        self.samples_ms.iter().copied().fold(f64::INFINITY, f64::min)
    }

    fn max_ms(&self) -> f64 {
        self.samples_ms
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max)
    }

    fn mean_ms(&self) -> f64 {
        self.samples_ms.iter().sum::<f64>() / self.samples_ms.len() as f64
    }

    fn total_ms(&self) -> f64 {
        self.samples_ms.iter().sum()
    }

    fn instant_count(&self) -> usize {
        self.samples_ms.iter().filter(|&&ms| ms < 1.0).count()
    }

    fn paced_count(&self) -> usize {
        self.samples_ms.iter().filter(|&&ms| ms >= 2.0).count()
    }

    fn print(&self) {
        println!(
            "\n=== {} | {:?} | {} (n={}) ===",
            self.label,
            self.mode,
            self.phase,
            self.samples_ms.len()
        );
        println!(
            "  min={:.3}ms  mean={:.3}ms  max={:.3}ms  total={:.3}ms",
            self.min_ms(),
            self.mean_ms(),
            self.max_ms(),
            self.total_ms()
        );
        println!(
            "  instant(<1ms)={}  paced(>=2ms)={}",
            self.instant_count(),
            self.paced_count()
        );
        print!("  samples_ms:");
        for ms in &self.samples_ms {
            print!(" {:.2}", ms);
        }
        println!();
    }
}

fn paint_frame(window: &mut impl PlatformWindow, color: u32) {
    window.draw(|win| {
        let pixels = win.framebuffer();
        for p in pixels.iter_mut() {
            *p = color;
        }
        win.present();
    });
}

fn pump_events(window: &mut impl PlatformWindow) {
    window.draw(|_| {});
}

fn measure_waits(tracker: &VsyncTracker, count: usize) -> Vec<Duration> {
    let mut samples = Vec::with_capacity(count);
    for _ in 0..count {
        let t = Instant::now();
        tracker.wait_for_vsync();
        samples.push(t.elapsed());
    }
    samples
}

fn main() -> ExitCode {
    println!("Opening real window for frame-pacing comparison...");

    let mut window = create_window(
        "miniwin frame pacing",
        None,
        640,
        480,
        false,
        WindowStyle::Standard,
    );

    // Show the window and process initial expose/resize traffic.
    for i in 0..10 {
        paint_frame(&mut *window, 0xFF202020 + i * 0x010101);
        window.wait_for_vsync();
    }

    const WARMUP: usize = 8;
    const SAMPLES: usize = 12;
    const IDLE: Duration = Duration::from_millis(250);

    let scenarios = [
        ("previous", VsyncMode::AlwaysSignal),
        ("current", VsyncMode::ArmGated),
    ];

    let mut reports = Vec::new();

    for (label, mode) in scenarios {
        let tracker = VsyncTracker::with_mode(mode);
        println!("\n--- scenario {label} ({mode:?}) ---");

        for i in 0..WARMUP {
            paint_frame(&mut *window, 0xFF304050 + (i as u32) * 0x080808);
            tracker.wait_for_vsync();
        }

        let steady = measure_waits(&tracker, SAMPLES);
        paint_frame(
            &mut *window,
            match mode {
                VsyncMode::AlwaysSignal => 0xFF803030,
                VsyncMode::ArmGated => 0xFF308030,
            },
        );
        reports.push(PaceReport::from_samples(
            label,
            mode,
            "steady (continuous waits)",
            &steady,
        ));

        // Idle while the display link keeps firing. Previous banks permits;
        // current must not.
        let idle_start = Instant::now();
        while idle_start.elapsed() < IDLE {
            pump_events(&mut *window);
            std::thread::sleep(Duration::from_millis(5));
        }

        let after_idle = measure_waits(&tracker, SAMPLES);
        paint_frame(
            &mut *window,
            match mode {
                VsyncMode::AlwaysSignal => 0xFFA04040,
                VsyncMode::ArmGated => 0xFF40A040,
            },
        );
        reports.push(PaceReport::from_samples(
            label,
            mode,
            "after 250ms idle",
            &after_idle,
        ));
    }

    for report in &reports {
        report.print();
    }

    let prev_idle = reports
        .iter()
        .find(|r| r.label == "previous" && r.phase.starts_with("after"))
        .unwrap();
    let curr_idle = reports
        .iter()
        .find(|r| r.label == "current" && r.phase.starts_with("after"))
        .unwrap();
    let curr_steady = reports
        .iter()
        .find(|r| r.label == "current" && r.phase.starts_with("steady"))
        .unwrap();

    let mut failed = false;

    if prev_idle.instant_count() < SAMPLES / 2 {
        eprintln!(
            "FAIL: expected legacy AlwaysSignal to burst after idle; instant={} samples={:?}",
            prev_idle.instant_count(),
            prev_idle.samples_ms
        );
        failed = true;
    }
    if prev_idle.total_ms() >= 20.0 {
        eprintln!(
            "FAIL: expected legacy after-idle total << paced frames, total_ms={:.3}",
            prev_idle.total_ms()
        );
        failed = true;
    }

    if curr_idle.instant_count() > 1 {
        eprintln!(
            "FAIL: current ArmGated burst after idle; instant={} samples={:?}",
            curr_idle.instant_count(),
            curr_idle.samples_ms
        );
        failed = true;
    }
    if curr_idle.paced_count() < SAMPLES - 1 {
        eprintln!(
            "FAIL: current ArmGated not paced after idle; paced={} samples={:?}",
            curr_idle.paced_count(),
            curr_idle.samples_ms
        );
        failed = true;
    }
    if curr_idle.total_ms() < 10.0 {
        eprintln!(
            "FAIL: current after-idle total too short (backlog?): {:.3}ms samples={:?}",
            curr_idle.total_ms(),
            curr_idle.samples_ms
        );
        failed = true;
    }

    if curr_steady.paced_count() < SAMPLES - 1 {
        eprintln!(
            "FAIL: current steady-state not paced; samples={:?}",
            curr_steady.samples_ms
        );
        failed = true;
    }

    if curr_idle.total_ms() <= prev_idle.total_ms() * 5.0 {
        eprintln!(
            "FAIL: expected current after-idle total ({:.3}ms) >> previous ({:.3}ms)",
            curr_idle.total_ms(),
            prev_idle.total_ms()
        );
        failed = true;
    }

    window.close();

    if failed {
        eprintln!("\nframe pacing comparison FAILED");
        ExitCode::FAILURE
    } else {
        println!("\nframe pacing comparison PASSED");
        println!(
            "previous after-idle total {:.3}ms (burst) vs current {:.3}ms (paced)",
            prev_idle.total_ms(),
            curr_idle.total_ms()
        );
        ExitCode::SUCCESS
    }
}
