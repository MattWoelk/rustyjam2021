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
        app.init_resource::<KeyActions>()
            .init_resource::<DebugKeyActions>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .label(GatherInput)
                    .with_system(set_keyboard_actions.label(GatherInput))
                    .with_system(set_debug_keyboard_actions.after(set_keyboard_actions)),
            );
    }
}

pub struct KeyActions {
    pub new_press: bool,
    pub char_stack: Vec<char>,
    pub keys_just_pressed: Vec<char>,
    pub space_pressed: bool,
    pub all_words: HashSet<String>,
    pub longest_word_option: Option<String>,
    pub all_collected_words: Vec<String>,
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
            all_collected_words: vec![],
        }
    }
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

// TODO: put this debug stuff in its own file/system/whatever
// TODO: disable this debug stuff when we're in --release

#[derive(Default)]
pub struct DebugKeyActions {
    pub boss_fight_started: bool,
}

fn set_debug_keyboard_actions(
    mut debug_action: ResMut<DebugKeyActions>,
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
) {
    debug_action.boss_fight_started = keyboard_input.just_pressed(KeyCode::F1);

    if debug_action.boss_fight_started {
        state.push(GameState::Boss).unwrap();
    }
}
