use bevy::{platform::collections::HashMap, prelude::*};

pub enum AnimationOption {
    None,
    NextFrame,
    GoToFrame(usize),
    SwitchToAnimation(String),
    PlaySound(String),
}

pub struct AnimationFrame {
    sprite_index: usize,
    delay_ms: u128,
    action: Vec<AnimationOption>,
}

impl AnimationFrame {
    pub fn new(sprite_index: usize, delay_ms: u128, action: Vec<AnimationOption>) -> Self {
        Self {
            sprite_index,
            delay_ms,
            action,
        }
    }
}

pub struct PerFrameAnimation {
    pub frames: Vec<AnimationFrame>,
}

impl PerFrameAnimation {
    pub fn new(frames: Vec<AnimationFrame>) -> Self {
        Self { frames }
    }
}

#[derive(Resource)]
pub struct Animations(HashMap<String, PerFrameAnimation>);

impl Animations {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_animation<S: ToString>(mut self, tag: S, animation: PerFrameAnimation) -> Self {
        self.0.insert(tag.to_string(), animation);
        self
    }
}
