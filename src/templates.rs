//! All templates that are used to render the web pages of this service.

use askama::Template;

mod filters {
    //! Custom filters for [`askama`] templates.

    /// Convert an image URL into a source set with different DPI scaling and output the `src` and
    /// `srcset` attributes for an `<img>` element.
    ///
    /// The different images are expected to be located next to the original image and to have a
    /// suffix in the form `@<scaling>x`. For example, an input of `logo.png` will create an output
    /// as follows:
    ///
    /// ```text
    /// src="logo.png" srcset="logo.png, logo@1.5x.png 1.5x, logo@2x.png 2x, ..."
    /// ```
    pub fn srcset(base: &str) -> askama::Result<String> {
        Ok(if let Some(pos) = base.rfind('.') {
            format!(
                "src=\"{0}\" srcset=\"{0}, \
                {name}@1.5x{ext} 1.5x, \
                {name}@2x{ext} 2x, \
                {name}@3x{ext} 3x, \
                {name}@4x{ext} 4x\"",
                base,
                name = &base[..pos],
                ext = &base[pos..],
            )
        } else {
            base.to_owned()
        })
    }
}

/// Template for the index page.
#[derive(Template)]
#[template(path = "index.html")]
pub struct Index;

/// Template for the login page.
#[derive(Template)]
#[template(path = "login.html")]
pub struct Login {
    /// Optional flash message that's shown as an error.
    pub flash: Option<String>,
}
