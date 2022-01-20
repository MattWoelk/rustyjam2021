use crate::game_plugin::enemy::Enemy;
use crate::game_plugin::GameState;
use crate::game_plugin::SystemLabels::GatherInput;
use bevy::prelude::*;
use std::collections::HashSet;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing)
                .label(GatherInput)
                .with_system(set_movement_actions.label(GatherInput))
                .with_system(set_shoot_actions.label(GatherInput)),
        );

        app.init_resource::<KeyActions>().add_system_set(
            SystemSet::on_update(GameState::Playing)
                .label(GatherInput)
                .with_system(set_keyboard_actions.label(GatherInput)),
        );
    }
}

#[derive(Default)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
    pub player_shoot: bool,
    pub player_switch_weapon: bool,
}

pub struct KeyActions {
    pub new_press: bool,
    pub char_stack: Vec<char>,
    pub keys_just_pressed: Vec<char>,
    pub space_pressed: bool,
    pub all_words: HashSet<String>,
    pub longest_word_option: Option<String>,
}

impl Default for KeyActions {
    fn default() -> Self {
        let all_words = include_str!("../../assets/words_alpha.txt")
            .lines()
            .map(|l| l.to_string())
            .collect::<HashSet<_>>();

        KeyActions {
            new_press: false,
            char_stack: vec![],
            keys_just_pressed: vec![],
            space_pressed: false,
            all_words,
            longest_word_option: None,
        }
    }
}

fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    if GameControl::Up.just_released(&keyboard_input)
        || GameControl::Up.pressed(&keyboard_input)
        || GameControl::Left.just_released(&keyboard_input)
        || GameControl::Left.pressed(&keyboard_input)
        || GameControl::Down.just_released(&keyboard_input)
        || GameControl::Down.pressed(&keyboard_input)
        || GameControl::Right.just_released(&keyboard_input)
        || GameControl::Right.pressed(&keyboard_input)
    {
        let mut player_movement = Vec2::ZERO;

        if GameControl::Up.just_released(&keyboard_input)
            || GameControl::Down.just_released(&keyboard_input)
        {
            if GameControl::Up.pressed(&keyboard_input) {
                player_movement.y = 1.;
            } else if GameControl::Down.pressed(&keyboard_input) {
                player_movement.y = -1.;
            } else {
                player_movement.y = 0.;
            }
        } else if GameControl::Up.just_pressed(&keyboard_input) {
            player_movement.y = 1.;
        } else if GameControl::Down.just_pressed(&keyboard_input) {
            player_movement.y = -1.;
        } else {
            player_movement.y = actions.player_movement.unwrap_or(Vec2::ZERO).y;
        }

        if GameControl::Right.just_released(&keyboard_input)
            || GameControl::Left.just_released(&keyboard_input)
        {
            if GameControl::Right.pressed(&keyboard_input) {
                player_movement.x = 1.;
            } else if GameControl::Left.pressed(&keyboard_input) {
                player_movement.x = -1.;
            } else {
                player_movement.x = 0.;
            }
        } else if GameControl::Right.just_pressed(&keyboard_input) {
            player_movement.x = 1.;
        } else if GameControl::Left.just_pressed(&keyboard_input) {
            player_movement.x = -1.;
        } else {
            player_movement.x = actions.player_movement.unwrap_or(Vec2::ZERO).x;
        }

        if player_movement != Vec2::ZERO {
            player_movement = player_movement.normalize();
            actions.player_movement = Some(player_movement);
        }
    } else {
        actions.player_movement = None;
    }
}

enum GameControl {
    Up,
    Down,
    Left,
    Right,
}

impl GameControl {
    fn just_released(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.just_released(KeyCode::W)
                    || keyboard_input.just_released(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.just_released(KeyCode::S)
                    || keyboard_input.just_released(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.just_released(KeyCode::A)
                    || keyboard_input.just_released(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.just_released(KeyCode::D)
                    || keyboard_input.just_released(KeyCode::Right)
            }
        }
    }

    fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right)
            }
        }
    }

    fn just_pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.just_pressed(KeyCode::W) || keyboard_input.just_pressed(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.just_pressed(KeyCode::S)
                    || keyboard_input.just_pressed(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.just_pressed(KeyCode::A)
                    || keyboard_input.just_pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.just_pressed(KeyCode::D)
                    || keyboard_input.just_pressed(KeyCode::Right)
            }
        }
    }
}

fn set_shoot_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    actions.player_shoot = keyboard_input.pressed(KeyCode::Space);
    actions.player_switch_weapon = keyboard_input.just_pressed(KeyCode::LShift)
        || keyboard_input.just_pressed(KeyCode::RShift);
}

fn set_keyboard_actions(
    mut action: ResMut<KeyActions>,
    keyboard_input: Res<Input<KeyCode>>,
    enemies: Query<&Enemy>,
) {
    let new_chars: Vec<char> = keyboard_input
        .get_just_pressed()
        .filter_map(|code: &bevy::prelude::KeyCode| match code {
            KeyCode::A => Some('a'),
            KeyCode::B => Some('b'),
            KeyCode::C => Some('c'),
            KeyCode::D => Some('d'),
            KeyCode::E => Some('e'),
            KeyCode::F => Some('f'),
            KeyCode::G => Some('g'),
            KeyCode::H => Some('h'),
            KeyCode::I => Some('i'),
            KeyCode::J => Some('j'),
            KeyCode::K => Some('k'),
            KeyCode::L => Some('l'),
            KeyCode::M => Some('m'),
            KeyCode::N => Some('n'),
            KeyCode::O => Some('o'),
            KeyCode::P => Some('p'),
            KeyCode::Q => Some('q'),
            KeyCode::R => Some('r'),
            KeyCode::S => Some('s'),
            KeyCode::T => Some('t'),
            KeyCode::U => Some('u'),
            KeyCode::V => Some('v'),
            KeyCode::W => Some('w'),
            KeyCode::X => Some('x'),
            KeyCode::Y => Some('y'),
            KeyCode::Z => Some('z'),
            _ => None,
        })
        .collect();

    for &c in new_chars.iter() {
        if enemies.iter().any(|e| e.letter == c) {
            action.char_stack.push(c);
        }
    }

    action.new_press = !new_chars.is_empty();
    action.keys_just_pressed = new_chars;
    action.space_pressed = keyboard_input.just_pressed(KeyCode::Space);

    // TODO: how often do we need to run this?
    //if action.new_press || action.space_pressed {
    action.longest_word_option = find_ending_word(
        &action.char_stack.iter().collect::<String>(),
        &action.all_words,
    );
    //}
}

fn find_ending_word(text: &str, all_words: &HashSet<String>) -> Option<String> {
    if text.len() < 3 {
        return None;
    }

    for length in (3..=text.len()).rev() {
        let substring: String = text.chars().skip(text.len() - length).collect();

        if all_words.contains(&substring) {
            return Some(substring);
        }
    }

    None
}
