//! Module template for implementing standard module structure.
//!
//! This file serves as a template for the public API of a module.
//! Replace this documentation with a description of your module's purpose.

mod impl_;
mod models;
mod errors;

#[cfg(test)]
mod tests;

// Re-export public types
pub use errors::TemplateError;
pub use models::{TemplateModel, TemplateConfig};

// Re-export public functions from implementation
pub use impl_::{
    create_template,
    update_template,
    delete_template,
    get_template,
    list_templates,
};

// Constants that should be available to users of this module
pub const DEFAULT_TEMPLATE_PATH: &str = "templates"; 