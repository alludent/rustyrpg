extern crate core;

use core::fmt;
use std::io;
use std::time::{Duration, SystemTime};


// todo, remove current save

fn print_main_help() {
    println!("\n=-=-==-===-===-==-= HELP MENU -=-=-==-=-==-===-=-");
    println!("* new (n)      Create a new game instance.");
    println!("                 safe: [n or new]");
    println!("                 quick: [n] [save name]");
    println!("* load (l)     Choose a saved game to load");
    println!("                 safe: [l or load]");
    println!("                 quick: [l] [save name]");
    println!("* remove (r)   Choose a saved game to remove");
    println!("                 safe: [r or remove]");
    println!("                 quick: [r] [save name]");
    println!("* view (v)     View current save data.");
    println!("                 usage: [v or view]");
    println!("* all (a)      List all saves.");
    println!("                 usage: [a or all]");
    println!("* market (m)   Obtain assets in the Market.");
    println!("                 usage: [m or market]");
    println!("* dungeon (d)  Test your luck in the Dungeon.");
    println!("                 usage: [d or dungeon]");
    println!("* wager (w)    Wager your rusty tokens.");
    println!("                 safe: [w or wager]");
    println!("                 quick: [w] [game type] [bet amount]");
    println!("* exit (e)     Exit the game.");
    println!("=-=-=--==-===-==-=-==-==-==-=-===-=-=-=-==-=-==-=");
    println!("\nUsage: [command to execute]");
}


fn print_new_help() {
    println!("\nUsage: [name of new file]");
    println!("c: cancel, h: help, v: view, a: all");
}


fn print_all_saves(saves: &Vec<Save>, curr_save_idx: usize) {
    let mut i: usize = 0;
    println!("\n-==-==-=-===- YOUR SAVES =-==-=-=-===-");
    for save in saves.iter() { // output the saved games
        if i == curr_save_idx {  
            println!("*{}: {1} rusty tokens, {2} assets", 
            save.name, save.balance, save.items.len()+save.plants.len()+save.animals.len());
        } else {
            println!("{}: {1} rusty tokens, {2} assets", 
            save.name, save.balance, save.items.len()+save.plants.len()+save.animals.len());
        }
        i += 1;
    }
    println!("=-====-=-=-===-=-=-==-===-=-===-=-==-=");
}


fn print_load_help(curr_save_idx: usize, saves: &Vec<Save>) {
    print_all_saves(saves, curr_save_idx);
    println!("\nUsage: [name of save]");
    println!("c: cancel, h: help, v: view, a: all");
}


fn print_remove_help(curr_save_idx: usize, saves: &Vec<Save>) {
    print_all_saves(saves, curr_save_idx);
    println!("\nUsage: [name of save]");
    println!("c: cancel, h: help, v: view, a: all");
}


fn print_wager_help() {
    println!("\n-==-====-==-==-====- GAME MENU ===-==-=-==-===-===-");
    println!("* coinX (cX)    bet on heads (ch) or tails (ct)");
    println!("                to win or lose bet amount.");
    println!("* diceX (dX)    guess the number rolled (X) on");
    println!("                a 6 sided dice to win or lose");
    println!("                bet amount times X.");
    println!("* nextX (nX)    guess if the next number is");
    println!("                higher (nh) or lower (nl). Initial");
    println!("                guess is from 50. Range is from 1-99.");
    println!("=-==-===--===-===-====-=-==-=-===-===-===-=-==-==-=");
    println!("\nUsage: [game] [bet amount]");
    println!("c: cancel, h: help, v: view, a: all");
}


fn print_low_market_help() {
    let base_item_cost: usize = 30;
    let base_plant_cost: usize = 25;
    let base_animal_cost: usize = 35;
    println!("\n==-==-==-===-==-=-= THE MARKET -====-==-=-=-=-===-");
    println!("Buy items to boost dungeon ability or token generation");
    println!("Buy plants/animals to generate rusty tokens.");
    println!("All products are stackable!");

    println!("\n--Low level items--");
    println!("* starter_fertilizer: {} rusty tokens", base_item_cost);
    println!("* starter_feed: {} rusty tokens", base_item_cost);
    println!("* rusty_sword: {} rusty tokens", base_item_cost*5*4);
    println!("* wooden_shield: {} rusty tokens", base_item_cost*5*7);
    println!("* leather_armor: {} rusty tokens", base_item_cost*5*12);

    println!("\n--Low level plants--");
    println!("* wild_grass: {} rusty tokens", base_plant_cost);
    println!("* autumn_leaf: {} rusty tokens", base_plant_cost*5*3);
    println!("* coral_fern: {} rusty tokens", base_plant_cost*5*6);
    println!("* poison_vine: {} rusty tokens", base_plant_cost*5*9);
    println!("* morning_oak: {} rusty tokens", base_plant_cost*5*12);
    println!("* bonsai_tree: {} rusty tokens", base_plant_cost*5*15);

    println!("\n--Low level animals-- ");
    println!("* wild_chicken: {} rusty tokens", base_animal_cost);
    println!("* striped_duck: {} rusty tokens", base_animal_cost*5*3);
    println!("* ironback_snail: {} rusty tokens", base_animal_cost*5*6);
    println!("* silver_rabbit: {} rusty tokens", base_animal_cost*5*9);
    println!("* steeltusk_boar: {} rusty tokens", base_animal_cost*5*12);

    println!("Have at least 1200 rusty tokens to unlock the next market!");
    println!("=-==-=-===-==-===-==-====-=-==-==-==-==-=-===-=-==");
    println!("\nUsage: [buy (b) or sell (s)] [amount to buy/sell] [product to buy/sell]");
    println!("checkout: buy items in your cart");
    println!("cart: remove/edit your cart");
    println!("c: cancel, h: help, v: view, a: all");
}


fn print_medium_market_help() {
    let base_item_cost: usize = 30;
    let base_plant_cost: usize = 25;
    let base_animal_cost: usize = 35;
    println!("\n==-==-==-===-==-=-= THE MARKET -====-==-=-=-=-===-");

    println!("\n--Low level items--");
    println!("* starter_fertilizer: {} rusty tokens", base_item_cost);
    println!("* starter_feed: {} rusty tokens", base_item_cost);
    println!("* rusty_sword: {} rusty tokens", base_item_cost*5*4);
    println!("* wooden_shield: {} rusty tokens", base_item_cost*5*7);
    println!("* leather_armor: {} rusty tokens", base_item_cost*5*12);

    println!("\n--Low level plants--");
    println!("* wild_grass: {} rusty tokens", base_plant_cost);
    println!("* autumn_leaf: {} rusty tokens", base_plant_cost*5*3);
    println!("* coral_fern: {} rusty tokens", base_plant_cost*5*6);
    println!("* poison_vine: {} rusty tokens", base_plant_cost*5*9);
    println!("* morning_oak: {} rusty tokens", base_plant_cost*5*12);
    println!("* bonsai_tree: {} rusty tokens", base_plant_cost*5*15);

    println!("\n--Low level animals-- ");
    println!("* wild_chicken: {} rusty tokens", base_animal_cost);
    println!("* striped_duck: {} rusty tokens", base_animal_cost*5*3);
    println!("* ironback_snail: {} rusty tokens", base_animal_cost*5*6);
    println!("* silver_rabbit: {} rusty tokens", base_animal_cost*5*9);
    println!("* steeltusk_boar: {} rusty tokens", base_animal_cost*5*12);

    println!("\n--Medium level items--");
    println!("* efficient_fertilizer: {} rusty tokens", base_item_cost*50);
    println!("* efficient_feed: {} rusty tokens", base_item_cost*50);
    println!("* steel_dagger: {} rusty tokens", base_item_cost*50*7);
    println!("* pocket_tower: {} rusty tokens", base_item_cost*50*10);
    println!("* steel_armor: {} rusty tokens", base_item_cost*50*15);

    println!("\n--Medium level plants--");
    println!("* bluesilver_grass: {} rusty tokens", base_plant_cost*50);
    println!("* giant_bamboo: {} rusty tokens", base_plant_cost*50*4);
    println!("* snapdragon: {} rusty tokens", base_plant_cost*50*8);
    println!("* firebloom: {} rusty tokens", base_plant_cost*50*12);
    println!("* golden_moss: {} rusty tokens", base_plant_cost*50*16);
    println!("* white_willow: {} rusty tokens", base_plant_cost*50*20);

    println!("\n--Medium level animals--");
    println!("* opal_turkey: {} rusty tokens", base_animal_cost*50);
    println!("* falling_squirrel: {} rusty tokens", base_animal_cost*50*4);
    println!("* gilded_fox: {} rusty tokens", base_animal_cost*50*8);
    println!("* silk_panther: {} rusty tokens", base_animal_cost*50*12);
    println!("* frostwing_falcon: {} rusty tokens", base_animal_cost*50*16);
    println!("* lunar_owl: {} rusty tokens", base_animal_cost*50*20);

    println!("\nHave at least 620000 rusty tokens to unlock the next market!");
    println!("=-==-=-===-==-===-==-====-=-==-==-==-==-=-===-=-==");
    println!("\nUsage: [buy (b) or sell (s)] [amount to buy/sell] [product to buy/sell]");
    println!("checkout: buy items in your cart");
    println!("cart: remove/edit your cart");
    println!("c: cancel, h: help, v: view, a: all");
}


fn print_high_market_help() {
    let base_item_cost: usize = 30;
    let base_plant_cost: usize = 25;
    let base_animal_cost: usize = 35;
    println!("\n==-==-==-===-==-=-= THE MARKET -====-==-=-=-=-===-");

    println!("\n--Low level items--");
    println!("* starter_fertilizer: {} rusty tokens", base_item_cost);
    println!("* starter_feed: {} rusty tokens", base_item_cost);
    println!("* rusty_sword: {} rusty tokens", base_item_cost*5*4);
    println!("* wooden_shield: {} rusty tokens", base_item_cost*5*7);
    println!("* leather_armor: {} rusty tokens", base_item_cost*5*12);

    println!("\n--Low level plants--");
    println!("* wild_grass: {} rusty tokens", base_plant_cost);
    println!("* autumn_leaf: {} rusty tokens", base_plant_cost*5*3);
    println!("* coral_fern: {} rusty tokens", base_plant_cost*5*6);
    println!("* poison_vine: {} rusty tokens", base_plant_cost*5*9);
    println!("* morning_oak: {} rusty tokens", base_plant_cost*5*12);
    println!("* bonsai_tree: {} rusty tokens", base_plant_cost*5*15);

    println!("\n--Low level animals--");
    println!("* wild_chicken: {} rusty tokens", base_animal_cost);
    println!("* striped_duck: {} rusty tokens", base_animal_cost*5*3);
    println!("* ironback_snail: {} rusty tokens", base_animal_cost*5*6);
    println!("* silver_rabbit: {} rusty tokens", base_animal_cost*5*9);
    println!("* steeltusk_boar: {} rusty tokens", base_animal_cost*5*12);

    println!("\n--Medium level items--");
    println!("* efficient_fertilizer: {} rusty tokens", base_item_cost*50);
    println!("* efficient_feed: {} rusty tokens", base_item_cost*50);
    println!("* steel_dagger: {} rusty tokens", base_item_cost*50*7);
    println!("* pocket_tower: {} rusty tokens", base_item_cost*50*10);
    println!("* steel_armor: {} rusty tokens", base_item_cost*50*15);

    println!("\n--Medium level plants--");
    println!("* bluesilver_grass: {} rusty tokens", base_plant_cost*50);
    println!("* giant_bamboo: {} rusty tokens", base_plant_cost*50*4);
    println!("* snapdragon: {} rusty tokens", base_plant_cost*50*8);
    println!("* firebloom: {} rusty tokens", base_plant_cost*50*12);
    println!("* golden_moss: {} rusty tokens", base_plant_cost*50*16);
    println!("* white_willow: {} rusty tokens", base_plant_cost*50*20);

    println!("\n--Medium level animals--");
    println!("* opal_turkey: {} rusty tokens", base_animal_cost*50);
    println!("* falling_squirrel: {} rusty tokens", base_animal_cost*50*4);
    println!("* gilded_fox: {} rusty tokens", base_animal_cost*50*8);
    println!("* silk_panther: {} rusty tokens", base_animal_cost*50*12);
    println!("* frostwing_falcon: {} rusty tokens", base_animal_cost*50*16);
    println!("* lunar_owl: {} rusty tokens", base_animal_cost*50*20);

    println!("\n--High level items--");
    println!("* masterwork_fertilizer: {} rusty tokens", base_item_cost*50*500);
    println!("* masterwork_feed: {} rusty tokens", base_item_cost*50*500);
    println!("* eternal_saber: {} rusty tokens", base_item_cost*50*500*10);
    println!("* infinite_aegis: {} rusty tokens", base_item_cost*50*500*17);
    println!("* world_armor: {} rusty tokens", base_item_cost*50*500*23);

    println!("\n--High level plants (produces a fortune of tokens)--");    
    println!("* astral_grass: {} rusty tokens", base_plant_cost*50*500);
    println!("* moonflower: {} rusty tokens", base_plant_cost*50*500*5);
    println!("* phoenix_blossom: {} rusty tokens", base_plant_cost*50*500*10);
    println!("* crystal_lotus: {} rusty tokens", base_plant_cost*50*500*15);
    println!("* world_tree: {} rusty tokens", base_plant_cost*50*500*23);

    println!("\n--High level animals (produces a fortune of tokens)--");
    println!("* golden_goose: {} rusty tokens", base_animal_cost*50*500);
    println!("* arcane_griffin: {} rusty tokens", base_animal_cost*50*500*5);
    println!("* soaring_phoenix: {} rusty tokens", base_animal_cost*50*500*10);
    println!("* dream_dragon: {} rusty tokens", base_animal_cost*50*500*15);
    println!("* nightmare_dragon: {} rusty tokens", base_animal_cost*50*500*20);
    println!("* world_serpent: {} rusty tokens", base_animal_cost*50*500*25);

    println!("\nticket: {} rusty tokens", base_item_cost*500*500*999999);

    println!("\n=-==-=-===-==-===-==-====-=-==-==-==-==-=-===-=-==");
    println!("\nUsage: [buy (b) or sell (s)] [amount to buy/sell] [product to buy/sell]");
    println!("checkout: buy items in your cart");
    println!("cart: remove/edit your cart");
    println!("c: cancel, h: help, v: view, a: all");
}


// more tokens = higher market
fn choose_market(saves: &mut Vec<Save>, curr_save_idx: usize) {
    if saves[curr_save_idx].balance < 25 {
        print_low_market_help(); // usage: [OP] [amount] [product]
    }
    else if saves[curr_save_idx].balance > 1200 {
        print_medium_market_help();
    }
    else if saves[curr_save_idx].balance > 620000 {
        print_high_market_help();
    }
}


fn print_cart_help(cart: &Vec<Product>) {
    // product *displays* [amount] [tag] 
    let mut _total_cost: usize = 0; 
    let mut _total_count: usize = 0; 
    println!("\n=-=-=-=-==- SHOPPING CART -==-=-=-==-=");
    for product in cart.iter() {
        let amount: usize = product.amount;
        let cost: usize = Product::cost(&product.tag) * amount;
        println!("{product}: {cost} rusty tokens");
        _total_cost += cost;
        _total_count += amount;
    }
    println!("\nTotal: {_total_count} items for {_total_cost} rusty tokens");   
    println!("==-==-=-===-==-==-====-==-===-==-=-==-");
    println!("\nUsage: [r or remove] [amount] [product]");
    println!("h: help, c: cancel");
}


fn print_balance(save: &Save) {
    println!("\n-==-=-===-===-==-==-=-==-=-=-=");
    println!("New balance: {} rusty tokens", save.balance);
    println!("=-==-==-=-==-=-==-==-=--==-=-=");
}


// welcome msg, returns player name
fn welcome() -> String {
    let mut input: String = String::new();

    loop {
        println!("\nWho are you?");
        io::stdin()
            .read_line(&mut input)
            .expect("Error: failed to read name");
        input = input.trim().to_string();
        if input.len() == 0 {
            println!("Name can't be empty.");
            continue;
        }
        else {
            break;
        }
    }

    println!("\n=-=-==-=-==-==-=-==--==-==-==-=-=-=-=-==-=-=-==-");
    println!("");
    println!("");
    println!(""); 
    println!("");
    println!("==-=-=-===-==-==-=-=-==-=-=-=-=-=-==-=-==--====-");

    println!("\n=-=-==-=-==-==-=-==--==-==-==-=-=-=-=-==-=-=-==-");
    println!("Welcome {input}. Here in the Walls of Descent,");
    println!("you can perform a variety of activities to gather");
    println!("enough money to escape. Don't fall into debt!"); 
    println!("Creator: dleer | Using: Rust");
    println!("==-=-=-===-==-==-=-=-==-=-=-=-=-=-==-=-==--====-");
    println!("Type \"h\" or \"help\" anytime!");
    
    return input;
}


// confirm an action
fn confirm_action() -> bool {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error: failed to read confirmation");
    input = input.trim().to_string();
    
    match input.as_str() {
        "yes" => return true,
        "y" => return true,
        "no" => return false,
        "n" => return false,
        _ => {
            println!("Did not enter \"yes\" or \"no\", assuming \"no\"");
            return false;
        },
    }
}


// check if save exists before performing operations
fn check_valid_save(curr_save_idx: usize, saves: &Vec<Save>) -> bool {
    if curr_save_idx == std::usize::MAX && saves.is_empty() {
        println!("\nNo games found. Please create a new game.");
        return false; 
    } 
    else if curr_save_idx == std::usize::MAX && !saves.is_empty() { 
        println!("\nGame not found. Please create a new game or load an existing one.");
        return false;
    }
    else {
        return true;
    }
}


// check at beginning of game loop if balance > 0
fn check_valid_balance(i: &mut usize, saves: &mut Vec<Save>) {
    if let Some(save) = saves.get_mut(*i) {  // get &mut save 
        if save.balance < 0 {
            println!("\n-==-===-=-==-== YOU LOST ==-=-=-===--=-=");
            println!("Create a new game or load an existing one.");
            println!("=-=-===-==-=-===-=-===-=-===-=-==-==-==-");
            saves.remove(*i);
            *i = std::usize::MAX;
        }
    }
}


// view current save data
fn view_current(curr_save_idx: usize, saves: &Vec<Save>) {
    if check_valid_save(curr_save_idx, &saves) {
        if let Some(save) = saves.get(curr_save_idx) {
            println!("{save}");
        }
    } 
}


// view all saves (name of save, balance, # items)
fn view_all(curr_save_idx: usize, saves: &Vec<Save>) {
    if curr_save_idx == std::usize::MAX && saves.is_empty() {
        println!("No games found. Please create a new game."); 
    } 
    else if curr_save_idx == std::usize::MAX && !saves.is_empty() { 
        print_all_saves(saves, curr_save_idx);
    }
    else {
        print_all_saves(saves, curr_save_idx);
    }
}


// create a new game and update curr save idx ( looped, has confirm )
fn new_game(player: &str, curr_save_idx: usize, saves: &mut Vec<Save>) -> Option<usize> {
    let mut input: String = String::new(); // user input
    let mut exists: bool; // name new game exists
    
    loop { // loop to get name of new game
        exists = false;
        input.clear();
        println!("\nNew> Name of new game, {player}?");
        io::stdin()
            .read_line(&mut input)
            .expect("Error: failed to read");
        input = input.trim().to_string();

        if input.len() == 0 {
            println!("Name can't be empty.");
            continue;
        }

        if input == "c" || input == "cancel" {
            return None;
        }
        if input == "h" || input == "help" {
            print_new_help();
            continue;
        }
        if input == "v" || input == "view" {
            view_current(curr_save_idx, saves);
            continue;
        }
        if input == "a" || input == "all" {
            view_all(curr_save_idx, saves);
            continue;
        }

        if input.contains(' ') {
            println!("Name can't have a space");
            continue;
        }

        for save in saves.iter() {
            if input == save.name {
                exists = true;
                break;
            }
        }
        
        if exists {
            println!("{input} already exists. Try again.");
        } 
        else {
            println!("Are you sure you want to create \"{input}\"? (y/n)");
            if confirm_action() {
                println!("\n=-===-=-==-= GAME CREATED -==-==-=-==-");
                println!("*{input}: 5 rusty tokens, 0 assets");
                let new_save: Save = Save {
                    name: input,
                    balance: 5,
                    items: Vec::new(),
                    plants: Vec::new(),
                    animals: Vec::new(),
                };
                println!("=-=-=-=-===-=-==-=-==-==-=-==-=-==-==-");

                saves.push(new_save);
                return Some(saves.len() - 1);
            } 
            else { 
                continue;
            }
        }   
    } // end of loop
    // logic after exiting new game loop
    
}


// create a new game and update curr save idx ( no loop, no confirm )
fn new_game_quick(saves: &mut Vec<Save>, name: &str) -> Option<usize> {
    let mut exists: bool; // name new game exists

    loop { // loop to get name of new game
        exists = false;
        if name == "c" {
            println!("Name cannot be \"c\"");
            return None;
        }
        for save in saves.iter() {
            if name == save.name {
                exists = true;
                break;
            }
        }
        
        if exists {
            println!("{name} already exists.");
            return None;
        } else {
            break;
        }   
    } // end of loop
    // logic after exiting new game loop
    println!("\n=-==-==-=-=- GAME CREATED -==-==-=-=-=");
    println!("*{name}: 5 rusty tokens, 0 assets");
    let new_save: Save = Save {
        name: name.to_string(),
        balance: 5,
        items: Vec::new(),
        plants: Vec::new(),
        animals: Vec::new()
    };
    println!("=-==-=-==-===-=-==-=-==-==-=-===-=-==-");

    saves.push(new_save);
    return Some(saves.len() - 1);
}


// load a saved game and update curr save idx ( looped, has confirm )
fn load_game(player: &str, curr_save_idx: usize, saves: &mut Vec<Save>) -> Option<usize> {
    let mut input: String = String::new(); // user in
    let mut exists: bool; // if game exists
    
    print_load_help(curr_save_idx, saves); // type in a string

    loop {
        input.clear();
        exists = false;
        println!("\nLoad> Which save would you like to load, {player}?");
        io::stdin()
            .read_line(&mut input)
            .expect("Error: failed to read name");
        input = input.trim().to_string();

        if input.len() == 0 {
            continue;
        }

        if input == "c" || input == "cancel" { // cancel load save, exit the func
            return None;
        }
        if input == "h" || input == "help" {
            print_load_help(curr_save_idx, saves);
            continue;
        }
        if input == "v" || input == "view" {
            view_current(curr_save_idx, saves);
            continue;
        }
        if input == "a" || input == "all" {
            view_all(curr_save_idx, saves);
            continue;
        }

        if input.contains(' ') {
            println!("\nInvalid amount of arguments.");
            continue;
        }

        let mut i: usize = 0;
        for save in saves.iter() { // get the index of the save to load
            if input == save.name {
                exists = true;
                break;
            }
            i += 1;
        }

        if exists {
            println!("Are you sure you want to load {input}? (y/n)");
            if confirm_action() {
                println!("\n-===-=-==-=-= SAVE LOADED ==-==-=-=-==");
                println!("*{input}: {} rusty tokens, {1} assets", 
                    saves[i].balance, saves[i].items.len()+saves[i].plants.len()+saves[i].animals.len());
                println!("=-=-===-=-==-===--===-==-===-==-=-=-==");
                return Some(i);
            }
            else {

            }
        } 
        else {
            println!("Can't find \"{input}\". Try again.");
        }
    } // end of loop to get name of save to load
}


// load a saved game and update curr save idx ( no loop, no confirm )
fn load_game_quick(saves: &mut Vec<Save>, name: &str) -> Option<usize> {
    let mut exists: bool; // if game exists

    let mut i: usize = 0;
    exists = false;
    for save in saves.iter() { // get the index of the save to load
        if name == save.name {
            exists = true;
            break;
        }
        i += 1;
    }

    if exists {
        println!("\n-==-===-=-== SAVE LOADED =-==-=-=-=-==");
        println!("*{name}: {} rusty tokens, {1} assets", 
            saves[i].balance, saves[i].items.len()+saves[i].plants.len()+saves[i].animals.len());
        println!("-==-==-==-=-=-==-===-=-=-=-==-=-==-===");
        return Some(i);
    } 
    else {
        println!("Can't find \"{name}\".");
        return None;
    }
}


// remove a saved game and update curr save idx ( looped, has confirm )
// return -1: removed lower index, 
// return 0: removed curr save
// return 1: removed upper index
// return 9: didnt remove
fn remove_game(player: &str, curr_save_idx: &mut usize, saves: &mut Vec<Save>) {
    let mut input: String = String::new(); // user in
    let mut exists: bool; // if game exists
    let mut tmp: usize = curr_save_idx.clone();
    
    print_remove_help(tmp, saves); // type in a string

    loop {
        tmp = curr_save_idx.clone();
        input.clear();
        exists = false;
        println!("\nRemove> Which save would you like to remove, {player}?");
        io::stdin()
            .read_line(&mut input)
            .expect("Error: failed to read name");
        input = input.trim().to_string();

        if input.len() == 0 {
            continue;
        }

        if input == "c" || input == "cancel" { // cancel load save, exit the func
            return;
        }
        if input == "h" || input == "help" {
            print_remove_help(tmp, saves);
            continue;
        }
        if input == "v" || input == "view" {
            view_current(tmp, saves);
            continue;
        }
        if input == "a" || input == "all" {
            view_all(tmp, saves);
            continue;
        }

        if input.contains(' ') {
            println!("\nInvalid amount of arguments.");
            continue;
        }

        let mut i: usize = 0;
        for save in saves.iter() { // get the index of the save to remove
            if input == save.name {
                exists = true;
                break;
            }
            i += 1;
        }

        if exists {
            println!("Are you sure you want to remove {input}? (y/n)");
            if confirm_action() {
                if let Some(_save) = saves.get(i) {  // get &mut save 
                    if *curr_save_idx == 0 && i != *curr_save_idx {
                        println!("\n-===-=-===-= SAVE REMOVED ==-==-=-=-==");
                        println!("{input}: {} rusty tokens, {1} assets", 
                        saves[i].balance, saves[i].items.len() + saves[i].plants.len() + saves[i].animals.len());
                        println!("=-=-===-=-==-===--===-==-===-==-=-=-==");
                    }
                    else if i < *curr_save_idx {
                        println!("\n-===-=-===-= SAVE REMOVED ==-==-=-=-==");
                        println!("{input}: {} rusty tokens, {1} assets", 
                        saves[i].balance, saves[i].items.len() + saves[i].plants.len() + saves[i].animals.len());
                        println!("=-=-===-=-==-===--===-==-===-==-=-=-==");
                        *curr_save_idx -= 1;
                    }
                    else if i > *curr_save_idx {
                        println!("\n-===-=-===-= SAVE REMOVED ==-==-=-=-==");
                        println!("{input}: {} rusty tokens, {1} assets", 
                        saves[i].balance, saves[i].items.len() + saves[i].plants.len() + saves[i].animals.len());
                        println!("=-=-===-=-==-===--===-==-===-==-=-=-==");
                        *curr_save_idx += 1;
                    }
                    else {
                        println!("\n-===-=-===-= SAVE REMOVED ==-==-=-=-==");
                        println!("*{input}: {} rusty tokens, {1} assets", 
                        saves[i].balance, saves[i].items.len() + saves[i].plants.len() + saves[i].animals.len());
                        println!("=-=-===-=-==-===--===-==-===-==-=-=-==");
                        *curr_save_idx = std::usize::MAX;
                    }
                    saves.remove(i);
                }
            }
            else {
                continue;
            }
        } 
        else {
            println!("Can't find \"{input}\". Try again.");
        }
    } // end of loop to get name of save to remove
}


// remove a saved game and update curr save idx ( no loop, no confirm )
// return -1: removed lower index, 
// return 0: removed curr save
// return 1: removed upper index
// return 9: didnt remove
fn remove_game_quick(curr_save_idx: &mut usize, saves: &mut Vec<Save>, name: &str) {
    let mut exists: bool; // if game exists
    exists = false;

    let mut i: usize = 0;
    for save in saves.iter() { // get the index of the save to remove
        if name == save.name {
            exists = true;
            break;
        }
        i += 1;
    }

    if exists {
        if let Some(_save) = saves.get_mut(i) {  // get &mut save 
            if *curr_save_idx == 0 && i != *curr_save_idx {
                println!("\n-===-=-===-= SAVE REMOVED ==-==-=-=-==");
                println!("{name}: {} rusty tokens, {1} assets", 
                saves[i].balance, saves[i].items.len() + saves[i].plants.len() + saves[i].animals.len());
                println!("=-=-===-=-==-===--===-==-===-==-=-=-==");
            }
            else if i < *curr_save_idx {
                println!("\n-===-=-===-= SAVE REMOVED ==-==-=-=-==");
                println!("{name}: {} rusty tokens, {1} assets", 
                saves[i].balance, saves[i].items.len() + saves[i].plants.len() + saves[i].animals.len());
                println!("=-=-===-=-==-===--===-==-===-==-=-=-==");
                *curr_save_idx -= 1;
            }
            else if i > *curr_save_idx {
                println!("\n-===-=-===-= SAVE REMOVED ==-==-=-=-==");
                println!("{name}: {} rusty tokens, {1} assets", 
                saves[i].balance, saves[i].items.len() + saves[i].plants.len() + saves[i].animals.len());
                println!("=-=-===-=-==-===--===-==-===-==-=-=-==");
                *curr_save_idx += 1;
            }
            else {
                println!("\n-===-=-===-= SAVE REMOVED ==-==-=-=-==");
                println!("*{name}: {} rusty tokens, {1} assets", 
                saves[i].balance, saves[i].items.len() + saves[i].plants.len() + saves[i].animals.len());
                println!("=-=-===-=-==-===--===-==-===-==-=-=-==");
                *curr_save_idx = std::usize::MAX;
            }
            saves.remove(i);
        }
    } 
    else {
        println!("Can't find \"{name}\".");
        return;
    }
}


// flip a coin, guess heads or tails, win/lose back bet amount
fn coin(guess: &str, bet_amount: usize, balance: & mut isize) {
    let result: &str;

    let since_epoch: Duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let ms: u128 = since_epoch.as_millis() as u128;
    if ms % 2 == 0 {
        result = "Heads";
    } else {
        result = "Tails";
    }

    if guess == result {
        println!("\n{result}! You won {} rusty tokens!", bet_amount);
        *balance += bet_amount as isize;
    } else {
        println!("\n{result}! You lost {} rusty tokens.", bet_amount);
        *balance -= bet_amount as isize;
    }
}


// roll a dice, guess 1-6, win/lose guess*bet amount
fn dice(guess: usize, bet_amount: usize, balance: &mut isize) {
    let since_epoch: Duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let ms: usize = since_epoch.as_millis() as usize;
    
    let result: usize = ms % 6 + 1;
    if guess == result {
        println!("\nRolled a {}. You won {1} rusty tokens!", result, bet_amount* guess);
        *balance += bet_amount as isize*guess as isize;
    } else {
        println!("\nRolled a {}. You lost {1} rusty tokens!", result, bet_amount * guess);
        *balance -= bet_amount as isize*guess as isize;
    }
} 


// guess higher or lower, first iteration is 50, win/lose bet amount (option to loop win double/lose double)
fn next(first_guess: &str, bet: usize, balance: &mut isize) {
    let mut input: String = String::new();

    let mut curr_num: u8 = 50; // init to 50 for first iteration
    let mut result: &str;


    let mut confirm: bool; // confirm to win double or nothing
    let mut winnings: usize = bet;
    let mut guess: String = first_guess.to_string();

    loop {
        input.clear();

        let since_epoch: Duration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let ms: u128 = since_epoch.as_millis() as u128;
        let next_num: u128 = ms % 99 + 1; // 1-99

        if curr_num < next_num as u8 { 
            result = "Higher";
        }
        else if curr_num > next_num as u8 {
            result = "Lower";
        }
        else { // curr num is equal to next num
            result = "Safe";
        } 

        if result == guess || result == "Safe" {
            println!("\nNext is {}. You won {1} rusty tokens!", next_num, winnings);
            println!("Double or nothing (y/n)? Current balance: {balance} rusty tokens");
            confirm = confirm_action();
            if confirm {
                winnings *= 2;
                curr_num = next_num as u8;
                loop {
                    input.clear();
                    guess.clear();
                    println!("\nHigher or lower than {curr_num}? (h/l)");
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Error: Failed to read");
                    input = input.trim().to_string();

                    if input.len() == 0 {
                        continue;
                    }
                    else {
                        guess = input.trim().to_owned();
                    }
                    if guess == "h" || guess == "higher" || guess == "Higher" {
                        guess = "Higher".to_string();
                        break;
                    }
                    else if guess == "l" || guess == "lower" || guess == "Lower" {
                        guess = "Lower".to_string();
                        break;
                    }
                    else {
                        println!("\nInvalid command [{}]. Try again.", guess);
                    }

                }
            }
            else {
                *balance += winnings as isize;
                println!("\n-==-=-===-===-==-==-=-==-=-=-=");
                println!("New balance: {} rusty tokens", balance);
                println!("=-==-==-=-==-=-==-==-=--==-=-=");
                break;
            }

        }
        else {
            // println!("You guessed {guess}, but curr {curr_num} and next {next_num}");
            println!("\nNext is {}. You lost {1} rusty tokens.", next_num, winnings);
            *balance -= winnings as isize;
            println!("\n-==-=-===-===-==-==-=-==-=-=-=");
            println!("New balance: {} rusty tokens", balance);
            println!("=-==-==-=-==-=-==-==-=--==-=-=");

            break;
        }

    }
}   


// wager game menu, choose coin, dice, next ( looped, has confirm)
fn wager(player: &str, curr_save_idx: usize, saves: &mut Vec<Save>) {
    let mut input: String = String::new();
    let mut command: Vec<String> = Vec::new(); // game argument

    let game_types: Vec<&str> = vec!["coinh", "coint", "ch", "ct", 
                                     "dice1", "dice2", "dice3", "dice4", "dice5", "dice6", 
                                     "d1", "d2", "d3", "d4", "d5", "d6",
                                     "nexth", "nextl", "nh", "nl"];

    print_wager_help(); // usage: [game] [bet amount]
    
    loop {
        input.clear();
        command.clear();

        let mut coin_arg: Option<&str> = None; // slices of command, checks if arg is Some, execute that
        let mut coin_type: Option<&str> = None;
        let mut dice_arg: Option<&str> = None; 
        let mut dice_type: Option<&str> = None;
        let mut next_arg: Option<&str> = None;
        let mut next_type: Option<&str> = None;

        let save: &mut Save = &mut saves[curr_save_idx];
        println!("\nWager> Which game would you like to play, {player}?");
        io::stdin()
            .read_line(&mut input)
            .expect("Error: Failed to read");
        input = input.trim().to_string();

        if input.len() == 0 {
            continue;
        }
        else {
            command = input
                .split_whitespace()
                .map(String::from)
                .collect();
        }
        if command[0] == "c" || command[0] == "cancel" {
            return;
        }
        if command[0] == "h" || command[0] == "help" {
            print_wager_help();
            continue;
        }
        if input == "v" || input == "view" {
            view_current(curr_save_idx, saves);
            continue;
        }
        if input == "a" || input == "all" {
            view_all(curr_save_idx, saves);
            continue;
        }

        if command.len() != 2 {
            println!("\nInvalid amount of arguments.");
            println!("Usage: [game] [bet amount]");
            continue;
        }
        
        
        let game_type: String = (command[0])[0..(command[0].len()-1)].to_string();
        let game_type_arg: String = (command[0])[(command[0].len()-1)..(command[0].len())].to_string();

        if !game_types.contains(&command[0].as_str()) {
            if command[0].len() == 2 || command[0].len() == 5 {
                if game_type == "c" || game_type == "coin" {
                    println!("\nInvalid argument \"{game_type_arg}\" for \"coin\".");
                    println!("Choose from: h | t");
                    continue;
                } 
                else if game_type == "d" || game_type == "dice" {
                    println!("\nInvalid argument \"{game_type_arg}\" for \"dice\".");
                    println!("Choose from: 1 | 2 | 3 | 4 | 5 | 6");
                    continue;
                }
                else if game_type == "n" || game_type == "next" {
                    println!("\nInvalid argument \"{game_type_arg}\" for \"next\".");
                    println!("Choose from: h | l");
                    continue;
                }
                else {
                    println!("\nInvalid game {game_type}.");
                    if (&command[0])[0..1].to_string() == "c" {
                        println!("Did you mean \"cX\" or \"coinX\"?");
                        continue;
                    }
                    else if (&command[0])[0..1].to_string() == "d" {
                        println!("Did you mean \"dX\" or \"diceX\"?");
                        continue;
                    }
                    else if (&command[0])[0..1].to_string() == "n" {
                        println!("Did you mean \"nX\" or \"nextX\"?");
                        continue; 
                    }
                    else {
                        println!("Choose from: coinX (cX) | diceX (dX) | nextX (nX)");
                        continue;
                    }
                }
            }
            else {
                if (&command[0])[0..1].to_string() == "c" {
                    println!("Did you mean \"cX\" or \"coinX\"?");
                    continue;
                }
                else if (&command[0])[0..1].to_string() == "d" {
                    println!("Did you mean \"dX\" or \"diceX\"?");
                    continue;
                }
                else if (&command[0])[0..1].to_string() == "n" {
                    println!("Did you mean \"nX\" or \"nextX\"?");
                    continue; 
                }
                else {
                    println!("Invalid game {}.", command[0]);
                    println!("Choose from: coinX (cX) | diceX (dX) | nextX (nX)");
                    continue;
                }
            }
        }
        
        // get game data

        if &(command[0])[0..1] == "c" {
            coin_arg = Some(&game_type);
            coin_type = Some(&game_type_arg);
           
        }
        else if &(command[0])[0..1] == "d" {
            dice_arg = Some(&game_type);
            dice_type = Some(&game_type_arg); 
        }
        else if &(command[0])[0..1] == "n" {
            next_arg = Some(&game_type);
            next_type = Some(&game_type_arg); 
        }

        if let Ok(bet_amount) = command[1].parse::<usize>() { // bet amount is valid type
            let mut tmp_s: &str = "";
            let mut tmp_i: usize = 0;
            if bet_amount <= save.balance as usize && bet_amount > 0 { // bet amount is valid amount
                if let Some(ref _some) = coin_arg { 
                    if let Some(arg) = coin_type {
                        if arg == "h" {
                            tmp_s = "Heads";
                        } 
                        else {
                            tmp_s = "Tails";
                        }
                    }
                    println!("You have {} rusty tokens. \nBet {1} rusty tokens on {2}? (y/n)", 
                    save.balance, bet_amount, tmp_s);
                    if confirm_action() {
                        coin(&tmp_s, bet_amount, &mut save.balance);
                        print_balance(save);
                    }
                    else {
                        continue;
                    }
                } // end of coin game

                else if let Some(_some) = dice_arg {
                    if let Some(arg) = dice_type {
                        tmp_s = arg;
                        if let Ok(res) = tmp_s.parse::<usize>() {
                            tmp_i = res;
                        }
                    }

                    println!("You have {} rusty tokens. \nBet {1} rusty tokens on rolling a {2}? (y/n)", 
                    save.balance, bet_amount, tmp_i);
                    if confirm_action() {
                        dice(tmp_i, bet_amount, &mut save.balance);
                        print_balance(save);
                    }
                    else {
                        continue;
                    }
                } // end of dice game

                else if let Some(_some) = next_arg { 
                    if let Some(arg) = next_type {
                        if arg == "h" {
                            tmp_s = "Higher";
                        } 
                        else if arg == "l" {
                            tmp_s = "Lower";
                        }
                    }
                    println!("You have {} rusty tokens. \nBet {1} rusty tokens on {2} than 50? (y/n)", 
                    save.balance, bet_amount, tmp_s);
                    if confirm_action() {
                        next(&tmp_s.to_string(), bet_amount, &mut save.balance);
                    } 
                    else {
                        continue;
                    }
                } // end of next game
            } // end of betting
            else {
                println!("\nInvalid bet amount.");
                println!("You have {} rusty tokens.", save.balance);
                println!("Bet interval (0, {}]", save.balance);
            }
        }
        else {
            println!("\n{} is of invalid type. Try again.", command[1]);
        }
    } // end of playing game
}


// wager game menu, choose coin,dice,next ( no loop, no confirm )
fn wager_quick(save: &mut Save, game_type: String, bet_amount: usize) {
    let game_types: Vec<&str> = vec!["coinh", "coint", "ch", "ct", 
                                     "dice1", "dice2", "dice3", "dice4", "dice5", "dice6", 
                                     "d1", "d2", "d3", "d4", "d5", "d6",
                                     "nexth", "nextl", "nh", "nl"];

    let mut coin_arg: Option<&str> = None; 
    let mut coin_type: Option<&str> = None;
    let mut dice_arg: Option<&str> = None;
    let mut dice_type: Option<&str> = None;
    let mut next_arg: Option<&str> = None;
    let mut next_type: Option<&str> = None;
    
    // get game data
    let game_arg: String = (game_type)[0..(game_type.len()-1)].to_string();
    let game_type_arg: String = (game_type)[(game_type.len()-1)..(game_type.len())].to_string();

    if !game_types.contains(&game_type.as_str()) {
        if game_type.len() == 2 || game_type.len() == 5 {
            if game_arg == "c" || game_arg == "coin" {
                println!("\nInvalid argument \"{game_type_arg}\" for \"coin\".");
                println!("Choose from: h | t");
                return;
            } 
            else if game_arg == "d" || game_arg == "dice" {
                println!("\nInvalid argument \"{game_type_arg}\" for \"dice\".");
                println!("Choose from: 1 | 2 | 3 | 4 | 5 | 6");
                return;
            }
            else if game_arg == "n" || game_arg == "next" {
                println!("\nInvalid argument \"{game_type_arg}\" for \"next\".");
                println!("Choose from: h | l");
                return;
            }
            else {
                println!("\nInvalid game {game_type}.");
                    if &(game_type)[0..1] == "c" {
                        println!("Did you mean \"cX\" or \"coinX\"?");
                        return;
                    }
                    else if &(game_type)[0..1] == "d" {
                        println!("Did you mean \"dX\" or \"diceX\"?");
                        return;
                    }
                    else if &(game_type)[0..1] == "n" {
                        println!("Did you mean \"nX\" or \"nextX\"?");
                        return; 
                    }
                    else {
                        println!("Choose from: coinX (cX) | diceX (dX) | nextX (nX)");
                        return;
                    }
            }
        }
        else {
            if (game_type)[0..1].to_string() == "c" {
                println!("Did you mean \"cX\" or \"coinX\"?");
                return;
            }
            else if (game_type)[0..1].to_string() == "d" {
                println!("Did you mean \"dX\" or \"diceX\"?");
                return;
            }
            else if (game_type)[0..1].to_string() == "n" {
                println!("Did you mean \"nX\" or \"nextX\"?");
                return; 
            }
            else {
                println!("\nInvalid game {game_type}.");
                println!("Choose from: coinX (cX) | diceX (dX) | nextX (nX)");
                return;
            }
        }
    }
    
    // get game data

    if &(game_type)[0..1] == "c" {
        coin_arg = Some(&game_type);
        coin_type = Some(&game_type_arg);
    }
    else if &(game_type)[0..1] == "d" {
        dice_arg = Some(&game_type);
        dice_type = Some(&game_type_arg); 
    }
    else if &(game_type)[0..1] == "n" {
        next_arg = Some(&game_type);
        next_type = Some(&game_type_arg); 
    }

    if bet_amount as isize <= save.balance && bet_amount > 0 { // bet amount is valid amount
        let mut tmp_s: &str = "";
        let mut tmp_i: usize = 0;
        if let Some(ref _some) = coin_arg { 
            if let Some(arg) = coin_type {
                if arg == "h" {
                    tmp_s = "Heads";
                } 
                else {
                    tmp_s = "Tails";
                }
            }
            coin(&tmp_s, bet_amount, &mut save.balance);
            print_balance(save);
            return;
        } // end of coin game

        else if let Some(_some) = dice_arg {
            if let Some(arg) = dice_type {
                tmp_s = arg;
                if let Ok(res) = tmp_s.parse::<usize>() {
                    tmp_i = res;
                }
            }
            dice(tmp_i, bet_amount, &mut save.balance);
            print_balance(save);
            return;
        } // end of dice game

        else if let Some(ref _some) = next_arg { 
            if let Some(arg) = next_type {
                if arg == "h" {
                    tmp_s = "Higher";
                } 
                else if arg == "l" {
                    tmp_s = "Lower";
                }
            }
            next(&tmp_s, bet_amount, &mut save.balance);
        } // end of next game
    } // end of betting
    else {
        println!("\nInvalid bet amount.");
        println!("You have {} rusty tokens.", save.balance);
        println!("Bet interval [1, {}]", save.balance);
    }
}


// market menu, buy/sell items, ( looped, has confirm )
fn market(player: &str, curr_save_idx: usize, saves: &mut Vec<Save>) {
    let mut input: String = String::new();
    let mut command: Vec<String> = Vec::new();
    
    choose_market(saves, curr_save_idx);

    let items: Vec<&str> = vec![
        "starter_fertilizer", "starter_feed", "rusty_sword", "wooden_shield", "leather_armor",
        "efficient_fertilizer", "efficient_feed", "steel_dagger", "pocket_tower", "steel_armor",
        "masterwork_fertilizer", "masterwork_feed", "eternal_saber", "infinite_aegis", "world_armor"
    ];

    let plants: Vec<&str> = vec![
        "wild_grass", "autumn_leaf", "coral_fern", "poison_vine", "morning_oak", "bonsai_tree",
        "bluesilver_grass", "giant_bamboo", "snapdragon", "firebloom", "golden_moss", "white_willow",
        "astral_grass", "moonflower", "phoenix_blossom", "crystal_lotus", "world_tree"
    ];

    let animals: Vec<&str> = vec![
        "wild_chicken", "striped_duck", "ironback_snail", "silver_rabbit", "steeltusk_boar",
        "opal_turkey", "falling_squirrel", "gilded_fox", "silk_panther", "frostwing_falcon", "lunar_owl",
        "golden_goose", "arcane_griffin", "soaring_phoenix", "dream_dragon", "nightmare_dragon", "world_serpent"
    ];

    let mut cart_total: usize = 0; // total _buying cost
    let mut shopping_cart: Vec<Product> = Vec::new();

    loop {
        input.clear();
        command.clear();

        let save: &mut Save = &mut saves[curr_save_idx];
        println!("\nMarket> What would you like to do, {player}?");
        io::stdin()
            .read_line(&mut input)
            .expect("Error: Failed to read");
        input = input.trim().to_string();

        // check if user just pressed enter
        if input.len() == 0 {
            continue;
        }
        else {
            command = input
                .split_whitespace()
                .map(String::from)
                .collect();
        }

        // command: [op] [amount] [product]
        if command.len() == 1 {
            if command[0] == "c" || command[0] == "cancel" {
                return;
            }
            else if command[0] == "h" || command[0] == "help" {
                choose_market(saves, curr_save_idx);
                continue;
            }
            else if command[0] == "v" || command[0] == "view" {
                view_current(curr_save_idx, &saves);
                continue;
            }
            else if command[0] == "a" || command[0] == "all" {
                view_all(curr_save_idx, &saves);
                continue;
            }
            else if command[0] == "cart" { // edit the cart
                if shopping_cart.is_empty() {
                    println!("\nShopping cart is empty.");
                    continue;
                }
                print_cart_help(&shopping_cart);
                let mut tmp_in: String = String::new();
                let mut tmp_cmmd: Vec<String> = Vec::new();
                loop {
                    if shopping_cart.is_empty() {
                        println!("\nShopping cart is empty. Returning to Market.");
                        break;
                    }
                    tmp_in.clear();
                    tmp_cmmd.clear();
                    // product *displays* [amount] [tag] 
                    println!("\nCart> What would you like to do, {player}?");
                    io::stdin()
                        .read_line(&mut tmp_in)
                        .expect("Error: Failed to read");
                    tmp_in = tmp_in.trim().to_string();
                    if tmp_in.len() == 0 {
                        continue;
                    }
                    else {
                        tmp_cmmd = tmp_in
                            .split_whitespace()
                            .map(String::from)
                            .collect();
                    }
                    match tmp_cmmd[0].as_str() {
                        "r" | "remove" => {
                            if tmp_cmmd.len() == 3 {
                                // check amount
                                let mut _amount: usize;
                                if let Ok(i) = tmp_cmmd[1].parse::<usize>() {
                                    _amount = i;
                                }
                                else {
                                    println!("\nInvalid amount type {}.", tmp_cmmd[1]);
                                    continue;
                                }

                                // check product
                                let mut found: bool = false;
                                let mut i: usize = 0;
                                for product in shopping_cart.iter_mut() {
                                    let cost = Product::cost(&product.tag) * _amount;
                                    if product.tag == tmp_cmmd[2] {
                                        if _amount > product.amount {
                                            println!("\nCan't remove x{_amount} {}.", product.tag);
                                        }
                                        else {
                                            if product.amount - _amount == 0 {
                                                shopping_cart.remove(i);
                                            }
                                            else {
                                                product.amount -= _amount;
                                            }
                                            cart_total -= cost;
                                            print_cart_help(&shopping_cart);
                                        }
                                        found = true;
                                        break;
                                    }
                                    i += 1;
                                } 
                                if !found {
                                    println!("\n{} is not in your shopping cart.", tmp_cmmd[2]);
                                    continue;
                                }
                            }
                            else {
                                println!("\nInvalid amount of arguments for \"remove\".");
                                println!("Usage: [r or remove] [amount] [product]");
                                continue;
                            }
                        },

                        "c" | "cancel" => {
                            if tmp_cmmd.len() == 1 {
                                break;
                            }
                            else {
                                println!("\nInvalid amount of arguments for \"cancel\"");
                                println!("Usage: [c or cancel]");
                                continue;
                            }
                        },

                        "h" | "help" => {
                            if tmp_cmmd.len() == 1 {
                                print_cart_help(&shopping_cart);
                                continue;
                            }
                            else {
                                println!("\nInvalid amount of arguments for \"help\"");
                                println!("Usage: [h or help]");
                                continue;
                            }
                        },

                        _ => {
                            println!("\nInvalid command [{}].", tmp_cmmd[0]);
                            if &(tmp_cmmd[0])[0..1] == "c" {
                                println!("Did you mean \"c\" or \"cancel\"?");
                                continue;
                            }
                            if &(tmp_cmmd[0])[0..1] == "r" {
                                println!("Did you mean \"r\" or \"remove\"?");
                                continue;
                            }
                            if &(tmp_cmmd[0])[0..1] == "h" {
                                println!("Did you mean \"h\" or \"help\"?");
                                continue;
                            }
                        },
                    }
                } // end of cart loop
            } // end of command[0] == "cart"
            else if command[0] == "checkout" {
                if shopping_cart.is_empty() {
                    println!("\nShopping cart is empty.");
                    continue;
                }
                if cart_total > save.balance as usize {
                    println!("\nYou don't have enough rusty tokens for this purchase.");
                    println!("Remove items from your [cart] if you wish to proceed.");
                    continue;
                }
                else { // checkout 
                    print_cart_help(&shopping_cart);
                    println!("\nAre you sure you want to checkout? (y/n)");
                    if confirm_action() {
                        save.balance -= cart_total as isize;
                        for product in shopping_cart.iter() {
                            if &product.kind == "Item" {
                                save.items.push(Item { tag: product.tag.clone(), amount: product.amount });
                            }
                            else if &product.kind == "Plant" {
                                save.plants.push(Plant { tag: product.tag.clone(), amount: product.amount });
                            }
                            else if &product.kind == "Animal" {
                                save.animals.push(Animal { tag: product.tag.clone(), amount: product.amount });
                            }
                            else {
                                println!("\n@@@UNEXPECTED ERROR@@@");
                                continue;
                            }
                        }
                        println!("\nThank you for your purchase of {cart_total} rusty tokens!");
                        print_balance(save);
                        return;
                    }
                    else {
                        continue;
                    }
                }
            } // end of checkout

            else {
                println!("\nInvalid command {}.", command[0]);
                if &(command[0])[0..1] == "b" {
                    println!("Did you mean \"b\" or \"buy\"?");
                    println!("Usage: [b or buy] [amount] [product]");
                }
                else if &(command[0])[0..1] == "s" {
                    println!("Did you mean \"s\" or \"sell\"?");
                    println!("Usage: [s or sell] [amount] [product]");
                }
                else if &(command[0])[0..1] == "h" {
                    println!("Did you mean \"h\" or \"help\"?");
                    println!("Usage: [h or help]");
                }
                else if &(command[0])[0..1] == "v" {
                    println!("Did you mean \"v\" or \"view\"?");
                    println!("Usage: [v or view]");
                }
                else if &(command[0])[0..1] == "a" {
                    println!("Did you mean \"a\" or \"all\"?");
                    println!("Usage: [a or all]");
                }
                else if &(command[0])[0..2] == "ca" {
                    println!("Did you mean \"cancel\" or \"cart\"?");
                    println!("Usage: [c or cancel] or [cart]");
                }
                else if &(command[0])[0..2] == "ch" {
                    println!("Did you mean \"checkout\"?");
                    println!("Usage: [checkout]");
                }
                else if &(command[0])[0..2] == "c" {
                    println!("Did you mean \"c\" or \"cancel\"?");
                    println!("Usage: [c or cancel]");
                }
            }
            continue;
        } // end of command.len() == 1
        else if command.len() != 3 {
            println!("\nInvalid amount of arguments.");
            println!("Type \"h\" for help.");
            continue;
        }
        
        // check operation
        let mut _buying: bool = false;
        match command[0].as_str() {
            "buy" | "b" => _buying = true,
            "sell" | "s" => _buying = false,
            _ => {
                println!("\n{} is not a valid operation.", command[0]);
                println!("Choose \"buy\" (b) or \"sell\" (s)");
                continue;
            }
        }
        
        // check amount
        let mut _amount: usize;
        if let Ok(i) = command[1].parse::<usize>() {
            _amount = i;
        }
        else {
            println!("\nInvalid amount type \"{}\".", command[1]);
            continue;
        }

        // product exist?
        if !items.contains(&command[2].as_str()) 
        && !plants.contains(&command[2].as_str()) 
        && !animals.contains(&command[2].as_str()) {
            println!("\n\"{}\" is not a valid product.", command[2]);
            println!("Type \"h\" for help.");
            continue;
        }
        
        // product buy: add it to shopping cart and amount of product to total cost
        // product sell: sell amount of product for half price u bought it
        else { // product is valid
            // command: [op] [amount] [type]
            let cost: usize = Product::cost(&command[2]) * _amount;
            
            if _buying {
                println!("\n-==-=-===-===-==-==-=-==-=-=-=");
                println!("Added x{_amount} {} to cart.", command[2]);
                println!("=-==-==-=-==-=-==-==-=--==-=-=");
                let mut found: bool = false;
                for product in shopping_cart.iter_mut() {
                    if product.tag == command[2] {
                        product.amount += _amount;
                        found = true;
                        break;
                    }
                }
                cart_total += cost;
                if !found {
                    let mut s: &str = "";
                    if items.contains(&command[2].as_str()) { s = "Item"; }
                    else if plants.contains(&command[2].as_str()) { s = "Plant"; }
                    else if animals.contains(&command[2].as_str()) { s = "Animal"; }
                    shopping_cart.push(Product { tag: command[2].clone(), kind: s.to_string(), amount: _amount });
                }
                if cart_total > save.balance as usize {
                    println!("\nWarning: You don't have enough rusty tokens for this purchase.");
                }
            }
            else { // selling
                if save.items.is_empty() && save.plants.is_empty() && save.animals.is_empty() {
                    println!("\nNothing to sell.");
                    continue;
                }
                else {
                    let mut i: usize;
                    if items.contains(&command[2].as_str()) {
                        i = 0;
                        for item in save.items.iter_mut() {
                            if item.tag == command[2] {
                                if _amount > item.amount {
                                    println!("Can't sell {_amount} {}, you only have {1}.", command[2], item.amount);
                                    continue;
                                }
                                else {
                                    println!("\nSell {_amount} {} for {1} rusty tokens? (y/n)", command[2], cost/2);
                                    if confirm_action() {
                                        save.balance += (cost/2) as isize;
                                        if item.amount - _amount == 0 {
                                            save.items.remove(i);
                                        }   
                                        else {
                                            item.amount -= _amount;
                                        }
                                        print_balance(save);
                                    } 
                                    else { break; }  
                                }
                                break;
                            }
                            i += 1;
                        }
                        continue;
                    }
                    
                    if plants.contains(&command[2].as_str()) {
                        i = 0;
                        for plant in save.plants.iter_mut() {
                            if plant.tag == command[2] {
                                if _amount > plant.amount {
                                    println!("\nCan't sell x{_amount} {}, you only have x{1}.", command[2], plant.amount);
                                    continue;
                                }
                                else {
                                    println!("\nSell x{_amount} {} for {1} rusty tokens? (y/n)", command[2], cost/2);
                                    if confirm_action() {
                                        save.balance += (cost/2) as isize;
                                        if plant.amount - _amount == 0 {
                                            save.plants.remove(i);
                                        }   
                                        else {
                                            plant.amount -= _amount;
                                        }
                                        print_balance(save);
                                    } 
                                    else { break; }  
                                }
                                break;
                            }
                            i += 1;
                        }
                        continue;
                    }

                    if animals.contains(&command[2].as_str()) {
                        i = 0;
                        for animal in save.animals.iter_mut() {
                            if animal.tag == command[2] {
                                if _amount > animal.amount {
                                    println!("Can't sell {_amount} {}, you only have {1}.", command[2], animal.amount);
                                    continue;
                                }
                                else {
                                    println!("\nSell {_amount} {} for {1} rusty tokens? (y/n)", command[2], cost/2);
                                    if confirm_action() {
                                        save.balance += (cost/2) as isize;
                                        if animal.amount - _amount == 0 {
                                            save.animals.remove(i);
                                        }   
                                        else {
                                            animal.amount -= _amount;
                                        }
                                        print_balance(save);
                                    } 
                                    else { break; }  
                                }
                                break;
                            }
                            i += 1;
                        }
                        continue;
                    }    
                }
            }   
        }
    }
}


// enter dungeon to find chests, kill mobs for tokens
fn dungeon(player: &str, curr_save_idx: usize, saves: &mut Vec<Save>){
    println!("\n{player} has entered the Dungeon...");
    
    let save: &mut Save = &mut saves[curr_save_idx];
    
    let mut hit_rate: Vec<isize> = vec![1]; // if hit the number in this vector, gain hit_tokens
    let mut hit_tokens: isize = 1;
    
    let mut layers: isize = 3; // number of iterations

    let mut total_tokens: isize = 0;

    // get player data
    for item in save.items.iter() {
        if item.tag == "rusty_sword" {
            hit_tokens += item.amount as isize;
            hit_rate = vec![1,2];
        }
        if item.tag == "steel_dagger" {
            hit_tokens += 5*item.amount as isize;
            hit_rate = vec![1,2,3];
        }
        if item.tag == "eternal_saber" {
            hit_tokens += 25*item.amount as isize;
            hit_rate = vec![1,2,3,4];
        }
        if item.tag == "wooden_shield" {
            layers += item.amount as isize;
        }   
        if item.tag == "leather_armor" {
            layers += 5*item.amount as isize;
        }
        if item.tag == "pocket_tower" {
            layers += 10*item.amount as isize;
        }
        if item.tag == "steel_armor" {
            layers += 15*item.amount as isize;
        }
        if item.tag == "infinte_aegis" {
            layers += 20*item.amount as isize;
        }
        if item.tag == "world_armor" {
            layers += 25*item.amount as isize;
        }
    }

    let common_chest_rarity: u8 = 3;
    let common_chest: isize = 5000;
    let uncommon_chest_rarity: u8 = 5;
    let uncommon_chest: isize = 50000;
    let rare_chest_rarity: u8 = 7;
    let rare_chest: isize = 500000;
    let epic_chest_rarity: u8 = 9;
    let epic_chest: isize = 5000000;
    let legendary_chest_rarity: u8 = 11;
    let legendary_chest: isize = 500000000;
    let heavenly_chest_rarity: u8 = 13;
    let heavenly_chest: isize = 50000000000;

    let mut cur_num: isize = 0;
    let mut _hit_num: isize = 0;
    let mut i: isize = 0;
    let mut killed_first: bool = false;
    let mut concurrent_nums: u8 = 0;

    loop {
        let since_epoch: Duration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let ms: u128 = since_epoch.as_millis() as u128;
        
        _hit_num = (ms % 4 + 1) as isize; // 1-99
        if cur_num != _hit_num {
            let lost_tokens: isize = _hit_num as isize * hit_tokens as isize / 2;
            if concurrent_nums < common_chest_rarity {
                if _hit_num == 1 || _hit_num == 2 {
                    println!("\n** You took some damage! **");
                    println!("** Dropped {lost_tokens} rusty tokens. **");
                    total_tokens -= lost_tokens;
                }
            }
            else if concurrent_nums >= common_chest_rarity && concurrent_nums < uncommon_chest_rarity {
                if _hit_num == 1 || _hit_num == 2 {
                    println!("\n** You took some damage! **");
                    println!("** Dropped {lost_tokens} rusty tokens. **");
                    total_tokens -= lost_tokens;
                }
                else {
                    println!("\n** You found a common chest! **");
                    println!("** Obtained {common_chest} rusty tokens. **");
                    total_tokens += common_chest;
                }
            }
            else if concurrent_nums >= uncommon_chest_rarity && concurrent_nums < rare_chest_rarity {
                if _hit_num == 1 || _hit_num == 2 {
                    println!("\n** You took some damage! **");
                    println!("** Dropped {} rusty tokens. **", lost_tokens*10);
                    total_tokens -= lost_tokens*10;
                }
                else {
                    println!("\n** You found a uncommon chest! **");
                    println!("** Obtained {uncommon_chest} rusty tokens. **");
                    total_tokens += uncommon_chest;
                }
            }
            else if concurrent_nums >= rare_chest_rarity && concurrent_nums < epic_chest_rarity {
                if _hit_num == 1 || _hit_num == 2 {
                    println!("\n** You took serious damage! **");
                    println!("** Dropped {} rusty tokens. **", lost_tokens*100);
                    total_tokens -= lost_tokens*100;
                }
                else {
                    println!("\n** You found a rare chest! **");
                    println!("** Obtained {rare_chest} rusty tokens. **");
                    total_tokens += rare_chest;
                }
            }
            else if concurrent_nums >= epic_chest_rarity && concurrent_nums < legendary_chest_rarity {
                if _hit_num == 1 || _hit_num == 2 {
                    println!("\n** You took some damage! **");
                    println!("** Dropped {} rusty tokens. **", lost_tokens*1000);
                    total_tokens -= lost_tokens*1000;
                }
                else {
                    println!("\n** You found a epic chest! **");
                    println!("** Obtained {epic_chest} rusty tokens. **");
                    total_tokens += epic_chest;
                }
            }
            else if concurrent_nums >= legendary_chest_rarity && concurrent_nums < heavenly_chest_rarity {
                if _hit_num == 1 || _hit_num == 2 {
                    println!("\n** You took some damage! **");
                    println!("** Dropped {} rusty tokens. **", lost_tokens*100000);
                    total_tokens -= lost_tokens*100000;
                }
                else {
                    println!("\n** You found a legendary chest! **");
                    println!("** Obtained {legendary_chest} rusty tokens. **");
                    total_tokens += legendary_chest;
                }
            }
            else {
                if _hit_num == 1 || _hit_num == 2 {
                    println!("\n** You took some damage! **");
                    println!("** Dropped {} rusty tokens. **", lost_tokens*10000000);
                    total_tokens -= lost_tokens*10000000;
                }
                else {
                    println!("\n** You found a heavenly chest! **");
                    println!("** Obtained {heavenly_chest} rusty tokens. **");
                    total_tokens += heavenly_chest;
                }
            }
            concurrent_nums = 0;
        }

        if i > layers {
            save.balance += total_tokens as isize;
            println!("\n{player} has sustained too much damage.");
            if total_tokens > 0 {
                println!("+{total_tokens} rusty tokens.");
            }
            else {
                println!("{total_tokens} rusty tokens.");
            }
            println!("Leaving the Dungeon...");
            println!("-==-=-=-=-=-==-=-=--=-===-==-=-==-=-=-=");
            println!("New balance: {} rusty tokens", save.balance);
            println!("=-==-==-=-==-=----=--==-==-=--=--==-=-=");
            break;
        }

        let randomize: isize;
        if _hit_num < cur_num {
            randomize = _hit_num as isize * hit_tokens as isize / 2;
        }
        else {
            randomize = -(_hit_num as isize * hit_tokens as isize / 2);
        }

        if hit_rate.contains(&_hit_num) {
            if !killed_first {
                println!("-==-=-=-=-=-==-=-=--=-===-==-=-==-=-=-=");
                killed_first = true;
            }
            let obtained: isize = (hit_tokens * _hit_num) + randomize;
            total_tokens += obtained;
            println!("* Killed a mob for {} rusty tokens", obtained)
        }
        

        if cur_num == _hit_num {
            concurrent_nums += 1;
        }

        cur_num = _hit_num;
        i += 1;
    }

}


// update balance at beginning of game loop
fn update_balance(curr_save_idx: usize, saves: &mut Vec<Save>) {
    if !saves.is_empty() {
        let save: &mut Save = &mut saves[curr_save_idx];
    
        let mut plant_multiplier: isize = 1;
        let mut animal_multiplier: isize = 1;
    
        if !save.items.is_empty() {
            for item in save.items.iter() {
                if item.tag == "starter_fertilizer" {
                    plant_multiplier += item.amount as isize;
                }
                if item.tag == "starter_feed" {
                    animal_multiplier += item.amount as isize;
                }
                if item.tag == "efficent_fertilizer" {
                    plant_multiplier += 5*item.amount as isize;
                }   
                if item.tag == "efficient_feed" {
                    animal_multiplier += 5*item.amount as isize;
                }
                if item.tag == "efficent_fertilizer" {
                    plant_multiplier += 25*item.amount as isize;
                }   
                if item.tag == "efficient_feed" {
                    animal_multiplier += 25*item.amount as isize;
                }
            }
        }
    
        if !save.plants.is_empty() {
            for plant in save.plants.iter() {
                let cost: isize = Product::cost(&plant.tag) as isize;
                save.balance += cost * plant_multiplier / 25;
            }
        }
    
        if !save.animals.is_empty() {
            for animal in save.animals.iter() {
                let cost = Product::cost(&animal.tag) as isize;
                save.balance += cost * animal_multiplier / 25;
            }
        }
    }
}


// items, plants, animals
struct Product {
    tag: String,
    kind: String,
    amount: usize,
}
// function to get the cost of a product given its tag
impl Product {
    fn cost(product: &str) -> usize {
        let base_item_cost: usize = 30;
        let base_plant_cost: usize = 25;
        let base_animal_cost: usize = 35;
        match product {
            "starter_fertilizer" => base_item_cost,
            "starter_feed" => base_item_cost,
            "rusty_sword" => base_item_cost*5*4,
            "wooden_shield" => base_item_cost*5*7,
            "leather_armor" => base_item_cost*5*12,

            "efficient_fertilizer" => base_item_cost*50,
            "efficient_feed" => base_item_cost*50,
            "steel_dagger" => base_item_cost*50*7,
            "pocket_tower" => base_item_cost*50*10,
            "steel_armor" => base_item_cost*50*15,

            "masterwork_fertilizer" => base_item_cost*500*500,
            "masterwork_feed" => base_item_cost*500*500,
            "eternal_saber" => base_item_cost*500*500*10,
            "infinite_aegis" => base_item_cost*500*500*17,
            "world_armor" => base_item_cost*500*500*23,



            "wild_grass" => base_plant_cost, 
            "autumn_leaf" => base_plant_cost*5*3,
            "coral_fern" => base_plant_cost*5*6,
            "poison_vine" => base_plant_cost*5*9,
            "morning_oak" => base_plant_cost*5*12,
            "bonsai_tree" => base_plant_cost*5*15,

            "bluesilver_grass" => base_plant_cost*50,
            "giant_bamboo" => base_plant_cost*50*4,
            "snapdragon" => base_plant_cost*50*8,
            "firebloom" => base_plant_cost*50*12,
            "golden_moss" => base_plant_cost*50*16,
            "white_willow" => base_plant_cost*50*20,

            "astral_grass" => base_plant_cost*500*500,
            "moonflower" => base_plant_cost*500*500*5,
            "phoenix_blossom" => base_plant_cost*500*500*10,
            "crystal_lotus" => base_plant_cost*500*500*15,
            "world_tree" => base_plant_cost*500*500*23,



            "wild_chicken" => base_animal_cost,
            "striped_duck" => base_animal_cost*5*3,
            "ironback_snail" => base_animal_cost*5*6,
            "silver_rabbit" => base_animal_cost*5*9,
            "steeltusk_boar" => base_animal_cost*5*12,

            "opal_turkey" => base_animal_cost*50,
            "falling_squirrel" => base_animal_cost*50*4,
            "gilded_fox" => base_animal_cost*50*8,
            "silk_panther" => base_animal_cost*50*12,
            "frostwing_falcon" => base_animal_cost*50*16,
            "lunar_owl" => base_animal_cost*50*20,

            "golden_goose" => base_animal_cost*500*500,
            "arcane_griffin" => base_animal_cost*500*500*5,
            "soaring_phoenix" => base_animal_cost*500*500*10,
            "dream_dragon" => base_animal_cost*500*500*15,
            "nightmare_dragon" => base_animal_cost*500*500*20,
            "world_serpent" => base_animal_cost*500*500*25,

            _ => {
                println!("\nProduct not found error9812");
                0
            }
        }
    }
}
// implement display to print out the product ( [amount] [tag] )
impl fmt::Display for Product {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x{} {1}", self.amount, self.tag)?;
        Ok(())
    }

}

// buy/sellables
struct Item {
    tag: String,
    amount: usize,
}
struct Plant {
    tag: String,
    amount: usize,
}
struct Animal {
    tag: String,
    amount: usize,
}

// hold name, balance, assets of a save instance
struct Save {
    name: String,
    balance: isize,
    items: Vec<Item>,
    plants: Vec<Plant>,
    animals: Vec<Animal>,
}
// implement display to print the save ( name, balance, assets )
impl fmt::Display for Save {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n-=-=-=-=- CURRENT SAVE -=-==-==-")?;
        writeln!(f, "*{}: {1} rusty tokens, {2} assets", 
        self.name, self.balance, self.items.len() + self.plants.len() + self.animals.len())?;

        writeln!(f, "\n--Items--")?;
        if self.items.is_empty() {
            writeln!(f, "None")?;
        }
        else {
            for i in 0..self.items.len() {
                if let Some(item) = self.items.get(i) {
                    writeln!(f, "x{} {1}", item.amount, item.tag)?;
                }
            } 
            
        }
        
        writeln!(f, "\n--Plants--")?;
        if self.plants.is_empty() {
            writeln!(f, "None")?;
        }
        else {
            for i in 0..self.plants.len() {
                if let Some(plant) = self.plants.get(i) {
                    writeln!(f, "x{} {1}", plant.amount, plant.tag)?;
                }
            } 
        }

        writeln!(f, "\n--Animals--")?;
        if self.animals.is_empty() {
            writeln!(f, "None")?;
        }
        else {
            for i in 0..self.animals.len() {
                if let Some(animal) = self.animals.get(i) {
                    writeln!(f, "x{} {1}", animal.amount, animal.tag)?;
                }
            } 
        }
        write!(f, "\n-=-==-=-=-=-==-=-=-==-===-=-==-=")?;
        Ok(())
    }
}


fn main() {
    println!("You have entered the /*&@# \\% $*&@-(|.");
    let mut input: String = String::new();
    let mut command: Vec<String> = Vec::new();
    let player_name: String = welcome(); // welcome msg
    let mut saves: Vec<Save> = Vec::new(); // hold all saves
    let mut curr_save_idx: usize = std::usize::MAX; // hold current save idx
    
    
    // main game loop   
    loop {
        // check if valid is >= 0
        update_balance(curr_save_idx, &mut saves);
        check_valid_balance(&mut curr_save_idx, &mut saves); 

        input.clear();
        command.clear();

        println!("\n\n\nMain> What would you like to do next?");

        io::stdin()
            .read_line(&mut input)
            .expect("Error: failed to read command");
        input = input.trim().to_string();

        // check empty command ( just enter pressed )
        if input.len() == 0 {
            continue;
        }
        else {
            command = input
                .split_whitespace()
                .map(String::from)
                .collect();
        }

        match command[0].as_str() {
            "h" | "help" => print_main_help(),

            "c" | "cancel" => println!("Nothing to cancel."),

            "r" | "remove" => {
                if command.len() == 1 {
                    if curr_save_idx == std::usize::MAX && saves.is_empty() {
                        println!("No games found. Please create a new game.");
                    } 
                    else {
                        remove_game(&player_name, &mut curr_save_idx, &mut saves);
                    }
                }
                else if command.len() == 2 {
                    if curr_save_idx == std::usize::MAX && saves.is_empty() {
                        println!("No games found. Please create a new game.");
                    } 
                    else {
                        remove_game_quick(&mut curr_save_idx, &mut saves, &command[1]);
                    }
                }
                else {
                    println!("\nInvalid arguments for \"remove\".");
                    println!("Safe usage: [r or remove]");
                    println!("Quick usage: [r] [save name]");
                }
            },

            "new" | "n" => {
                if command.len() == 1 {
                    if let Some(save_idx) = new_game(&player_name, curr_save_idx, &mut saves) {
                        curr_save_idx = save_idx;
                    }
                }
                else if command.len() == 2 {
                    if let Some(save_idx) = new_game_quick(&mut saves, &command[1]) {
                        curr_save_idx = save_idx;
                    }
                }
                else {
                    println!("\nInvalid arguments for \"new\".");
                    println!("Safe usage: [n or new]");
                    println!("Quick usage: [n] [save name]");
                }
            }, // end of new

            "load" | "l" => {
                if command.len() == 1 {
                    if curr_save_idx == std::usize::MAX && saves.is_empty() {
                        println!("No games found. Please create a new game.");
                    } 
                    else {
                        if let Some(load_idx) = load_game(&player_name, curr_save_idx, &mut saves) {
                            curr_save_idx = load_idx;
                        }
                    }
                }
                else if command.len() == 2 {
                    if curr_save_idx == std::usize::MAX && saves.is_empty() {
                        println!("No games found. Please create a new game.");
                    } 
                    else {
                        if let Some(load_idx) = load_game_quick(&mut saves, &command[1]) {
                            curr_save_idx = load_idx;
                        }
                    }
                }
                else {
                    println!("\nInvalid arguments for \"load\".");
                    println!("Safe usage: [l or load]");
                    println!("Quick usage: [l] [save name]");
                }
            }, // end of load

            "view" | "v" => {   
                if command.len() == 1 {
                    view_current(curr_save_idx, &saves);
                } 
                else {
                    println!("\nInvalid arguments for \"view\".");
                    println!("Usage: [v or view]");
                }
            }, // end of view

            "all" | "a" => {
                if command.len() == 1 {
                    view_all(curr_save_idx, &saves)
                } 
                else {
                    println!("\nInvalid arguments for \"all\".");
                    println!("Usage: [a or all]");
                }
            }, // end of all

            "wager" | "w" => {
                if command.len() == 1 {
                    if check_valid_save(curr_save_idx, &saves) {
                        // if let Some(save) = saves.get_mut(curr_save_idx) { // get &mut save 
                        wager(&player_name, curr_save_idx, &mut saves);
                        // }
                    }
                }
                else if command.len() == 3 {
                    if check_valid_save(curr_save_idx, &saves) {
                        if let Some(save) = saves.get_mut(curr_save_idx) { // get &mut save 
                            if let Ok(bet_amount) = command[2].parse::<usize>() {
                                wager_quick(save, command[1].to_string(), bet_amount);
                            }
                            else {
                                println!("\nInvalid bet amount {}", command[2]);
                            }
                        }

                    }
                }
                else {
                    println!("\nInvalid arguments for \"wager\".");
                    println!("Safe usage: [w or wager]");
                    println!("Quick usage: [w] [game] [bet amount]");
                }
            }, // end of wager

            "market" | "m" => {
                if command.len() == 1 {
                    if check_valid_save(curr_save_idx, &saves) {
                        market(&player_name, curr_save_idx, &mut saves); 
                    }
                }   
                else if command.len() == 3 {
                    
                }
                else {
                    println!("\nInvalid arguments for \"market\".");
                    println!("Safe usage: [m or market]");
                    println!("Quick usage: [mb or ms] [amount to buy/sell] [thing to buy/sell]");
                }
            }, // end of market

            "dungeon" | "d" => {
                if command.len() == 1 {
                    if check_valid_save(curr_save_idx, &saves) {
                        dungeon(&player_name, curr_save_idx, &mut saves);
                    }
                }
                else {
                    println!("\nInvalid arguments for \"dungeon\".");
                    println!("Usage: [d or dungeon]");
                }
            },

            "exit" | "e" => {
                if command.len() == 1 {
                    saves.drain(..);
                    println!("Goodbye.");
                    return;
                } else {
                    println!("\nInvalid arguments for \"exit\".");
                }
            }, // end of exit

            _ => {
                println!("\nInvalid command [{}]", command[0]);
                if &(command[0])[0..1] == "n" {
                    println!("Did you mean \"n\" or \"new\"?");
                }
                else if &(command[0])[0..1] == "l" {
                    println!("Did you mean \"l\" or \"load\"?");
                }
                else if &(command[0])[0..1] == "v" {
                    println!("Did you mean \"v\" or \"view\"?");
                }
                else if &(command[0])[0..1] == "r" {
                    println!("Did you mean \"r\" or \"remove\"?");
                }
                else if &(command[0])[0..1] == "a" {
                    println!("Did you mean \"a\" or \"all\"?");
                }
                else if &(command[0])[0..1] == "m" {
                    println!("Did you mean \"m\" or \"market\"?");
                }
                else if &(command[0])[0..1] == "w" {
                    println!("Did you mean \"w\" or \"wager\"?");
                }
                else if &(command[0])[0..1] == "e" {
                    println!("Did you mean \"e\" or \"exit\"?");
                }
                else if &(command[0])[0..1] == "d" {
                    println!("Did you mean \"d\" or \"dungeon\"?");
                }
            }
        }
    }

}
