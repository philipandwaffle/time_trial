use bevy::prelude::*;
pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputStates>().add_system(process_input);
    }
}

#[derive(Resource)]
pub struct InputStates {
    pub move_up: bool,
    pub move_left: bool,
    pub move_down: bool,
    pub move_right: bool,
    pub next_level: bool,
    pub prev_level: bool,
}
impl Default for InputStates {
    fn default() -> Self {
        return Self {
            move_up: false,
            move_left: false,
            move_down: false,
            move_right: false,
            next_level: false,
            prev_level: false,
        };
    }
}

fn process_input(input: Res<Input<KeyCode>>, mut state: ResMut<InputStates>) {
    state.move_up = input.pressed(KeyCode::W);
    state.move_left = input.pressed(KeyCode::A);
    state.move_down = input.pressed(KeyCode::S);
    state.move_right = input.pressed(KeyCode::D);

    state.next_level = input.pressed(KeyCode::Left);
    state.prev_level = input.pressed(KeyCode::Right);
}
