//! SigmaOS Screenshot Tool
//! Screenshot tool with multiple modes (fullscreen, window, region, timed)

pub mod screenshot;

pub use screenshot::ScreenshotTool;
pub use screenshot::ScreenshotConfig;
pub use screenshot::ScreenshotMode;
pub use screenshot::ImageFormat;
