// Escape the Obsidian Tower - Control Flow Exercise
// Tests: for loops, while loops, inner loops, conditionals, labeled breaks

use std::io::{self, Write};

// Define possible player actions
#[derive(Debug, Clone, Copy)]
enum Action {
    Attack,
    Defend,
    Flee,
}

fn main() {
    println!("========================================");
    println!("  ESCAPE THE OBSIDIAN TOWER");
    println!("========================================\n");

    // ===== STEP 1: Initialize player state =====
    let mut player_hp = 100;
    let mut player_escaped = false;
    let max_floors = 5;

    // ===== STEP 2: Outer loop - iterate through floors =====
    // The label 'tower_loop allows us to break out of this entire loop from deep inside
    'tower_loop: for floor in 1..=max_floors {
        println!("\n--- FLOOR {} ---\n", floor);

        // Each floor has an enemy with HP based on the floor number
        let mut enemy_hp = 30 + (floor as i32 * 10);
        let mut floor_cleared = false;

        // ===== STEP 3: Middle loop - continue while fight is active =====
        // This loop runs as long as both player and enemy are alive
        while enemy_hp > 0 && player_hp > 0 {
            println!("  Player HP: {}  |  Enemy HP: {}", player_hp, enemy_hp);

            // ===== STEP 4: Inner loop - handle one player turn =====
            // This loop can repeat if the turn is invalid (using continue)
            loop {
                // Get player input from terminal
                // If input is invalid, continue will loop back to prompt again
                let action = match get_player_action() {
                    Some(act) => act,
                    None => {
                        // Invalid input: continue loops back to ask again
                        continue;
                    }
                };
                println!("  You choose: {:?}", action);

                // Execute the action
                match action {
                    Action::Attack => {
                        // Player attacks: deal random damage between 5-15
                        let damage = 5 + (floor as i32 % 13); // Simple "randomness"
                        enemy_hp -= damage;
                        println!("  You attack for {} damage!\n", damage);
                        // Turn is complete, exit inner loop
                        break;
                    }
                    Action::Defend => {
                        // Player defends: reduce incoming damage later
                        println!("  You brace yourself for impact!\n");
                        break;
                    }
                    Action::Flee => {
                        // Player tries to flee
                        println!("  You run for your life!\n");
                        player_escaped = true;
                        // Use labeled break to jump out of the floor loop entirely
                        // This skips the rest of the current floor AND all remaining floors
                        break 'tower_loop;
                    }
                }
                // Inner loop ends here
            }

            // ===== STEP 5: Check if enemy is defeated =====
            if enemy_hp <= 0 {
                floor_cleared = true;
                println!("  >> Enemy defeated on Floor {}!\n", floor);
                break; // Exit the while loop
            }

            // ===== STEP 6: Enemy's turn =====
            // Only happens if player didn't defeat the enemy
            if player_hp > 0 && enemy_hp > 0 {
                let enemy_damage = 8 + (floor as i32 % 5);
                player_hp -= enemy_damage;
                println!("  Enemy attacks you for {} damage!\n", enemy_damage);

                // Check if player is dead
                if player_hp <= 0 {
                    println!("  >> You were defeated on Floor {}!\n", floor);
                    break; // Exit the while loop
                }
            }
            // while loop continues: back to top to check conditions again
        }

        // ===== STEP 7: Evaluate floor outcome =====
        // This if/else determines whether we continue to the next floor or stop
        if player_hp <= 0 {
            // Player died - end the game
            println!("GAME OVER - You died on Floor {}.", floor);
            break; // Exit the for loop, no more floors
        } else if floor_cleared {
            // Floor was cleared - continue to next floor
            if floor < max_floors {
                println!("Preparing for Floor {}...", floor + 1);
            }
            // for loop naturally continues to next iteration
        }
    }
    // 'tower_loop ends here

    // ===== STEP 8: Print final outcome =====
    println!("\n========================================");
    if player_escaped {
        println!("  SUCCESS: You escaped the Obsidian Tower!");
        println!("  You fled with {} HP remaining.", player_hp);
    } else if player_hp <= 0 {
        println!("  FAILURE: You were defeated in the tower.");
    } else {
        println!("  SUCCESS: You cleared all {} floors!", max_floors);
        println!("  You escaped with {} HP remaining.", player_hp);
    }
    println!("========================================");
}

// Helper function: Get player action from terminal input
// Displays options and reads user input
// Returns Some(Action) if valid input, None if invalid (which triggers continue in the turn loop)
fn get_player_action() -> Option<Action> {
    // Display action options to the player
    println!("\n  ========== YOUR TURN ==========");
    println!("  [1] Attack  - Deal damage to enemy");
    println!("  [2] Defend  - Reduce incoming damage");
    println!("  [3] Flee    - Try to escape the tower");
    print!("  Enter your choice (1/2/3): ");
    
    // Flush stdout to ensure the prompt displays immediately
    io::stdout().flush().ok();

    // Read a line from terminal input
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Trim whitespace and convert to lowercase
            let choice = input.trim().to_lowercase();

            // Match input to an action
            match choice.as_str() {
                "1" | "attack" => Some(Action::Attack),
                "2" | "defend" => Some(Action::Defend),
                "3" | "flee" => Some(Action::Flee),
                _ => {
                    // Invalid input - print error and return None
                    // This will trigger continue in the inner loop, re-prompting the player
                    println!("  ERROR: Invalid choice '{}'. Please enter 1, 2, or 3.\n", input.trim());
                    None
                }
            }
        }
        Err(e) => {
            // Error reading input
            println!("  ERROR: Failed to read input: {}\n", e);
            None
        }
    }
}