//! a Component is a bunch of code that is attached/run by an entity. They can be reused no matter
//! the entity 
use crate::entity::Entity;

/// This marks a struct as a component. Components are like entities but dont have children and
/// other 'physical' aspects. They can store their own data. But should works with all entities
pub trait Component {
    /// Called when a component is created
    fn start_component(&mut self);

    /// Update loop on a component, mutates state
    fn update_component(&mut self, parent: &mut Entity);

}
