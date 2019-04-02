use std::collections::HashMap;

use amethyst::ecs::world::Index;

use super::component_prelude::*;

pub enum State {
    Enter,
    Leave,
    Steady,
    None,
}

pub struct Data {
    pub state:                State,
    set_collision_this_frame: bool,
}

impl Data {
    pub fn should_remove(&self) -> bool {
        if let State::None = self.state {
            true
        } else {
            false
        }
    }

    pub fn unset(&mut self) {
        // Set state of NOT colliding entity to `Leave` if it was previously
        // in collision and not `Leave`, otherwise remove the entity from the HashMap.
        match self.state {
            State::Leave => self.state = State::None, // Stage for removal
            _ => self.state = State::Leave,
        }
    }
}

/// Entities with collision perform collision detection against
/// all other collision entities, every frame.
/// Depending on if they are in collision, data will be set.
pub struct Collision {
    pub collisions: HashMap<Index, Data>,
}

impl Collision {
    pub fn new() -> Self {
        Self {
            collisions: HashMap::new(),
        }
    }

    pub fn in_collision(&self) -> bool {
        !self.collisions.is_empty()
    }

    pub fn collision_with(&self, entity_id: Index) -> Option<&Data> {
        self.collisions.get(&entity_id)
    }

    pub fn in_collision_with(&self, entity_id: Index) -> bool {
        if let Some(data) = self.collisions.get(&entity_id) {
            match data.state {
                State::Leave => false,
                _ => true,
            }
        } else {
            false
        }
    }

    /// Is called when an entity is colliding with this entity
    pub fn set_collision_with(&mut self, entity_id: Index) {
        if let Some(data) = self.collisions.get_mut(&entity_id) {
            // Set state of colliding entity to ...
            data.state = match data.state {
                // `Enter` if it was `Leave` previously
                State::Leave => State::Enter,
                // `Steady` if it was any other state previously
                _ => State::Steady,
            };
            data.set_collision_this_frame = true;
        } else {
            self.collisions.insert(entity_id, Data {
                state:                    State::Enter,
                set_collision_this_frame: true,
            });
        }
    }

    pub fn update(&mut self) {
        let mut to_remove = Vec::new();
        for (&id, collision) in self.collisions.iter_mut() {
            if collision.set_collision_this_frame {
                // Entity collision data was modified this frame, stage for deletion next frame
                collision.set_collision_this_frame = false;
            } else {
                // Entity collision data was NOT modified this frame, set State to `Leave` or remove
                // self.unset_collision_with_entry((id, collision));
                collision.unset();
                if collision.should_remove() {
                    to_remove.push(id);
                }
            }
        }
        for id in to_remove {
            self.collisions.remove(&id);
        }
    }
}

impl Component for Collision {
    type Storage = DenseVecStorage<Self>;
}
