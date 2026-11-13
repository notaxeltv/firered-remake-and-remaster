//! UI language for the Rust prototype shell.
//!
//! Game logic and imported FireRed data stay USA-sourced. This module only
//! translates presentation strings we own (HUD, Settings, battle shell).
//! Imported NPC/script dialogue remains English until a dialogue catalog lands.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub(super) enum Language {
    English,
    #[default]
    Italian,
}

impl Language {
    pub(super) fn from_ini(ini: &str) -> Self {
        if ini.lines().any(|l| l.trim() == "language=en") {
            Self::English
        } else {
            // Default Italian for this fork; explicit language=it also selects it.
            Self::Italian
        }
    }

    pub(super) fn ini_value(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Italian => "it",
        }
    }

    pub(super) fn toggle(self) -> Self {
        match self {
            Self::English => Self::Italian,
            Self::Italian => Self::English,
        }
    }

    pub(super) fn label(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::Italian => "Italiano",
        }
    }
}

pub(super) fn map_title(lang: Language, map_id: &str, fallback: &str) -> String {
    if !matches!(lang, Language::Italian) {
        return fallback.to_string();
    }
    let it = match map_id {
        "pallet" => "BIANCAVILLA",
        "route1" => "PERCORSO 1",
        "viridian" => "SMERALDOPOLI",
        "home" | "home_2f" => "CASA DEL GIOCATORE",
        "rival_home" => "CASA DEL RIVALE",
        "oak_lab" | "lab" => "LABORATORIO DI OAK",
        _ => return fallback.to_string(),
    };
    it.to_string()
}

pub(super) fn settings(
    lang: Language,
    orbit: bool,
    smooth: bool,
    third_person: bool,
    diagonal: bool,
    free_move: bool,
    source_art: bool,
    music: bool,
    battle_slice: bool,
    native_state: bool,
    field_scripts: bool,
    red_model: bool,
) -> String {
    let on = |v: bool| match lang {
        Language::English => {
            if v {
                "ON"
            } else {
                "OFF"
            }
        }
        Language::Italian => {
            if v {
                "SI"
            } else {
                "NO"
            }
        }
    };
    match lang {
        Language::English => format!(
            "SETTINGS\n\n1   Camera rotation: {}\n2   Smooth movement: {}\n3   Third-person mouse camera: {}\n4   Diagonal grid movement: {}\n5   Free movement: {}\n6   Original surface textures: {}\n7   Music: {}\n8   Opening rival battle slice: {}\n9   Native save/load (restart): {}\n0   Original field scripts (partial): {}\nV   Imported Red playable model: {}\nI   Language: {}\n\nTab / F1 / Esc: close     Changes save automatically.",
            on(orbit),
            on(smooth),
            on(third_person),
            on(diagonal),
            on(free_move),
            on(source_art),
            on(music),
            on(battle_slice),
            on(native_state),
            on(field_scripts),
            on(red_model),
            lang.label()
        ),
        Language::Italian => format!(
            "IMPOSTAZIONI\n\n1   Rotazione camera: {}\n2   Movimento fluido: {}\n3   Camera in terza persona (mouse): {}\n4   Movimento diagonale a griglia: {}\n5   Movimento libero: {}\n6   Texture originali: {}\n7   Musica: {}\n8   Battaglia rivale (slice): {}\n9   Salvataggio nativo (riavvio): {}\n0   Script di campo originali (parziale): {}\nV   Modello Red giocabile: {}\nI   Lingua: {}\n\nTab / F1 / Esc: chiudi     Le modifiche si salvano da sole.",
            on(orbit),
            on(smooth),
            on(third_person),
            on(diagonal),
            on(free_move),
            on(source_art),
            on(music),
            on(battle_slice),
            on(native_state),
            on(field_scripts),
            on(red_model),
            lang.label()
        ),
    }
}

pub(super) fn settings_extras(
    lang: Language,
    actor_scenes: bool,
    character_animation: bool,
    trainer_models: bool,
    pokemon_models: bool,
    letsgo_scenery: bool,
    effects: bool,
    quick_start: bool,
    battle_3d: bool,
    letsgo_shading: bool,
) -> String {
    let on = |v: bool| match lang {
        Language::English => {
            if v {
                "ON"
            } else {
                "OFF"
            }
        }
        Language::Italian => {
            if v {
                "SI"
            } else {
                "NO"
            }
        }
    };
    match lang {
        Language::English => format!(
            "\nM: Original actors and map scenes (partial): {}\nA: Character animations: {}   T: Trainer models: {}   P: Pokemon models: {}   L: Let's Go scenery: {}   K: Effects: {}\nY: Slice quick start (pick a starter, skip story): {}   G: 3D battle stage: {}\nH: Let's Go shading (trees, ground, water): {}",
            on(actor_scenes),
            on(character_animation),
            on(trainer_models),
            on(pokemon_models),
            on(letsgo_scenery),
            on(effects),
            on(quick_start),
            on(battle_3d),
            on(letsgo_shading)
        ),
        Language::Italian => format!(
            "\nM: Attori e scene mappa originali (parziale): {}\nA: Animazioni personaggi: {}   T: Modelli allenatori: {}   P: Modelli Pokemon: {}   L: Scenario Let's Go: {}   K: Effetti: {}\nY: Avvio rapido slice (starter, salta storia): {}   G: Palco battaglia 3D: {}\nH: Ombreggiatura Let's Go (alberi, suolo, acqua): {}",
            on(actor_scenes),
            on(character_animation),
            on(trainer_models),
            on(pokemon_models),
            on(letsgo_scenery),
            on(effects),
            on(quick_start),
            on(battle_3d),
            on(letsgo_shading)
        ),
    }
}

pub(super) fn reset_prompt(lang: Language, pending: bool) -> &'static str {
    match (lang, pending) {
        (Language::English, true) => {
            "\nReset all Rust progress? Press R again to confirm. Previous save backed up."
        }
        (Language::English, false) => {
            "\nR: reset Rust save and progress (keeps current location)"
        }
        (Language::Italian, true) => {
            "\nAzzerare tutti i progressi Rust? Premi di nuovo R per confermare. Salvataggio precedente in backup."
        }
        (Language::Italian, false) => {
            "\nR: azzera salvataggio e progressi Rust (mantiene la posizione)"
        }
    }
}

pub(super) fn starter_prompt(lang: Language, map_title: &str) -> String {
    match lang {
        Language::English => format!(
            "{map_title}   |   PLAYABLE SLICE\nChoose your partner:   1  Bulbasaur     2  Charmander     3  Squirtle\nThen explore Pallet Town and Route 1: tall grass has wild Pokemon. Tab: settings"
        ),
        Language::Italian => format!(
            "{map_title}   |   SLICE GIOCABILE\nScegli il partner:   1  Bulbasaur     2  Charmander     3  Squirtle\nPoi esplora Biancavilla e Percorso 1: nell'erba alta ci sono Pokemon selvatici. Tab: impostazioni"
        ),
    }
}

pub(super) fn field_controls(lang: Language) -> &'static str {
    match lang {
        Language::English => {
            "WASD: move   Shift: run   Space: interact   Mouse: look   Esc: cursor   Tab: settings   B: rival battle"
        }
        Language::Italian => {
            "WASD: muovi   Shift: corri   Spazio: interagisci   Mouse: guarda   Esc: cursore   Tab: impostazioni   B: battaglia rivale"
        }
    }
}

pub(super) fn field_header(lang: Language) -> &'static str {
    match lang {
        Language::English => "NATIVE RUST SLICE",
        Language::Italian => "SLICE RUST NATIVO",
    }
}

pub(super) fn opening_hint(lang: Language, map_id: &str) -> Option<&'static str> {
    match (lang, map_id) {
        (Language::English, "route1") => {
            Some("South: Pallet Town. Talk to the Mart clerk. Jump ledges heading south.")
        }
        (Language::English, "pallet") => {
            Some("North: Route 1. Enter the houses and lab. Read signs and talk to neighbours.")
        }
        (Language::Italian, "route1") => {
            Some("Sud: Biancavilla. Parla con l'addetto del Market. Salta i dirupi verso sud.")
        }
        (Language::Italian, "pallet") => {
            Some("Nord: Percorso 1. Entra nelle case e nel laboratorio. Leggi i cartelli e parla con i vicini.")
        }
        _ => None,
    }
}

pub(super) fn indoor_hint(lang: Language) -> &'static str {
    match lang {
        Language::English => "Explore the rooms. Doors and stairs connect the original maps.",
        Language::Italian => "Esplora le stanze. Porte e scale collegano le mappe originali.",
    }
}

pub(super) fn explore_hint(lang: Language) -> &'static str {
    match lang {
        Language::English => "Explore the area.",
        Language::Italian => "Esplora la zona.",
    }
}

pub(super) fn continue_prompt(lang: Language) -> &'static str {
    match lang {
        Language::English => "Space / Enter: continue",
        Language::Italian => "Spazio / Invio: continua",
    }
}

pub(super) fn cardinal(lang: Language, dir: &str) -> &'static str {
    match (lang, dir) {
        (Language::English, "up") => "North",
        (Language::English, "down") => "South",
        (Language::English, "left") => "West",
        (Language::English, "right") => "East",
        (Language::Italian, "up") => "Nord",
        (Language::Italian, "down") => "Sud",
        (Language::Italian, "left") => "Ovest",
        (Language::Italian, "right") => "Est",
        _ => "?",
    }
}

/// Connection-based outdoor hint; uses display names already localized by caller.
pub(super) fn connections_hint(lang: Language, parts: &[(/*cardinal*/ &str, /*place*/ &str)]) -> String {
    if parts.is_empty() {
        return explore_hint(lang).to_string();
    }
    let body = parts
        .iter()
        .map(|(c, place)| format!("{c}: {place}"))
        .collect::<Vec<_>>()
        .join(". ");
    format!("{body}.")
}

pub(super) fn dialogue_line(lang: Language, name: &str, line: &str) -> String {
    // Phase 1: UI chrome only. Imported script text stays as sourced until catalogued.
    let _ = lang;
    format!("{name}\n\n{line}\n\n{}", continue_prompt(lang))
}

pub(super) fn save_reset_failed(lang: Language, err: &str) -> String {
    match lang {
        Language::English => format!("Save reset failed: {err}"),
        Language::Italian => format!("Reset salvataggio non riuscito: {err}"),
    }
}

// --- Battle shell -----------------------------------------------------------------

pub(super) fn battle_header_wild(lang: Language) -> &'static str {
    match lang {
        Language::English => "WILD POKEMON BATTLE  |  NATIVE RUST",
        Language::Italian => "LOTTA POKEMON SELVATICO  |  RUST NATIVO",
    }
}

pub(super) fn battle_header_rival(lang: Language) -> &'static str {
    match lang {
        Language::English => "FIRST RIVAL BATTLE  |  NATIVE RUST",
        Language::Italian => "PRIMA BATTAGLIA COL RIVALE  |  RUST NATIVO",
    }
}

pub(super) fn battle_subtitle_wild(lang: Language) -> &'static str {
    match lang {
        Language::English => {
            "Wild battle. Throw Poke Balls with [C], fight with [1-4], or flee with [5]."
        }
        Language::Italian => {
            "Lotta selvatica. Lancia Poke Ball con [C], attacca con [1-4], o fuggi con [5]."
        }
    }
}

pub(super) fn battle_subtitle_rival(lang: Language) -> &'static str {
    match lang {
        Language::English => {
            "Original FireRed trainer AI and four opening moves. Fixed test starter and random seed."
        }
        Language::Italian => {
            "IA allenatore FireRed originale e quattro mosse iniziali. Starter di test fisso e seed casuale."
        }
    }
}

pub(super) fn battle_status_caught(lang: Language) -> &'static str {
    match lang {
        Language::English => "Pokemon caught! Press Esc to return to field",
        Language::Italian => "Pokemon catturato! Premi Esc per tornare al campo",
    }
}

pub(super) fn battle_status_fled(lang: Language) -> &'static str {
    match lang {
        Language::English => "Got away safely! Press Esc to return to field",
        Language::Italian => "Scampato pericolo! Premi Esc per tornare al campo",
    }
}

pub(super) fn battle_status_won(lang: Language) -> &'static str {
    match lang {
        Language::English => "Battle ended - You won! Press Esc or R to return",
        Language::Italian => "Battaglia finita - Hai vinto! Premi Esc o R per tornare",
    }
}

pub(super) fn battle_status_lost(lang: Language) -> &'static str {
    match lang {
        Language::English => "Battle ended - Defeated! Press Esc or R to return",
        Language::Italian => "Battaglia finita - Sconfitto! Premi Esc o R per tornare",
    }
}

pub(super) fn battle_status_turn(lang: Language, turn: u32) -> String {
    match lang {
        Language::English => format!(
            "Turn {turn}   |   Enemy move used   |   Choose your move [1-4], Catch [C], or Run [5]"
        ),
        Language::Italian => format!(
            "Turno {turn}   |   Mossa nemica usata   |   Scegli mossa [1-4], Cattura [C], o Fuga [5]"
        ),
    }
}

pub(super) fn battle_status_wild_appear(lang: Language, name: &str, level: u8) -> String {
    match lang {
        Language::English => format!(
            "Wild {name} appeared! Lv. {level}. Catch with [C] or attack with [1-4]."
        ),
        Language::Italian => format!(
            "Appare {name} selvatico! Lv. {level}. Cattura con [C] o attacca con [1-4]."
        ),
    }
}

pub(super) fn battle_status_rival_fixture(lang: Language) -> &'static str {
    match lang {
        Language::English => {
            "Charmander: Hardy, IVs 31 / EVs 0. Rival Squirtle: original trainer-generated stats."
        }
        Language::Italian => {
            "Charmander: Hardy, IV 31 / EV 0. Rivale Squirtle: stats generate dall'allenatore originale."
        }
    }
}

pub(super) fn battle_btn_ball(lang: Language) -> &'static str {
    match lang {
        Language::English => "C  Poke Ball",
        Language::Italian => "C  Poke Ball",
    }
}

pub(super) fn battle_error(lang: Language, english: &str) -> String {
    if matches!(lang, Language::English) {
        return english.to_string();
    }
    let it = match english {
        "A world battle cannot be restarted." => "Una lotta del mondo non si puo ricominciare.",
        "Finish this world battle before returning." => {
            "Termina questa lotta del mondo prima di tornare."
        }
        "OAK: You cannot run from a TRAINER battle!" => {
            "OAK: Non puoi fuggire da una lotta con un ALLENATORE!"
        }
        "The trainer blocked the BALL!" => "L'allenatore ha bloccato la BALL!",
        "The battle has already ended!" => "La lotta e gia finita!",
        "The Pokémon Boxes are full!" | "The Pokemon Boxes are full!" => {
            "I Box Pokemon sono pieni!"
        }
        "No POKE BALLS are available." => "Nessuna POKE BALL disponibile.",
        _ => english,
    };
    it.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_italian_unless_en_set() {
        assert_eq!(Language::from_ini(""), Language::Italian);
        assert_eq!(Language::from_ini("language=it\n"), Language::Italian);
        assert_eq!(Language::from_ini("language=en\n"), Language::English);
    }

    #[test]
    fn italian_settings_mention_lingua() {
        let s = settings(
            Language::Italian,
            true,
            true,
            true,
            false,
            false,
            true,
            false,
            true,
            false,
            true,
            true,
        );
        assert!(s.contains("IMPOSTAZIONI"));
        assert!(s.contains("Lingua: Italiano"));
    }
}
