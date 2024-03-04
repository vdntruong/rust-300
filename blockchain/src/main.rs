#[macro_use]
extern crate serde_derive;

use std::io;
use std::io::Write;
use std::process;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();

    print!("input a miner address: ");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut miner_addr);

    print!("input difficulty: ");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("we need a integer");

    println!("generating genesis block!");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    let mut choice = String::new();
    loop {
        choice.clear();
        println!("Menu");
        println!("1. New transaction");
        println!("2. Mine block");
        println!("3. Change difficulty");
        println!("4. Change reward");
        println!("0. Exit!");
        print!("Enter command: ");
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting!");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                print!("enter sender address: ");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut sender);
                sender = sender.trim().to_string();

                let mut receiver = String::new();
                print!("enter receiver address: ");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut receiver);
                receiver = receiver.trim().to_string();

                let mut amount = String::new();
                print!("enter amount: ");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(sender, receiver, amount.trim().parse().unwrap());
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generation failed"),
                }
            }
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generaion failed"),
                }
            }
            3 => {
                let mut new_diff = String::new();
                print!("enter new difficulty: ");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Updated difficulty"),
                    false => println!("Failed update difficulty"),
                }
            }
            4 => {
                let mut new_reward = String::new();
                print!("Enter new reward: ");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut new_reward);
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Updated reward"),
                    false => println!("Failed update reward"),
                }
            }
            _ => println!("Invalid command please retry"),
        }
    }
}
