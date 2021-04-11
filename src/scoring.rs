use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct Score {
    pub score: u32,
}

#[derive(Default)]
pub struct ScoringPlugin;

impl Plugin for ScoringPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score { score: 0 });
    }
}
