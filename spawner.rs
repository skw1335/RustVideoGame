use rltk::{RGB, RandomNumberGenerator,  }; 
use specs::prelude::*;
use super::{CombatStats, Player, Renderable, Name, Position, Rect, Viewshed, Monster, BlocksTile, map::MAPWIDTH, Item, Consumable, ProvidesHealing, Ranged, InflictsDamage, AreaOfEffect, Confusion, SerializeMe};
use specs::saveload::{MarkedBuilder, SimpleMarker};


pub fn player(ecs : &mut World, player_x : i32, player_y : i32) -> Entity {
    ecs
        .create_entity()
        .with(Position { x: player_x, y : player_y } )
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            render_order: 0
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles : Vec::new(), range: 8, dirty: true })
        .with(Name{name: "Player".to_string() })
        .with(CombatStats{ max_hp: 30, hp: 30, defense: 2, attack_power: 5 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

fn room_table() -> RandomTable {
    RandomTable::new()
        .add("Goblin", 10)
        .add("Orc", 1)
        .add("Health Potion", 7)
        .add("Fireball Scroll", 2)
        .add("Confusion Scroll", 2)
        .add("Magic Missile Scroll", 4)
}

const MAX_MONSTERS: i32 = 4;
const MAX_ITEMS: i32 = 20;

fn orc(ecs: &mut World, x: i32, y: i32) { monster(ecs, x, y, rltk::to_cp437('o'), "Orc"); }
fn goblin(ecs: &mut World, x: i32, y: i32) { monster(ecs, x, y, rltk::to_cp437('g'), "Goblin"); }

fn monster<S : ToString>(ecs: &mut World, x: i32, y: i32, glyph : rltk::FontCharType, name : S) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph, 
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            render_order: 1
        })
        .with(Viewshed { visible_tiles : Vec::new(), range: 8, dirty: true })
        .with(Monster{})
        .with(Name { name : name.to_string() })
        .with(BlocksTile{})
        .with(CombatStats { max_hp: 16, hp: 16, defense: 1, attack_power: 4 })
        .marked::<SimpleMarker<SerializeMe>>() 
        .build();
}
pub fn spawn_room(ecs: &mut World, room : &Rect) {
    let spawn_table : room_table();
    let mut spawn_points : HashMap<usize, String> = HashMap::new();

    //scope to keep borrow checker happy 
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_spawns = rng.roll_dice(1, MAX_MONSTERS + 3) - 3;
        
        for _i in 0 .. num_spawns {
            let mut added = false;
            let mut tries = 0
            while !added && tries < 20 {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !spawn_points.contains_key(&idx) {
                    spawn_points.insert(idx, spawn_table.roll(&mut rng));
                    added = true;
                } else {
                    tries += 1;
                }
            }
        }
    }
    
    //Actually spawn the monsters/potions
    for spawn in spawn_points.iter() {
        let x = (*spawn.0 % MAPWIDTH) as i32;
        let y = (*spawn.0 / MAPWIDTH) as i32;

        match spawn.1.as_ref() {
            "Goblin" => goblin(ecs, x, y),
            "Orc" => orc(ecs, x, y),
            "Health Potion" => health_potion(ecs, x, y),
            "Fireball Scroll" => fireball_scroll(ecs, x, y),
            "Confusion Scroll" => confusion_scroll(ecs, x, y),
            "Magic Missile Scroll" => magic_missile_scroll(ecs, x, y),
            _ => {}
        }
    }
}
fn health_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('¡'),
            fg: RGB::named(rltk::MAGENTA),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Health Potion".to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(ProvidesHealing { heal_amount: 8})
        .build();
}

fn magic_missle_scroll(ecs: &mut World, x: i32, y:i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::CYAN),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name { name: "Magic Missle Scroll".to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(Ranged { range: 6 })
        .with(InflictsDamage{ damage: 8 })
        .marked::<SimpleMarker<SerializeMe>>() 
        .build();
}

fn fireball_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
    .with(Name{ name: "Fireball Scroll".to_string() })
    .with(Item{})
    .with(Consumable{})
    .with(Ranged{ range: 6 })
    .with(InflictsDamage { damage: 20 })
    .with(AreaOfEffect{ radius: 3 })
    .marked::<SimpleMarker<SerializeMe>>() 
    .build();
}

fn confusion_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y} )
        .with(Renderable{
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::PINK),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name { name: "Confusion Scroll". to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(Confusion { turns: 4 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}