use rand::Rng;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use crate::names;
use crate::visualization;

#[derive(Debug, Clone)]
pub struct TypeOfGuy {
    pub name: String,
    pub strength: u8,
    pub charisma: u8,
    pub agility: u8,
    pub battle_cries: Vec<String>,
}

pub struct GameState {
    pub builtin_types_of_guy: Vec<TypeOfGuy>,
}

impl TypeOfGuy {
    fn new(
        name: String,
        strength: u8,
        agility: u8,
        charisma: u8,
        battle_cries: Vec<String>,
    ) -> Self {
        TypeOfGuy {
            name,
            strength,
            agility,
            charisma,
            battle_cries,
        }
    }
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            builtin_types_of_guy: vec![
                TypeOfGuy::new(
                    "Guy who's made of nails".to_string(),
                    18,
                    0,
                    0,
                    vec!["Nailed it!".to_string()],
                ),
                TypeOfGuy::new(
                    "Guy who's made of normal guy stuff, except his hands, which are made of nails"
                        .to_string(),
                    14,
                    6,
                    4,
                    vec!["Why is life pain?".to_string()],
                ),
            ],
        }
    }
}

#[derive(Clone)]
pub struct Guy<'a> {
    pub name: String,
    pub guy_type: &'a TypeOfGuy,
}

impl<'a> Guy<'a> {
    fn new(name: String, guy_type: &'a TypeOfGuy) -> Self {
        Guy { name, guy_type }
    }
}

fn select_random_guy_type(state: &GameState) -> TypeOfGuy {
    let mut rng = rand::thread_rng();

    // Get total number of guy types
    let builtin_count = state.builtin_types_of_guy.len();
    let total_count = builtin_count;

    // Generate a single random index for all types
    let idx = rng.gen_range(0..total_count);

    // Select from builtin types
    state.builtin_types_of_guy[idx].clone()
}

fn roll_attribute_contest(attr1: u8, attr2: u8) -> (bool, u32, u32) {
    let mut rng = rand::thread_rng();

    loop {
        let roll1 = rng.gen_range(1..=DICE_SIDES) + attr1 as u32;
        let roll2 = rng.gen_range(1..=DICE_SIDES) + attr2 as u32;

        if roll1 != roll2 {
            return (roll1 > roll2, roll1, roll2);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ContestType {
    Strength,
    Agility,
    Charisma,
}

const DICE_SIDES: u32 = 20;

fn fight_round(guy1: &Guy, guy2: &Guy, round: u8) -> bool {
    let mut rng = rand::thread_rng();

    // Randomly select which attribute to contest
    let contest_type = rng.gen_range(0..3);
    let (attr1, attr2, contest_type) = match contest_type {
        0 => (
            guy1.guy_type.strength,
            guy2.guy_type.strength,
            ContestType::Strength,
        ),
        1 => (
            guy1.guy_type.agility,
            guy2.guy_type.agility,
            ContestType::Agility,
        ),
        _ => (
            guy1.guy_type.charisma,
            guy2.guy_type.charisma,
            ContestType::Charisma,
        ),
    };

    let (guy1_wins, roll1, roll2) = roll_attribute_contest(attr1, attr2);
    visualization::print_fight_round(round, guy1, guy2, contest_type, attr1, attr2, roll1, roll2);
    guy1_wins
}

fn fight(state: &GameState) {
    // Generate two random fighters
    let guy_type1 = select_random_guy_type(state);
    let guy_type2 = select_random_guy_type(state);

    let guy1 = Guy::new(names::generate_name(), &guy_type1);
    let guy2 = Guy::new(names::generate_name(), &guy_type2);

    let mut guy1_wins = 0;
    let mut guy2_wins = 0;

    visualization::print_fight_introduction(&guy1, &guy2);

    for round in 1..=3 {
        if fight_round(&guy1, &guy2, round) {
            guy1_wins += 1;
        } else {
            guy2_wins += 1;
        }

        if guy1_wins == 2 {
            visualization::print_winner(&guy1.name);
            return;
        } else if guy2_wins == 2 {
            visualization::print_winner(&guy2.name);
            return;
        }
    }
}

fn main_loop(state: GameState) -> rustyline::Result<()> {
    visualization::print_header();
    visualization::print_menu();

    let mut rl = DefaultEditor::new()?;
    loop {
        match rl.readline("guy> ") {
            Ok(line) => {
                let command = line.trim().to_lowercase();
                if command.is_empty() {
                    continue; // Skip empty lines
                }
                rl.add_history_entry(&line)?;

                match command.as_str() {
                    "quit" => break,
                    "types" => visualization::print_guy_types(&state),
                    "fight" => fight(&state),
                    "help" => visualization::print_menu(),
                    _ => println!("Unknown command: {}", command),
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                eprintln!("Error reading line: {:?}", err);
            }
        }
    }
    Ok(())
}

pub fn run_game() -> wasmtime::Result<()> {
    let state = GameState::new();

    if let Err(e) = main_loop(state) {
        return Err(wasmtime::Error::msg(format!("Error in main loop: {:?}", e)));
    }
    Ok(())
}
