use anyhow::{Context, Result};
use heigtbitult::{bindings::types::KeyBinding, keyboard::*};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;

fn key_name_to_value(key_name: &str) -> Result<u8> {
    if let Some(hex_str) = key_name
        .strip_prefix("keycode(")
        .and_then(|s| s.strip_suffix(")"))
    {
        // Convertir en u32 d'abord pour vérifier la taille
        let value = u32::from_str_radix(hex_str, 16)
            .with_context(|| format!("Invalid hexadecimal format: {}. Expected format: keycode(XX) where XX is a hexadecimal number", hex_str))?;

        // Vérifier si le nombre peut tenir dans un u8
        if value > u8::MAX as u32 {
            return Err(anyhow::anyhow!(
                "Keycode value {} is too large for u8 (max: {})",
                value,
                u8::MAX
            ));
        }

        return Ok(value as u8);
    }

    match key_name {
        // Touches alphabétiques
        "A" => Ok(KEY_A),
        "B" => Ok(KEY_B),
        "C" => Ok(KEY_C),
        "D" => Ok(KEY_D),
        "E" => Ok(KEY_E),
        "F" => Ok(KEY_F),
        "G" => Ok(KEY_G),
        "H" => Ok(KEY_H),
        "I" => Ok(KEY_I),
        "J" => Ok(KEY_J),
        "K" => Ok(KEY_K),
        "L" => Ok(KEY_L),
        "M" => Ok(KEY_M),
        "N" => Ok(KEY_N),
        "O" => Ok(KEY_O),
        "P" => Ok(KEY_P),
        "Q" => Ok(KEY_Q),
        "R" => Ok(KEY_R),
        "S" => Ok(KEY_S),
        "T" => Ok(KEY_T),
        "U" => Ok(KEY_U),
        "V" => Ok(KEY_V),
        "W" => Ok(KEY_W),
        "X" => Ok(KEY_X),
        "Y" => Ok(KEY_Y),
        "Z" => Ok(KEY_Z),

        // Touches numériques
        "0" => Ok(KEY_0),
        "1" => Ok(KEY_1),
        "2" => Ok(KEY_2),
        "3" => Ok(KEY_3),
        "4" => Ok(KEY_4),
        "5" => Ok(KEY_5),
        "6" => Ok(KEY_6),
        "7" => Ok(KEY_7),
        "8" => Ok(KEY_8),
        "9" => Ok(KEY_9),

        // Touches de fonction
        "F1" => Ok(KEY_F1),
        "F2" => Ok(KEY_F2),
        "F3" => Ok(KEY_F3),
        "F4" => Ok(KEY_F4),
        "F5" => Ok(KEY_F5),
        "F6" => Ok(KEY_F6),
        "F7" => Ok(KEY_F7),
        "F8" => Ok(KEY_F8),
        "F9" => Ok(KEY_F9),
        "F10" => Ok(KEY_F10),
        "F11" => Ok(KEY_F11),
        "F12" => Ok(KEY_F12),

        // Touches de navigation
        "UP" => Ok(KEY_UP),
        "DOWN" => Ok(KEY_DOWN),
        "LEFT" => Ok(KEY_LEFT),
        "RIGHT" => Ok(KEY_RIGHT),
        "HOME" => Ok(KEY_HOME),
        "END" => Ok(KEY_END),
        "PAGE_UP" => Ok(KEY_PAGEUP),
        "PAGE_DOWN" => Ok(KEY_PAGEDOWN),
        "INSERT" => Ok(KEY_INS),
        "DELETE" => Ok(KEY_DEL),

        // Touches de contrôle
        "ESC" => Ok(KEY_ESC),
        "TAB" => Ok(KEY_TAB),
        "CAPS_LOCK" => Ok(KEY_CAPS),
        "ENTER" => Ok(KEY_ENTER),
        "SPACE" => Ok(KEY_SPACE),
        "BACKSPACE" => Ok(KEY_BACK),

        // Touches modificatrices
        "LEFT_SHIFT" => Ok(KEY_LSHIFT),
        "RIGHT_SHIFT" => Ok(KEY_RSHIFT),
        "LEFT_CTRL" => Ok(KEY_LCTRL),
        "RIGHT_CTRL" => Ok(KEY_RCTRL),
        "LEFT_ALT" => Ok(KEY_LALT),
        "RIGHT_ALT" => Ok(KEY_RALT),
        "LEFT_WIN" => Ok(KEY_LWIN),

        // Touches spéciales
        "PRINT_SCREEN" => Ok(KEY_PRTSCR),
        "SCROLL_LOCK" => Ok(KEY_SCOLLLOCK),
        "PAUSE" => Ok(KEY_PAUSE),

        // Touches de ponctuation
        "COMMA" => Ok(KEY_COMMA),
        "DOT" => Ok(KEY_DOT),
        "SEMICOLON" => Ok(KEY_SEMICOLON),
        "QUOTE" => Ok(KEY_QUOTE),
        "LEFT_BRACKET" => Ok(KEY_LEFTBRACKET),
        "RIGHT_BRACKET" => Ok(KEY_RIGHTBRACKET),
        "VERTICAL_LINE" => Ok(KEY_VERTICALLINE),
        "TILDE" => Ok(KEY_WAVE),
        "EQUAL" => Ok(KEY_EQUAL),
        "MINUS" => Ok(KEY_SUB),
        "QUESTION" => Ok(KEY_QUESTION),

        // Pavé numérique
        "NUMPAD_0" => Ok(KEYBOARDPAD_0),
        "NUMPAD_1" => Ok(KEYBOARDPAD_1),
        "NUMPAD_2" => Ok(KEYBOARDPAD_2),
        "NUMPAD_3" => Ok(KEYBOARDPAD_3),
        "NUMPAD_4" => Ok(KEYBOARDPAD_4),
        "NUMPAD_5" => Ok(KEYBOARDPAD_5),
        "NUMPAD_6" => Ok(KEYBOARDPAD_6),
        "NUMPAD_7" => Ok(KEYBOARDPAD_7),
        "NUMPAD_8" => Ok(KEYBOARDPAD_8),
        "NUMPAD_9" => Ok(KEYBOARDPAD_9),
        "NUMPAD_PLUS" => Ok(KEYBOARDPAD_ADD),
        "NUMPAD_MINUS" => Ok(KEYBOARDPAD_SUB),
        "NUMPAD_MULTIPLY" => Ok(KEYBOARDPAD_MUL),
        "NUMPAD_DIVIDE" => Ok(KEYBOARDPAD_DIV),
        "NUMPAD_DOT" => Ok(KEYBOARDPAD_DOT),
        "NUMPAD_ENTER" => Ok(KEYBOARDPAD_ENTER),
        "NUM_LOCK" => Ok(KEYBOARDPAD_NUMLOCK),

        // Valeur nulle
        "NULL" => Ok(KEY_NULL),

        _ => Err(anyhow::anyhow!("Invalid key name: {}", key_name)),
    }
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub name: String,
    pub bindings: HashMap<String, Vec<String>>,
}

impl Profile {
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read profile from {}", path))?;
        toml::from_str(&content).with_context(|| format!("Failed to parse TOML from {}", path))
    }

    pub fn to_key_bindings(&self) -> Result<Vec<KeyBinding>> {
        let button_map: HashMap<_, _> = crate::config::BUTTON_NAMES
            .iter()
            .enumerate()
            .map(|(i, &name)| (name, i))
            .collect();

        let mut result = vec![[KEY_NULL; 4]; 16];

        for (button, keys) in &self.bindings {
            let idx = *button_map
                .get(button.as_str())
                .with_context(|| format!("Invalid button name: {}", button))?;

            for (i, key_name) in keys.iter().enumerate().take(4) {
                result[idx][i] = key_name_to_value(key_name.as_str()).with_context(|| {
                    format!("Invalid key name for button {}: {}", button, key_name)
                })?;
            }
        }

        Ok(result)
    }
}
