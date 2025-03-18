//! Unit tests extracted from lib.rs

use image-build::*;
mod tests;

use anyhow::{Context, Result};
use common_config::load_config;
use common_models::{Config, ImageNaming};
use image::{GenericImageView, ImageFormat};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
#[cfg(feature = "avif")]
use std::process::Command;

/// Options for building images
#[derive(Debug, Clone)]
pub struct BuildImagesOptions {
    /// Output directory for optimized images
    pub output_dir: PathBuf,
    /// Source directory containing content
    pub source_dir: PathBuf,
    /// Source image filename
    pub source_filename: String,
    /// Specific article to process (optional)
    pub article: Option<String>,
    /// Specific topic to process (optional)
    pub topic: Option<String>,
}
