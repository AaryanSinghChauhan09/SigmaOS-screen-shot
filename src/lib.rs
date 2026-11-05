#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureMode { FullScreen, ActiveWindow, Region(u32, u32, u32, u32) }

#[derive(Debug)]
pub struct ScreenshotEngine {
    pub mode: CaptureMode,
}

impl ScreenshotEngine {
    pub fn new(mode: CaptureMode) -> Self { Self { mode } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_screenshot_engine_aaa() {
        // Arrange & Act
        let eng = ScreenshotEngine::new(CaptureMode::Region(0, 0, 800, 600));
        // Assert
        assert_eq!(eng.mode, CaptureMode::Region(0, 0, 800, 600));
    }
}
