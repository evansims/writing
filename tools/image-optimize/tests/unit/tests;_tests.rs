//! Unit tests extracted from lib.rs

use image-optimize::*;
mod tests;

use anyhow::Result;
use common_config::load_config;
use common_errors::{WritingError, OptionValidationExt};
use image::{GenericImageView, DynamicImage};
use image::imageops::FilterType;
use std::fs;
use std::io::{Write, BufWriter};
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Supported output formats for image optimization
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Jpeg,
    #[cfg(feature = "webp")]
    WebP,
}
