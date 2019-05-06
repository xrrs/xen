//! Engine code for xen, handles event loop, XR, rendering
use std::error::Error;
use std::{thread, time};
// For multithreading
use std::sync::{Arc, Mutex};

use crate::entity::Entity;

pub mod render;

/// Simple VR Experience, Xen will handle rendering, component updates, and updating entities
pub trait SimpleExperience {
    /// Setup the scene
    fn start(&mut self);

}


/// Xen engine
struct Xen<Game> {
    /// Game object passed from the user
    game: Game,
    /// List of entities in a scene
    entities: Arc<Mutex<Vec<Box<Entity>>>>
}

impl Xen<Box<SimpleExperience>> {
    /// Start running the Experience
    fn run(mut self) -> Result<(), Box<dyn Error>> {
        // // Start
        // {
        //     // start game
        //     self.game.start();


        //     for entity in self.entities.get_mut().unwrap() {
        //         entity.start();
        //     }

        // }


        // // Update thread
        // let update_thread = thread::spawn(move || {
        //     loop {
        //         println!("xen: Update Tick");
        //         for entity in self.entities.get_mut().unwrap() {
        //             entity.update();
        //         }
        //     }
        // });

        Ok(())
    }

}
