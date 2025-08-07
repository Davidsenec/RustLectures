use std::{io, io::Write};
use rand::Rng;

struct Player {
    hp:i32, 
    stamina:i32,
    power:i32,
    gold:i32,
}

struct Enemy {
    name: String,
    hp:i32, 
    power:i32,
    gr:i32,
}

enum Direction {
    N,
    S,
    E,
    W,
}

enum Encounter {
    Nothing,
    Meat,
    Water,
    Herb,
    Enemy(Enemy),
}

fn enemy_encounter() -> Enemy {
    let mut rng = rand::thread_rng();
    let chance = rng.gen_range(0..=100);
    
    match chance {
        0..=59 => Enemy { 
            name: String::from("a fucking Rat"), 
            hp: 10, 
            power: 2, 
            gr: 10 
        },
        60..=89 => Enemy { 
            name: String::from("the Wolf"), 
            hp: 20, 
            power: 5, 
            gr: 20 
        },
        _ => Enemy { 
            name: String::from("The Mighty Boar"), 
            hp: 30, 
            power: 10, 
            gr: 30 
        }
    }
}

fn encounter(player: &mut Player) -> Encounter {
    let mut rng = rand::thread_rng();
    let e_chance = rng.gen_range(0..=100);
    
    match e_chance {
        0..=24 => {
            println!("You encounter nothing of interest.");
            Encounter::Nothing
        },
        25..=44 => {
            if player.hp < 100 {
                player.hp += 5;
                println!("You found some meat to eat. +5 HP");
                println!("You now have {} HP.", player.hp);
            } else {
                println!("You found some meat, but you are already at full health, what a waste.");
            }
            Encounter::Meat
        },
        45..=64 => {
            player.stamina += 3;
            println!("You found a stream of water to drink from. +2 stamina");
            println!("You now have {} Stamina.", player.stamina);
            Encounter::Water
        },
        65..=79 => {
            player.power += 1;
            println!("You found a special herb. You make a ointment with it.");
            println!("You now have {} Power", player.power);
            Encounter::Herb
        },
        _ => {
            Encounter::Enemy(enemy_encounter())
        },
    }
}

fn moving(player: &mut Player) -> Direction {
    println!("Choose a direction to move in traveler (n, s, e, w): ");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        _n => {
            println!("You move in the direction: {}", input.trim().to_uppercase());
            player.stamina -= 1;
            return Direction::N
        }
        _s => {
            println!("You move in the direction: {}", input.trim().to_uppercase());
            player.stamina -= 1;
            return Direction::S
        }
        _e => {
            println!("You move in the direction: {}", input.trim().to_uppercase());
            player.stamina -= 1;
            return Direction::E
        }
        _w => {
            println!("You move in the direction: {}", input.trim().to_uppercase());
            player.stamina -= 1;
            return Direction::W
        }
        _ => {
            println!("That's n not a direction so you move North");
            return Direction::N
        }
    }
}

fn combat(player: &mut Player, enemy: &mut Enemy) {
    while player.hp > 0 && enemy.hp > 0 {
        enemy.hp -= player.power;
        println!("You hit the {} for {} damage", enemy.name, player.power);
        if enemy.hp <= 0 {
            println!("You've defeated the {}", enemy.name);
            player.gold += enemy.gr;
            println!("You've gained {} gold!", enemy.gr);
        }
        player.hp -= enemy.power;
        println!("The enemy hits you for {}", enemy.power);
        if player.hp <= 0 {
            println!("You've been bested by {}", enemy.name);
        }

    }

}

fn main() {
    let mut Player = Player {
        hp: 100,
        stamina: 10,
        power: 10,
        gold: 0,
    };

    println!("Welcome traveler to Rustworld. Be weary of the scary creatures on this world like borrowing and lifetimes.");

    loop {
        println!("You have {} HP, {} Stamina, {} Power, and {} Gold.", 
                Player.hp, Player.stamina, Player.power, Player.gold);
        
        if Player.stamina <= 0 {
            println!("You've used up all your stamina traveler.");
            println!("You die of exhaustion.");
            break;
        }
        if Player.hp <= 0 {
            println!("You've lost all your precious life.");
            println!("You die a horrible death, via out of scope.");
            break;
        }
        if Player.gold >= 100 {
            println!("You've made enough money to escape from Rustworld, hopefully you can understand lifetimes now.");
            println!("Farewell traveler, may we see each other again in the next lab.");
            break;
        }

        moving(&mut Player);
        encounter(&mut Player);
        
    }
}