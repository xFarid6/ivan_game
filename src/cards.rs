use std::{collections::HashMap, fs};

use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng, Rng};

#[macro_export]
macro_rules! enum_str {
    // Only matches enums which variants have no values
    (enum $name:ident {
        $($variant:ident),*,
    }) => {
        enum $name {
            $($variant),*
        }

        impl $name {
            fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };

    // Also matches enums which variants have a value of any expression
    (enum $name:ident {
        $($variant:ident = $val:expr),*,
    }) => {
        enum $name {
            $($variant = $val),*
        }

        impl $name {
            fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}


pub const CARD_NAMES_ARRAY: [&str; 56] = ["10_of_clubs",
                                "10_of_diamonds",
                                "10_of_hearts",
                                "10_of_spades",
                                "2_of_clubs",
                                "2_of_diamonds",
                                "2_of_hearts",
                                "2_of_spades",
                                "3_of_clubs",
                                "3_of_diamonds",
                                "3_of_hearts",
                                "3_of_spades",
                                "4_of_clubs",
                                "4_of_diamonds",
                                "4_of_hearts",
                                "4_of_spades",
                                "5_of_clubs",
                                "5_of_diamonds",
                                "5_of_hearts",
                                "5_of_spades",
                                "6_of_clubs",
                                "6_of_diamonds",
                                "6_of_hearts",
                                "6_of_spades",
                                "7_of_clubs",
                                "7_of_diamonds",
                                "7_of_hearts",
                                "7_of_spades",
                                "8_of_clubs",
                                "8_of_diamonds",
                                "8_of_hearts",
                                "8_of_spades",
                                "9_of_clubs",
                                "9_of_diamonds",
                                "9_of_hearts",
                                "9_of_spades",
                                "ace_of_clubs",
                                "ace_of_diamonds",
                                "ace_of_hearts",
                                "ace_of_spades",
                                "back",
                                "back@2x",
                                "black_joker",
                                "jack_of_clubs",
                                "jack_of_diamonds",
                                "jack_of_hearts",
                                "jack_of_spades",
                                "king_of_clubs",
                                "king_of_diamonds",
                                "king_of_hearts",
                                "king_of_spades",
                                "queen_of_clubs",
                                "queen_of_diamonds",
                                "queen_of_hearts",
                                "queen_of_spades",
                                "red_joker"
];

// ====== STRUCTS ======

#[derive(Resource)]
pub struct CardHandles {
    pub cards_map: HashMap<String, Handle<Image>>,
}

#[derive(Debug, Component)]
enum Suit {
    Clubs = 0,
    Hearts = 1,
    Spades = 2, 
    Diamonds = 3
}

impl Suit {
    fn into_string(&self) -> String {
        let ans = match self {
            Suit::Clubs => "Clubs",
            Suit::Hearts => "Hearts",
            Suit::Spades => "Spades",
            Suit::Diamonds => "Diamonds",
        };
        ans.to_string()
    }

    fn from_string(raw_string: &str) -> Self {
        // let raw_string = raw_string.to_lowercase();
        match raw_string {
            "clubs" => Suit::Clubs,
            "hearts" => Suit::Hearts,
            "spades" => Self::Spades,
            "diamonds" => Self::Diamonds,
            _ => panic!("String could not be converted into any Suit variant")
        }
    }
}

#[derive(Debug, Bundle)]
struct CardBundle {
    suit: Suit,
    front: SpriteBundle,
}


// ====== METHODS ======


pub fn load_cards_pngs(
    asset_server: Res<AssetServer>,
    mut handles_map: ResMut<CardHandles>    
) {
    let cards_path = fs::read_dir("assets/playing-cards-assets-master/png").unwrap();

    for path in cards_path {
        // println!("Name: {}", path.unwrap().path().display());
        let path = path.unwrap().path().into_os_string().into_string().expect("Failed to convert PathBuf to String");
        let path = path.replace("assets/", "");
        // println!("{:?}", path);
        // example: "playing-cards-assets-master/png\\queen_of_hearts.png"

        // Save the handle to the image
        let handle: Handle<Image> = asset_server.load(path.clone());

        // Construct the key for the hashmap
        let map_key = path.replace(".png", "");
        let mut map_key: Vec<_> = map_key.split("\\").collect();
        let map_key = map_key[1];
        // dbg!(map_key);

        handles_map.cards_map.insert(map_key.to_owned(), handle);

        /* This appears to be not needed
        // now we have, for example: "7_of_clubs"
        // and we want: "seven_of_clubs"
        let number = map_key.split("_").into_iter().next().unwrap();
        match number.parse::<u8>() {
            Ok(n) => {
                // Remove the number in front
                let map_key = map_key.replace(number, "");
                
                // convert n to a string
                let string_n = int_to_string(n);
                let map_key = string_n + &map_key.to_string();
                // println!("{:?}", map_key);

                // Finally add it to the map
                handles_map.cards_map.insert(map_key.to_string(), handle);
            },
            Err(_) => { handles_map.cards_map.insert(map_key.to_owned(), handle); },
        }
        */

    }
    // dbg!(&handles_map.cards_map);
}

/// Converts a single digit to it's literal name
pub fn int_to_string(n: u8) -> String {
    let ans = match n {
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        _ => panic!("Only convert single digits or 10!")
    };

    ans.to_string()
}

pub fn spawn_card(
    card_name: String, 
    mut commands: Commands, 
    card_handles: Res<CardHandles>
) {
    // Card name example: 7_of_clubs
    let suit_part: Vec<_> = card_name.split("_").collect();
    let card_suit = suit_part.last().expect("Maybe vec was empty?");
    dbg!(card_suit);
    dbg!(&card_name);

    commands.spawn(CardBundle {
        suit: Suit::from_string(card_suit),
        front: SpriteBundle {
            texture: card_handles.cards_map.get(&card_name).expect("No handles found for this key").clone(),
            transform: Transform {
                translation: Vec3 { x: 0., y: 400., z: 1. },
                ..Default::default()
            },
            ..default()
        },
    });

    commands.spawn(SpriteBundle {
        texture: card_handles.cards_map.get("back").expect("No handles found for this key").clone(),
        transform: Transform {
            translation: Vec3 { x: 0., y: 400., z: 0. },
            ..Default::default()
        },
        ..default()
    });
}

pub fn spawn_random_card(
    mut commands: Commands,
    card_handles: Res<CardHandles>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    if !keyboard_input.just_released(KeyCode::KeyC) { return; }

    let invalid_cards = ["back", "back@2x", "black_joker", "red_joker"];

    let mut rng = rand::thread_rng();

    while let Some(card_name) = CARD_NAMES_ARRAY.choose(&mut rng){
        if !invalid_cards.contains(card_name) {
            spawn_card(card_name.to_owned().to_owned(), commands, card_handles);
            return;
        }
    };    
}
