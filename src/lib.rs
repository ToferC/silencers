use std::fmt;

// https://github.com/kmerfeld/cardgame/blob/master/board/src/lib.rs

const HEIGHT: u32 = 5;
const WIDTH: u32 = 5;

pub struct Game {
    pub board: Board,
    pub silencer_player: Player,
    pub ascendant_player: Player,
    pub events: Vec<Event>,
}

pub struct Player {
    name: String,
    score: i32,
}

#[derive(Debug)]
pub struct Board {
    width: u32,
    height: u32,
    pub mission: Mission,
    pub tiles: Vec<Tile>,
    pub characters: Vec<Character>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mission_height = median(self.height);
        let mission_width = median(self.width);

        write!(f, "***Board***\n\n")?;

        for (j, line) in self.tiles.as_slice().chunks(self.width as usize).enumerate() {
            for (i, tile) in line.iter().enumerate() {
                if i == mission_height as usize && j == mission_width as usize {
                    write!(f, "{} ", self.mission.name)?;
                } else {
                // write Tile
                write!(f, "{} ", tile.name)?;
                }
            }
        write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Default for Board {
    fn default() -> Board {

        let mut tiles = Vec::new();

        for _ in 0..WIDTH {
            for _ in 0..HEIGHT {
                tiles.push(Tile::default())
            }
        }

        return Board {
            height: HEIGHT,
            width: WIDTH,
            mission: Mission::default(),
            tiles: tiles,
            characters: Vec::new(),
        };
    }
}

impl Board {
    // Add
}

pub enum Event {
    Cataclysm,
    RainOfFire,
    MilitaryIntervention,
}

#[derive(Debug)]
pub struct Tile {
    name: String,
    description: String,
    cover: i32,
    exposure: i32,
    mayhem: i32,
    threshold: i32,
    flipped: bool,
    flip_name: String,
    flip_description: String,
    flip_cover: i32,
    flip_exposure: i32,
    flip_mayhem: i32,
    flip_effect: Effect,
}

impl Default for Tile {
    fn default() -> Tile {
        return Tile {
            name: String::from("Barracks"),
            description: String::from("Dull, flat and imposing"),
            cover: 0,
            exposure: 0,
            mayhem: 0,
            threshold: 3,
            flipped: false,
            flip_name: String::from("Blazing inferno"),
            flip_description: String::from("Melting rock and intense heat"),
            flip_cover: 0,
            flip_exposure: 0,
            flip_mayhem: 3,
            flip_effect: Effect::AlterZone,
        }
    }
}

#[derive(Debug)]
pub struct Mission {
    name: String,
    description: String,
    objectives: Vec<Objective>,
    home_tile: Tile,
}

impl Default for Mission {
    fn default() -> Mission {
        return Mission {
            name: String::from("A Night at the Opera"),
            description: String::from("It was a dark and stormy night"),
            objectives: Vec::new(),
            home_tile: Tile {
                name: String::from("The Cathedral"),
                description: String::from("A tall, imposing gothic cathedral"),
                cover: 3,
                exposure: 2,
                mayhem: 0,
                threshold: 5,
                flipped: false,
                flip_name: String::from("The Broken Cathedral"),
                flip_description: String::from("A shattered hulk of a once imposing gothic cathedral"),
                flip_cover: 4,
                flip_exposure: 3,
                flip_mayhem: 5,
                flip_effect: Effect::AlterZone,
            }
        }
    }
}

#[derive(Debug)]
pub struct Objective {
    name: String,
    description: String,
    complete: bool,
}

#[derive(Debug)]
pub enum Actions {
    Unleash,
    Move,
    Craft,
    Interact,
    Obfuscate,
    Rest,
}

/// ActionResult is the result of the game's 2d6+stat roll
pub enum ActionResult {
    Failure,
    SuccessWithCost,
    CriticalSuccess,
    Overload,
}

#[derive(Debug)]
pub struct Character {
    // Character represents an actor in Silencers
    name: String,
    background: String,
    role: Role,
    health: i32,
    fame: i32,
    psyche: i32,
    strength: i32,
    agility: i32,
    skill: i32,
    presence: i32,
    ingenuity: i32,
    power: Power,
    location: Tile,
}

#[derive(Debug)]
enum Role {
    // Role separates the different kinds of Characters in Silencers
    Ascendant,
    Silencer,
    Reporter,
}

#[derive(Debug)]
pub struct Power {
    name: String,
    description: String,
    effect: Effect,
}

#[derive(Debug)]
enum Effect {
    // Effect is a game changing ability that breaks regular game rules
    Attack,
    Defend,
    Manipulate,
    Control,
    Heal,
    Movement,
    AlterZone,
}

fn mean(numbers: &Vec<u32>) -> f32 {
    let sum: u32 = numbers.iter().sum();

    sum as f32 / numbers.len() as f32
}

fn median(number: u32) -> u32 {
        let v: Vec<u32> = (0..number).collect();

        let mid: usize = v.len() / 2;

        v[mid]
    }