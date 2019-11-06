// https://github.com/kmerfeld/cardgame/blob/master/board/src/lib.rs

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

pub struct Board {
    pub mission: Mission,
    pub tiles: Vec<Tile>,
    pub characters: Vec<Character>,

}

impl Default for Board {
    fn default() -> Board {
        return Board {
            mission: Mission::default(),
            tiles: Vec::new(),
            characters: Vec::new(),
        };
    }
}

pub enum Event {
    Cataclysm,
    RainOfFire,
    MilitaryIntervention,
}

pub struct Tile {
    name: String,
    description: String,
    cover: i32,
    exposure: i32,
    mayhem: i32,
}

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
            }
        }
    }
}


pub struct Objective {
    name: String,
    description: String,
    complete: bool,
}

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

/// Character represents an actor in Silencers
pub struct Character {
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

// Role separates the different kinds of Characters in Silencers
enum Role {
    Ascendant,
    Silencer,
    Reporter,
}

pub struct Power {
    name: String,
    description: String,
    effect: Effect,
}

// Effect is a game changing ability that breaks regular game rules
enum Effect {
    Attack,
    Defend,
    Manipulate,
    Control,
    Heal,
    Movement,
    AlterZone,
}