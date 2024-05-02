use super::{MapBuilder, Map, Rect, apply_room_to_map,
TileType, Position, spawner, SHOW_MAPGEN_VISUALIZER};
use rltk::RandomNumberGenerator;
use specs::prelude::*;
use std::collections::HashMap;
const MIN_ROOM_SIZE : i32 = 8;

pub struct CellularAutomataBuilder { 
    map : Map, 
    starting_position : Position,
    depth: i32, 
    history: Vec<Map>,
    noise_areas : HashMap<i32, Vec<usize>>
}

impl MapBuilder for CellularAutomataBuilder {
    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&self) -> Position {
        self.starting_position.clone()
    }

    fn get_snapshot_history(&self) -> Vec<Map> {
        self.history.clone()
    }

    fn build_map(&mut self)  {
       self.build();
    }

    fn spawn_entities(&mut self, ecs : &mut World) {
        for area in self.noise_areas.iter() {
            spawner::spawn_region(ecs, area.1, self.depth);
        }
    }
    

    fn take_snapshot(&mut self) {
        if SHOW_MAPGEN_VISUALIZER {
            let mut snapshot = self.map.clone();
            for v in snapshot.revealed_tiles.iter_mut() {
                *v = true;
            }
            self.history.push(snapshot);
        }
    }
}

impl CellularAutomataBuilder {
    pub fn new(new_depth : i32) -> CellularAutomataBuilder {
        CellularAutomataBuilder{
            map : Map::new(new_depth),
            starting_position : Position{ x: 0, y : 0 },
            depth : new_depth,
            history: Vec::new(),
            noise_areas : HashMap::new()
        }
    }
    fn build(&mut self) {
        let mut rng = RandomNumberGenerator::new();
        
        // First we completely randomize the map, setting 55% of it to be floor.
        for y in 1..self.map.height-1 {
            for x in 1..self.map.width-1 {
                let roll = rng.roll_dice(1, 100);
                let idx = self.map.xy_idx(x, y);
                if roll > 55 { self.map.tiles[idx] = TileType::Floor }
                else { self.map.tiles[idx] = TileType::Wall } 
            }
        }
        self.take_snapshot();
    
        // Now we iteratively apply cellular automata rules
        for _i in 0..15 {
            let mut newtiles = self.map.tiles.clone();
            
            for y in 1..self.map.height-1 {
                for x  in 1..self.map.width-1 {
                    let idx = self.map.xy_idx(x, y);
                    let mut neighbors = 0;
                    if self.map.tiles[idx - 1] == TileType::Wall { neighbors += 1; }
                    if self.map.tiles[idx + 1] == TileType::Wall { neighbors += 1; }
                    if self.map.tiles[idx - self.map.width as usize] == TileType::Wall { neighbors += 1; }
                    if self.map.tiles[idx + self.map.width as usize] == TileType::Wall { neighbors += 1; }
                    if self.map.tiles[idx - (self.map.width as usize - 1)] == TileType::Wall { neighbors += 1; }
                    if self.map.tiles[idx - (self.map.width as usize + 1)] == TileType::Wall { neighbors += 1; }
                    if self.map.tiles[idx + (self.map.width as usize - 1)] == TileType::Wall { neighbors += 1; }
                    if self.map.tiles[idx + (self.map.width as usize + 1)] == TileType::Wall { neighbors += 1; }

                    if neighbors > 4 || neighbors == 0 {
                        newtiles[idx] = TileType::Wall;
                    }
                    else { 
                        newtiles[idx] = TileType::Floor;
                    }
                }
            }

            self.map.tiles = newtiles.clone();
            self.take_snapshot();
        }
     // Placing the Player
     self.starting_position = Position { x: self.map.width / 2, y: self.map.height / 2 };
     let mut start_idx = self.map.xy_idx(self.starting_position.x, self.starting_position.y);
     while self.map.tiles[start_idx] != TileType::Floor {
         self.starting_position.y += 1;
         start_idx = self.map.xy_idx(self.starting_position.x, self.starting_position.y);
     }
     //Placing the exit
     let mut rng = RandomNumberGenerator::new();
     let roll = rng.roll_dice(4, 4);
     match roll {
         // Left side of map
         1 => { 
             let edge = 0;
                for i in 0..self.map.height-1 {
                    let idx = self.map.xy_idx(edge, i);
                }
         }
         // Top side of map
         2 => {
             let edge = self.map.height-1;
                for i in 0..self.map.width-1 {
                    let idx = self.map.xy_idx(i, edge);
            }
         }
         // Right side of map
         3 => { 
             let edge = self.map.width-1; 
                for i in 0..self.map.height-1 {
                    let idx = self.map.xy_idx(edge, i);
                }
         }
         // Bottom side of map
    //     4 => { 
      //        let mut edge = 0;
        //      let mut i = 0;
          //      while i < self.map.width-1 {
            //        i += 1;
              //      let idx = self.map.xy_idx(i, edge);
                //    if self.map.tiles[idx] == TileType::Floor {
                  //      self.map.tiles[idx] = TileType::DownStairs;
                    //} else { edge += 1;
                    //         i = 0;
             //   }
      //   }
  //  }
         _ => {}
     }
     // todo
    
     // Build a noise map for spawning entities 
     let mut noise = rltk::FastNoise::seeded(rng.roll_dice(1, 65536) as u64);
     noise.set_noise_type(rltk::NoiseType::Cellular);
     noise.set_frequency(0.08);
     noise.set_cellular_distance_function(rltk::CellularDistanceFunction::Euclidean);

     for y in 1 .. self.map.height-1 {
         for x in 1 .. self.map.width-1 {
             let idx = self.map.xy_idx(x, y);
             if self.map.tiles[idx] == TileType::Floor {
                 let cell_value_f = noise.get_noise(x as f32, y as f32) * 10240.0;
                 let cell_value = cell_value_f as i32;

                 if self.noise_areas.contains_key(&cell_value) {
                     self.noise_areas.get_mut(&cell_value).unwrap().push(idx);
                 } else {
                     self.noise_areas.insert(cell_value, vec![idx]);
                    }
                }
            }
        }
    }
}

