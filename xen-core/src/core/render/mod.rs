//! Rendering code for xen

/// This trait allows for an Entity to be drawn
pub trait Drawable {

    // TODO: Figure out return type
    /// Draws an entity
    fn draw(&self);

}


mod window {
    /// Window struct
    struct Window {
        width: usize,
        height: usize,
        name: String
    }

    impl Window {
        /// Create a new window
        pub fn new(name: String, width: usize, height: usize) -> Self {
            Window {
                name,
                width,
                height,
            }
        }

        /// Update with window
        pub fn update() {

        }
    }

}
