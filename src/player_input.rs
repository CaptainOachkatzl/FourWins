use bevy::prelude::*;
use std::collections::HashSet;

pub struct PlayerInput {
    checked_keys: Box<[KeyCode]>,
    keystates: HashSet<KeyCode>,
    previous_keystates: HashSet<KeyCode>,
}

impl PlayerInput {
    pub fn new(keys: Box<[KeyCode]>) -> PlayerInput {
        return PlayerInput { 
            checked_keys: keys,
            keystates: HashSet::new(),
            previous_keystates: HashSet::new(),
        }
    }

    pub fn update(&mut self, keyboard_input: Res<Input<KeyCode>>) {
        self.previous_keystates = self.keystates.clone();
        self.keystates = self.generate_hashset_from_input(keyboard_input);
    }

    fn generate_hashset_from_input(&self, keyboard_input: Res<Input<KeyCode>>) -> HashSet<KeyCode> {
        let mut set: HashSet<KeyCode> = HashSet::new();
        for i in 0..self.checked_keys.len() {
            if keyboard_input.pressed(self.checked_keys[i]) {
                set.insert(self.checked_keys[i]);
            }
        }

        return set;
    }

    pub fn just_pressed(&self, key: KeyCode) -> bool {
        return !self.previous_keystates.contains(&key) && self.keystates.contains(&key);
    }
}