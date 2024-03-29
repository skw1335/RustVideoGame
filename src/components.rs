use specs::prelude::*;
use specs_derive::*;
use rltk::{RGB};
use serde::{Serialize, Deserialize};
use specs::saveload::{Marker, ConvertSaveload};
use specs::error::NoError;

#[derive(Component, ConvertSaveload, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order : i32
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Player {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Consumable {} 

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Item{} 

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct ProvidesHealing {
    pub heal_amount : i32
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Name {
    pub name : String
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct CombatStats {
    pub max_hp : i32,
    pub hp : i32,
    pub defense : i32,
    pub attack_power: i32
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToMelee {
    pub target : Entity
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct BlocksTile {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Monster {}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Viewshed {
    pub visible_tiles : Vec<rltk::Point>, 
    pub range : i32,
    pub dirty : bool
}
#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Ranged {
    pub range : i32
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct InflictsDamage {
    pub damage : i32
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct SufferDamage {
    pub amount : Vec<i32>
}
impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount : i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let dmg = SufferDamage { amount : vec![amount] };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct AreaOfEffect {
    pub radius : i32
}
#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct InBackpack {
   pub owner : Entity
}
#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToPickupItem {
    pub collected_by : Entity,
    pub item : Entity
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToDropItem {
    pub item : Entity
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToUseItem {
    pub item : Entity,
    pub target : Option<rltk::Point>
}


//Status Effects


#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Confusion {
    pub turns : i32
}

//Serialize/Deserialize
pub struct SerializeMe;


#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SerializationHelper {
    pub map : super::map::Map
}
