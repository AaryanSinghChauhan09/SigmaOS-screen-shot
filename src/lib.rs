//! SigmaOS-screen-shot — screenshot capture & annotation engine
//! (Omarchy `omasnap` parity).
//!
//! Models capture modes (full screen, active window, region), a
//! post-capture annotation layer (arrows, text, blur boxes, highlight),
//! output destinations (file, clipboard), and timestamped naming.
//! Pixel capture itself is delegated to the compositor/portal.

/// Where the finished shot goes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputTarget {
    /// Save to `dir` with generated filename.
    File { dir: String },
    /// Copy to clipboard only.
    Clipboard,
    /// Both file and clipboard.
    Both { dir: String },
}

/// Capture source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureMode {
    FullScreen,
    ActiveWindow,
    /// x, y, width, height in pixels.
    Region(i32, i32, u32, u32),
}

/// Annotation primitives drawn on top of the capture.
#[derive(Debug, Clone, PartialEq)]
pub enum Annotation {
    /// Arrow from (x1,y1) to (x2,y2).
    Arrow { x1: i32, y1: i32, x2: i32, y2: i32 },
    /// Text label at position.
    Text { x: i32, y: i32, content: String },
    /// Blur/redact rectangle.
    Blur { x: i32, y: i32, w: u32, h: u32 },
    /// Highlight rectangle (translucent).
    Highlight { x: i32, y: i32, w: u32, h: u32 },
}

/// A finished screenshot record.
#[derive(Debug, Clone)]
pub struct Screenshot {
    pub width: u32,
    pub height: u32,
    pub annotations: Vec<Annotation>,
    pub captured_unix: i64,
}

/// Filename: `sigmaos-YYYYMMDD-HHMMSS.png` style (pure function of time).
pub fn filename_for(unix_secs: i64) -> String {
    // compact civil timestamp via days-from-epoch math
    let days = unix_secs.div_euclid(86_400);
    let secs = unix_secs.rem_euclid(86_400);
    let (y, m, d) = civil_from_days(days);
    format!(
        "sigmaos-{:04}{:02}{:02}-{:02}{:02}{:02}.png",
        y,
        m,
        d,
        secs / 3600,
        (secs % 3600) / 60,
        secs % 60
    )
}

/// Howard Hinnant's civil-from-days algorithm.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m as u32, d as u32)
}

#[derive(Debug)]
pub struct ScreenshotEngine {
    pub mode: CaptureMode,
    annotations: Vec<Annotation>,
}

impl ScreenshotEngine {
    pub fn new(mode: CaptureMode) -> Self {
        Self {
            mode,
            annotations: Vec::new(),
        }
    }

    pub fn annotate(&mut self, a: Annotation) {
        self.annotations.push(a);
    }

    pub fn clear_annotations(&mut self) {
        self.annotations.clear();
    }

    /// Resolve output path for a file target at `unix_secs`.
    /// Returns (path, to_clipboard).
    pub fn resolve_output(&self, target: &OutputTarget, unix_secs: i64) -> (Option<String>, bool) {
        let name = filename_for(unix_secs);
        match target {
            OutputTarget::File { dir } => (Some(format!("{dir}/{name}")), false),
            OutputTarget::Clipboard => (None, true),
            OutputTarget::Both { dir } => (Some(format!("{dir}/{name}")), true),
        }
    }

    /// Produce the capture record. Dimensions come from the mode or
    /// caller-supplied actual capture size.
    pub fn capture(&self, actual_w: u32, actual_h: u32, unix_secs: i64) -> Screenshot {
        Screenshot {
            width: actual_w,
            height: actual_h,
            annotations: self.annotations.clone(),
            captured_unix: unix_secs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screenshot_engine_aaa() {
        let eng = ScreenshotEngine::new(CaptureMode::Region(0, 0, 800, 600));
        assert_eq!(eng.mode, CaptureMode::Region(0, 0, 800, 600));
    }

    #[test]
    fn filename_is_timestamped() {
        // 2024-01-01T12:54:56Z = 1704113696
        let name = filename_for(1_704_113_696);
        assert_eq!(name, "sigmaos-20240101-125456.png");
    }

    #[test]
    fn output_targets() {
        let eng = ScreenshotEngine::new(CaptureMode::FullScreen);
        let (path, clip) = eng.resolve_output(
            &OutputTarget::Both { dir: "/tmp".into() },
            1_704_113_696,
        );
        assert_eq!(path.as_deref(), Some("/tmp/sigmaos-20240101-125456.png"));
        assert!(clip);
        let (p2, c2) = eng.resolve_output(&OutputTarget::Clipboard, 0);
        assert!(p2.is_none() && c2);
    }

    #[test]
    fn annotations_attach_to_capture() {
        let mut eng = ScreenshotEngine::new(CaptureMode::ActiveWindow);
        eng.annotate(Annotation::Arrow { x1: 0, y1: 0, x2: 10, y2: 10 });
        eng.annotate(Annotation::Blur { x: 5, y: 5, w: 50, h: 50 });
        let shot = eng.capture(1920, 1080, 0);
        assert_eq!(shot.annotations.len(), 2);
        assert_eq!(shot.width, 1920);
    }
}
