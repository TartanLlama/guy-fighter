use crate::game::{ContestType, Guy, GameState, TypeOfGuy};
use rand::seq::SliceRandom;
use unicode_width::UnicodeWidthStr;

const STRENGTH_EMOJI: &str = "💪";
const AGILITY_EMOJI: &str = "🏃";
const CHARISMA_EMOJI: &str = "✨";

fn create_bar(value: u8) -> String {
    let bar_length = 20;
    let filled = "█".repeat(value as usize);
    let empty = "░".repeat(bar_length - value as usize);
    format!("[{}{}]", filled, empty)
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut wrapped = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > width {
            wrapped.push(current_line.clone());
            current_line.clear();
        } else if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }
    if !current_line.is_empty() {
        wrapped.push(current_line);
    }

    wrapped
}
fn box_content(lines: &[String], width: usize) {
    println!("╭─{}─╮", "─".repeat(width));
    for line in lines {
        let visual_width = UnicodeWidthStr::width(line.as_str());
        let padding = width.saturating_sub(visual_width);
        println!("│ {}{} │", line, " ".repeat(padding));
    }
    println!("╰─{}─╯", "─".repeat(width));
}

fn print_guy_card(guy: &TypeOfGuy) {
    let wrapped_name = wrap_text(&guy.name, 50);

    let mut content = Vec::new();
    content.push(format!("👤 {}", wrapped_name[0]));
    for chunk in &wrapped_name[1..] {
        content.push(format!("   {}", chunk));
    }
    content.push(" ".to_string());
    content.push(format!(
        "{} Strength: {:2} {}",
        STRENGTH_EMOJI,
        guy.strength,
        create_bar(guy.strength)
    ));
    content.push(format!(
        "{} Agility:  {:2} {}",
        AGILITY_EMOJI,
        guy.agility,
        create_bar(guy.agility)
    ));
    content.push(format!(
        "{} Charisma: {:2} {}",
        CHARISMA_EMOJI,
        guy.charisma,
        create_bar(guy.charisma)
    ));

    box_content(&content, 50);
}

fn print_guy_types_header() {
    println!("\n{}", "═".repeat(57));
    println!("{}", "🥊 FIGHTER ROSTER 🥊");
    println!("{}", "═".repeat(57));
}

pub fn print_guy_types(state: &GameState) {
    print_guy_types_header();

    // Print builtin types
    for guy in state.builtin_types_of_guy.iter() {
        print_guy_card(guy);
    }
}

pub fn print_header() {
    println!(
        "
╔══════════════════════════════════════════════════════════════════════════════════════╗
║                                                                                      ║
║  ██████╗ ██╗   ██╗██╗   ██╗    ███████╗██╗ ██████╗ ██╗  ██╗████████╗███████╗██████╗  ║
║ ██╔════╝ ██║   ██║╚██╗ ██╔╝    ██╔════╝██║██╔════╝ ██║  ██║╚══██╔══╝██╔════╝██╔══██╗ ║
║ ██║  ███╗██║   ██║ ╚████╔╝     █████╗  ██║██║  ███╗███████║   ██║   █████╗  ██████╔╝ ║
║ ██║   ██║██║   ██║  ╚██╔╝      ██╔══╝  ██║██║   ██║██╔══██║   ██║   ██╔══╝  ██╔══██╗ ║
║ ╚██████╔╝╚██████╔╝   ██║       ██║     ██║╚██████╔╝██║  ██║   ██║   ███████╗██║  ██║ ║
║  ╚═════╝  ╚═════╝    ╚═╝       ╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚══════╝╚═╝  ╚═╝ ║
║                                                                                      ║
║                                                                                      ║ 
║                                                                                      ║
║                                🥊 ULTIMATE EDITION 🥊                                ║
╚══════════════════════════════════════════════════════════════════════════════════════╝"
    );
}

pub fn print_menu() {
    println!("\n╭────────────────────────────────╮");
    println!("│           🎮 Menu 🎮           │");
    println!("├────────────────────────────────┤");
    println!("│  types   │ 👥 View guy types   │");
    println!("│  fight   │ ⚔️ Start battle     │");
    println!("│  help    │ ❓ Show help        │");
    println!("│  quit    │ 🚪 Exit game        │");
    println!("╰────────────────────────────────╯");
}

pub fn print_fight_round(
    round: u8,
    guy1: &Guy,
    guy2: &Guy,
    contest_type: ContestType,
    attr1: u8,
    attr2: u8,
    roll1: u32,
    roll2: u32,
) {
    let (contest_name, contest_emoji) = match contest_type {
        ContestType::Strength => ("Strength", STRENGTH_EMOJI),
        ContestType::Agility => ("Agility", AGILITY_EMOJI),
        ContestType::Charisma => ("Charisma", CHARISMA_EMOJI),
    };

    let header_lines = vec![format!(
        "ROUND {} - {} {} Contest",
        round, contest_emoji, contest_name
    )];
    box_content(&header_lines, 35);

    println!();

    println!(
        "🔴 {} (Base {}: {}) rolled: {}",
        guy1.name, contest_name, attr1, roll1
    );
    println!(
        "🔵 {} (Base {}: {}) rolled: {}",
        guy2.name, contest_name, attr2, roll2
    );
    println!();

    if roll1 > roll2 {
        println!("🎉 {} wins the round! 🎉", guy1.name);
    } else {
        println!("🎉 {} wins the round! 🎉", guy2.name);
    }

    println!();
}

fn print_fight_card(guy: &Guy, corner: &str) {
    let corner_emoji = if corner == "red" { "🔴" } else { "🔵" };
    let wrapped_type_name = wrap_text(&guy.guy_type.name, 41);

    println!(
        "┌─ {:>4} CORNER ─────────────────────────────────────┐",
        corner.to_uppercase()
    );
    println!("│ {} {}", corner_emoji, guy.name);
    println!("│ 📝 Type: {}", wrapped_type_name[0]);
    for line in &wrapped_type_name[1..] {
        println!("│          {}", line);
    }
    println!(
        "│ {} Strength: {} │ {} Agility: {} │ {} Charisma: {}",
        STRENGTH_EMOJI,
        guy.guy_type.strength,
        AGILITY_EMOJI,
        guy.guy_type.agility,
        CHARISMA_EMOJI,
        guy.guy_type.charisma
    );
    println!(
        "| Battle cry: {}",
        guy.guy_type.battle_cries
            .choose(&mut rand::thread_rng())
            .map_or("No battle cry", |cry| cry)
    );
    println!("└───────────────────────────────────────────────────┘");
}

pub fn print_fight_introduction(guy1: &Guy, guy2: &Guy) {
    println!("╔══════════════════════════════════════════╗");
    println!("║            🥊 FIGHT TIME! 🥊             ║");
    println!("╚══════════════════════════════════════════╝");
    println!();

    print_fight_card(guy1, "red");
    println!();
    println!("                         🆚");
    println!();
    print_fight_card(guy2, "blue");

    println!();
    println!("🏁 BEST OF 3 ROUNDS - FIGHT! 🏁");
    println!("{}", "═".repeat(50));
    println!();
}

pub fn print_winner(winner_name: &str) {
    println!();
    println!(
        "🎉🏆🎉 {} WINS THE FIGHT! 🎉🏆🎉",
        winner_name.to_uppercase()
    );
    println!("");
    println!("{}", "═".repeat(50));
}
