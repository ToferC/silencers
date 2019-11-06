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
    mission: Mission,
    tiles: Vec<Tile>,
}

impl Default for Board {
    fn default() -> Board {
        return Board {
            mission: Mission::default(),
            tiles: Vec::new(),
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
    pub characters: Vec<Character>,
}

pub struct Mission {
    name: String,
    description: String,
    objectives: Vec<Objective>,
    cover: i32,
    exposure: i32,
    mayhem: i32,
    pub characters: Vec<Character>,
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