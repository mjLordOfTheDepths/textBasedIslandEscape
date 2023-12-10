
fn main() {
    let mut location: i32 = 0;
    let mut goblinCompanion: i8 = 0;
    let mut hp: i8 = 10;
    let mut inBattle: i8 = 0;
    let mut goblinLordHp: i8 = 10;
    let attack: i8 = 2;
    let mut rustySword: i8 = 0;
    let mut hasPotion:i8 = 0;
    let mut goblinSword: i8 = 0;
    let mut leftSouth:i8 = 0;
    let mut leftEast:i8 = 0;
    let mut pirateHp:i8 = 12;

    println!("Welcome adventurer, to the world of Phanglosien!\nYou, a humble merchant, appear to have crashed your ship on a remote Island. You don't remember much of last night. Strangly, you awake a few meters before the wreck on the East coast of the Island.\n\nWhat would you like to do?\n(Type 'help' for a list of possible actions)");

    while (location < 999) {

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "help" => {
                println!("Type 'North' to go North \nType 'South' to explore the South \nType 'West' to explore West \nType 'East' to go east \nType 'Quit' to exit");
            }

            "Quit" => {
                location = 999;
            }

            "North" => { println!("You decide to head North. You discover a cave filled with red tribal markings. You seem to recognise these scratchings as Goblonic Text, yet, you cannot read Goblonic. Would you like to enter? y/n");
            location = 1;
                while location == 1 {

                    if (goblinCompanion == 2 || goblinSword != 0) {
                        println!("You've done all that has do be done in this area.");
                        location = 0;
                    }

                    else {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        let input = input.trim();

                        if input == "n" {
                            location = 0;
                            println!("You head back to where you came from.")
                        }
                        else {
                            println!("You enter the cave. You hear water dripping from stalagtites onto the cold stony ground.");
                            println!("A goblin approaches clad in heavy fur attire.");
                            println!("Goblin: 'You want fight me? You enter cave of exiled Swordlord of the Goblin Horde to fight?' y/n");
                            let mut input = String::new();
                            std::io::stdin().read_line(&mut input).unwrap();
                            let input = input.trim();

                            if input == "n" {
                                println!("Goblin: 'Oh ok! You want be friend of Swordlord of the Goblin Horde? :3'");
                                println!("1. Yes, of course! I could use a fellowman on my quest to get off this Island!\n2. Get away from me you disgusting Goblonic freakazoid.");
                                let mut input = String::new();
                                std::io::stdin().read_line(&mut input).unwrap();
                                let input = input.trim();

                                if input == "2" {
                                    println!("Goblin: 'You mean... you just like ex-wife Arg-ox. :('");
                                    location = 0;
                                }

                                else {
                                    println!("Goblin: 'Yayyy!!!! Me help you get off Island. Sometimes Orc Pirate Arghg port on West of Island. You me go take ship!'\nYou head back to the wilderness with your new friend :)\n");
                                    goblinCompanion = 2;
                                }
                                
                            }

                            else {
                                println!("Goblin: 'Ok, mighty warrior! I fight you for my sword! You beat me, I teach you the secrets of the Sword of The Lord of the Goblin Horde, but know this; if you win you must take its knowledge to the grave. En Garde!'");
                                println!("The Swordlord of the Goblin Horde allows you the first hit. Type 'Hit' to attack, and if you have a health potion, type 'Heal' to regain all your health.");

                                inBattle = 1;

                                while inBattle == 1 {
                                    let mut input = String::new();
                                    std::io::stdin().read_line(&mut input).unwrap();
                                    let input = input.trim();

                                    match input {
                                        "Hit" => {
                                            goblinLordHp = goblinLordHp - (attack + rustySword);
                                            println!("You hit the Goblin! He has {} HP remaining!", goblinLordHp);
                                            hp = hp - 2;
                                            println!("You were hit back! You have {} HP remaining!", hp);

                                            if hp < 1 {
                                                println!("You died. Game over.");
                                                location = 999;
                                                inBattle = 0;
                                            }

                                            if (goblinLordHp < 1 && hp > 0) {
                                                println!("Goblin: 'You defeat me in combat! I teach you the ways of my sword.'\nThe Swordlord of the Goblin Horde gives you the Sword of the Swordlord of the Goblin Horde and trains you in its ways\n");
                                                goblinSword = 4;
                                                rustySword = 1;
                                                inBattle = 0;
                                                println!("You leave the Goblin Cave and return to where you started.");
                                                location = 0;
                                            }
                                        }
                                        "Heal" => {
                                            if hasPotion == 1 {
                                                hp = 10;
                                                println!("Your health has been restored!");
                                            }
                                            else {
                                                println!("You dont have a health potion.");
                                            }
                                        }

                                        _ => {
                                            println!("Invalid Input.");
                                        }
                                    }
                                }
                            }   

                        }
                    }

                }

            }

            "South" => { 

                if leftSouth == 1 {
                    println!("You head South and see and empty beach. There's nothing here for you. You head back to where you came from.");
                }

                else {
                    println!("You treck South where you discover a human skeleton clutching a note in one hand, and a health potion in the other. What would you like to do?\n\n(Options: 'read_note', 'take_potion', 'return')");
                    location = 2;

                    while location == 2 {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        let input = input.trim();

                        match input {
                            "read_note" => {
                                println!("The note reads 'By the Gods... if my Giddy Aunt asks me to partake in the Heart of Iron once more, I'm going to get drunk on Health Potions and toss myself off Harks-Port into the sea.'")
                            }
                            "take_potion" => {
                                if hasPotion == 0 {
                                    println!("You take the potion.");
                                    hasPotion = 1;
                                }
                                else {
                                    println!("You already took the potion.");
                                }
                            }
                            "return" => {
                                println!("As you leave the South, a wave comes in and washes away the corpse.\n");
                                location = 0;
                                leftSouth = 1;

                            }
                            _ => {
                                println!("Invalid Input.");
                            }
                        }
                        
                    }
                }
            }

            "East" => {

                if leftEast == 1 {
                    println!("You've done all that's to be done in that area.");
                }

                else {
                    println!("You venture Eastwards back t'wards your crashed ship. You are confronted with the corpses of your ship-mates. Would you like to search for supplies?: y/n");
                    location = 3;

                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim();

                    if input == "n" {
                        println!("You go back to where you came from.\n");
                        location = 0;
                    }

                    else {
                        println!("You enter what's left of the hull. The winds have blown sand into the now exposed inner-body of the ship. ");
                        if goblinSword != 0 {
                            print!("However, there is nothing here for you but lost dreams and a rusty sword. You take the sword. You return to where you came from.\n");
                            leftEast = 1;
                            location = 0;
                            rustySword = 1;
                        }
                        else {
                            print!("You find your Western shipmate's old rusty cutlass. It's not much, but you need something to defend yourself with. You return back where you came from with your newfound weapon.\n");
                            rustySword = 3;
                            leftEast = 1;
                            location = 0;
                        }
                    }
                    
                }
            }

            "West" => {
                println!("You creep to the West and see what appears to be an Orcish Pirate Captain warming himself over a campfire. Will you approach? y/n");
                location = 4;

                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                if input == "n" {
                    println!("You head back the way you came.");
                    location = 0;
                }

                else {
                    println!("The Pirate sees you approach and points his cutlas in your general direction.\nOrc Pirate: 'What be thy business approaching mine own self ya welp? I had thought all th'merchantfolk had died on impact last night. I telleth thee this, scoundrel, thou shan't be taking my ship off't shore!'\n How will you respond?");
                    println!("1. Noble fellowman, I do humbly request to be taken aloft, off this Island. \n2. I tell thee this, giveth me thy ship or thou'll be seagull feed along with my fellowmen. \n3. Thy ship is mine! To battle! [+1 attack bonus on first hit]");

                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim();

                    match input {
                        "1" => {
                            println!("I shall tell thee this, scoundrel. Give me a health potion, the Sword of a Goblin, and a Western Cutlass, and I shalt give thee voyage to th'nearest port. Else, it's to battle!");
                            if ((hasPotion != 0) && (rustySword != 0) && (goblinSword != 0)) {
                                println!("You hand the items to the pirate. \nOrc Pirate: 'Arrr alright then. Get thy boots aboard.'");
                                println!("The pirate takes you to the pirates cove of Keelhaul. Perhaps you will begin your life anew as a privateer.\n\nYou win! You got the 'Swashbuckler' ending!");
                            }

                            else {
                                println!("\nAye... I thought as much... HYAAAAAA!");
                                println!("[The Pirate gets the jump on you, he gets a +1 attack bonus on his first hit]");
                                hp = hp - 1;

                                println!("Type 'Hit' to attack, and if you have a health potion, type 'Heal' to regain all your health.");

                                inBattle = 1;

                                while inBattle == 1 {
                                    let mut input = String::new();
                                    std::io::stdin().read_line(&mut input).unwrap();
                                    let input = input.trim();

                                    match input {
                                        "Hit" => {
                                            pirateHp = pirateHp - (attack + rustySword + goblinSword + goblinCompanion);
                                            println!("You hit the Pirate! He has {} HP remaining!", pirateHp);
                                            hp = hp - 3;
                                            println!("You were hit back! You have {} HP remaining!", hp);

                                            if hp < 1 {
                                                println!("You died. Game over.");
                                                location = 999;
                                                inBattle = 0;
                                            }

                                            if (pirateHp < 1 && hp > 0) {
                                                println!("Pirate: 'Arrrrghh! I hath been slain! Curses be unto thy house!'\n");

                                                inBattle = 0;
                                                println!("\nYou slay the Orcish Pirate and comendeer his ship. You sail westward until you eventually port in the western kingdom of Dras'bon.\n\nYou win! You got the 'Take that!' ending.");
                                                location = 999;
                                                std::process::exit;
                                            }
                                        }
                                        "Heal" => {
                                            if hasPotion == 1 {
                                                hp = 10;
                                                println!("Your health has been restored!");
                                            }
                                            else {
                                                println!("You dont have a health potion.");
                                            }
                                        }

                                        _ => {
                                            println!("Invalid Input.");
                                        }
                                    }
                                }

                            }

                        }

                        "2" => {
                            if ((rustySword != 0) && (goblinCompanion != 0)) {
                                println!("Goblin: 'Yes! You give boat or we cut your head off!'");
                                println!("Orc Pirate: 'I may be a swash-buckling privateer... but I know when I'm beat. Take the ship. It's so over.'\n");
                                println!("You and your Goblin friend travel the harsh seas of Phanglosien for three days. Finally, after the third night, you arrive on the port of Lanpart. You will live to see another day.\n\nYou win! You got the 'Strength in Numbers' ending!");
                                location = 999;
                                
                            }
                        }

                        "3" => {
                            println!("Pirate: 'Hwaa?!'");

                            pirateHp -= 1;

                            println!("Type 'Hit' to attack, and if you have a health potion, type 'Heal' to regain all your health.");

                            inBattle = 1;

                            while inBattle == 1 {
                                let mut input = String::new();
                                std::io::stdin().read_line(&mut input).unwrap();
                                let input = input.trim();

                                match input {
                                    "Hit" => {
                                        pirateHp = pirateHp - (attack + rustySword + goblinSword + goblinCompanion);
                                        println!("You hit the Pirate! He has {} HP remaining!", pirateHp);
                                        hp = hp - 3;
                                        println!("You were hit back! You have {} HP remaining!", hp);

                                        if hp < 1 {
                                            println!("You died. Game over.");
                                            location = 999;
                                            inBattle = 0;
                                        }

                                        if (pirateHp < 1 && hp > 0) {
                                            println!("Pirate: 'Arrrrghh! I hath been slain! Curses be unto thy house!'\n");

                                            println!("\nYou slay the Orcish Pirate and comendeer his ship. You sail westward until you eventually port in the western kingdom of Dras'bon.\n\nYou win! You got the 'Take that!' ending.");
                                            location = 999;
                                            inBattle = 0;
                                        }
                                    }
                                    "Heal" => {
                                        if hasPotion == 1 {
                                            hp = 10;
                                            println!("Your health has been restored!");
                                        }
                                        else {
                                            println!("You dont have a health potion.");
                                        }
                                    }

                                    _ => {
                                        println!("Invalid Input.");
                                    }
                                }
                            }

                        }

                        _ => {
                            println!("Invalid command.");
                        }

                    } //232
    
                } //224

            } //211

            _ => {
                println!("Invalid Input.");
            }

        } //24

    } // 18

}
