//! Entity code for xen, handles children, and other aspects of a object.

pub mod primitive;

/// A 'physical' object in an experience
pub trait Entity {

    /// Initilize the object
    fn start(&mut self);

    /// Called as fast as possible in between frames
    fn update(&mut self);

    /// Called at a fixed rate of 90 FPS or the highest FPS possible
    fn fixed_update(&mut self);
}
