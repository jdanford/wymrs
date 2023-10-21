use std::collections::HashMap;

use num::clamp;
use rand::{rngs::StdRng, Rng, SeedableRng};
use rand_distr::Normal;

use crate::{
    color,
    config::{PIXEL_FORMAT, SPAWN_INTERVAL},
    random_wyrm_color, tile, Color, Direction, NewWyrmParams, Point, RelativeDirection, Result,
    Wyrm,
};

pub struct World {
    pub width: u16,
    pub height: u16,
    pub wyrms: HashMap<u16, Wyrm>,
    next_wyrm_id: u16,
    tiles: Vec<u16>,
    current_step: usize,
    rng: StdRng,
}

pub struct NewWorldParams {
    pub width: u16,
    pub height: u16,
}

pub type Neighbors = Vec<(RelativeDirection, i8)>;

impl World {
    #[must_use]
    pub fn new(params: &NewWorldParams) -> Self {
        let tile_count = usize::from(params.width) * usize::from(params.height);
        let mut world = World {
            width: params.width,
            height: params.height,
            wyrms: HashMap::new(),
            next_wyrm_id: tile::WYRM,
            tiles: vec![tile::EMPTY; tile_count],
            current_step: 0,
            rng: StdRng::from_entropy(),
        };

        world.fill();
        world
    }

    fn fill(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let position = Point::new(x, y);
                let tile = if self.at_edge(position) {
                    tile::WALL
                } else if self.rng.gen_bool(1.0 / 16.0) {
                    tile::FOOD
                } else {
                    tile::EMPTY
                };

                self.set_tile(position, tile);
            }
        }
    }

    fn get_tile(&self, position: Point<u16>) -> Result<u16> {
        let index = self.index(position);
        self.tiles
            .get(index)
            .copied()
            .ok_or(format!("invalid position: {position:?}"))
    }

    pub fn set_tile(&mut self, position: Point<u16>, tile: u16) {
        let index = self.index(position);
        self.tiles[index] = tile;
    }

    fn index(&self, position: Point<u16>) -> usize {
        usize::from(position.y) * usize::from(self.width) + usize::from(position.x)
    }

    fn at_edge(&self, point: Point<u16>) -> bool {
        point.x == 0 || point.x == self.width - 1 || point.y == 0 || point.y == self.height - 1
    }

    fn get_next_wyrm_id(&mut self) -> u16 {
        let next_id = self.next_wyrm_id;
        if self.next_wyrm_id == u16::MAX {
            self.next_wyrm_id = tile::WYRM;
            while self.wyrms.contains_key(&self.next_wyrm_id) {
                self.next_wyrm_id += 1;
            }
        } else {
            self.next_wyrm_id += 1;
        }

        next_id
    }

    pub fn create_wyrm(&mut self, position: Point<u16>) -> Result<()> {
        let current_tile = self.get_tile(position)?;
        if current_tile == tile::WALL || current_tile >= tile::WYRM {
            return Ok(());
        }

        let id = self.get_next_wyrm_id();
        let color = random_wyrm_color(&mut self.rng, id)?;
        let direction_index = self.rng.gen_range(0..=3);
        let direction = Direction::try_from(direction_index)
            .map_err(|_| format!("can't cast {direction_index} to direction"))?;
        let wyrm = Wyrm::new(&NewWyrmParams {
            id,
            color,
            direction,
            position,
        });

        self.wyrms.insert(id, wyrm);
        self.set_tile(position, id);
        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn create_random_wyrm(&mut self) -> Result<()> {
        let distribution = Normal::new(0.5, 0.1).map_err(|e| e.to_string())?;
        let rx = (self.rng.sample(distribution) * f32::from(self.width)) as u16;
        let ry = (self.rng.sample(distribution) * f32::from(self.height)) as u16;
        let x = clamp(rx, 1, self.width - 2);
        let y = clamp(ry, 1, self.height - 2);
        let position = Point::new(x, y);
        self.create_wyrm(position)
    }

    pub fn step(&mut self) -> Result<()> {
        if self.current_step > SPAWN_INTERVAL / 2 && self.current_step % SPAWN_INTERVAL == 0 {
            self.create_random_wyrm()?;
        }

        let sorted_wyrm_ids: Vec<u16> = self.wyrms.keys().copied().collect();
        for wyrm_id in sorted_wyrm_ids {
            if self.wyrms.contains_key(&wyrm_id) {
                self.update_wyrm(wyrm_id)?;
            }
        }

        self.current_step += 1;
        Ok(())
    }

    fn get_wyrm(&self, wyrm_id: u16) -> Result<&Wyrm> {
        self.wyrms
            .get(&wyrm_id)
            .ok_or(format!("(get_wyrm) invalid wyrm ID: {wyrm_id}"))
    }

    fn get_wyrm_mut(&mut self, wyrm_id: u16) -> Result<&mut Wyrm> {
        self.wyrms
            .get_mut(&wyrm_id)
            .ok_or(format!("(get_wyrm_mut) invalid wyrm ID: {wyrm_id}"))
    }

    fn update_wyrm(&mut self, wyrm_id: u16) -> Result<()> {
        let wyrm = self.get_wyrm(wyrm_id)?;
        let neighbors = self.get_neighbors(wyrm.head(), wyrm.direction)?;
        let (relative_direction, _) = neighbors[0];
        self.do_wyrm_action(wyrm_id, relative_direction)
    }

    fn do_wyrm_action(
        &mut self,
        wyrm_id: u16,
        relative_direction: RelativeDirection,
    ) -> Result<()> {
        let wyrm = self.get_wyrm(wyrm_id)?;
        let direction = wyrm.direction.rotate(relative_direction);
        let destination = wyrm.head().move_in_direction(direction)?;

        let tile_id = self.get_tile(destination)?;
        match tile_id {
            tile::WALL => self.destroy_wyrm(wyrm_id),
            tile::EMPTY => {
                let poop = self.rng.gen_bool(1.0 / 32.0);
                self.move_wyrm(wyrm_id, direction, false, poop)
            }
            tile::FOOD => self.move_wyrm(wyrm_id, direction, true, false),
            _ if tile_id == wyrm_id => self.destroy_wyrm(wyrm_id),
            enemy_wyrm_id => self.fight_wyrms(wyrm_id, enemy_wyrm_id),
        }?;

        Ok(())
    }

    fn move_wyrm(
        &mut self,
        wyrm_id: u16,
        direction: Direction,
        grow: bool,
        poop: bool,
    ) -> Result<()> {
        let wyrm = self.get_wyrm_mut(wyrm_id)?;
        let destination = wyrm.head().move_in_direction(direction)?;
        wyrm.segments.push_front(destination);
        self.set_tile(destination, wyrm_id);

        if !grow {
            let wyrm = self.get_wyrm_mut(wyrm_id)?;
            let end = wyrm.segments.pop_back().expect("wyrm is empty");
            let tile = if poop { tile::FOOD } else { tile::EMPTY };
            self.set_tile(end, tile);
        }

        let wyrm = self.get_wyrm_mut(wyrm_id)?;
        wyrm.direction = direction;
        Ok(())
    }

    #[allow(clippy::cast_precision_loss)]
    fn destroy_wyrm(&mut self, wyrm_id: u16) -> Result<()> {
        let wyrm = self
            .wyrms
            .remove(&wyrm_id)
            .ok_or(format!("(destroy_wyrm) invalid wyrm ID: {wyrm_id}"))?;
        for (i, position) in wyrm.segments.iter().copied().enumerate() {
            let food_chance = clamp(1.0 / (i as f64 + 1.0) + 0.5, 0.0, 1.0);
            let tile = if self.rng.gen_bool(food_chance) {
                tile::FOOD
            } else {
                tile::EMPTY
            };
            self.set_tile(position, tile);
        }

        Ok(())
    }

    #[allow(clippy::similar_names)]
    #[allow(clippy::cast_precision_loss)]
    fn fight_wyrms(&mut self, wyrm_a_id: u16, wyrm_b_id: u16) -> Result<()> {
        let wyrm_a = self.get_wyrm(wyrm_a_id)?;
        let wyrm_b = self.get_wyrm(wyrm_b_id)?;
        let ratio = wyrm_a.size() as f32 / wyrm_b.size() as f32;
        let advantage = self.rng.gen_range(8.0..=12.0) / 10.0;
        let final_ratio = ratio * advantage;
        let (winner_id, loser_id) = if final_ratio >= 0.5 {
            (wyrm_a_id, wyrm_b_id)
        } else {
            (wyrm_b_id, wyrm_a_id)
        };

        self.destroy_wyrm(loser_id)?;
        self.do_wyrm_action(winner_id, RelativeDirection::Forward)
    }

    fn get_tile_color(&self, tile: u16) -> Color {
        match tile {
            tile::EMPTY => *color::EMPTY,
            tile::WALL => *color::WALL,
            tile::FOOD => *color::FOOD,
            i => self.wyrms.get(&i).map_or(*color::MISSING, |w| w.color),
        }
    }

    fn get_neighbors(&self, position: Point<u16>, forward: Direction) -> Result<Neighbors> {
        let left = forward.rotate(RelativeDirection::Left);
        let right = forward.rotate(RelativeDirection::Right);

        let forward_position = position.move_in_direction(forward)?;
        let left_position = position.move_in_direction(left)?;
        let right_position = position.move_in_direction(right)?;

        let forward_tile = self.get_tile(forward_position)?;
        let left_tile = self.get_tile(left_position)?;
        let right_tile = self.get_tile(right_position)?;

        let mut neighbors = vec![
            (RelativeDirection::Forward, tile::score(forward_tile)),
            (RelativeDirection::Left, tile::score(left_tile)),
            (RelativeDirection::Right, tile::score(right_tile)),
        ];
        neighbors.sort_by(|(_, score_a), (_, score_b)| score_b.cmp(score_a));
        Ok(neighbors)
    }

    pub fn render(&self, pixel_data: &mut [u8]) {
        for y in 0..self.height {
            for x in 0..self.width {
                let position = Point::new(x, y);
                let tile_index = self.index(position);
                let tile = self.tiles[tile_index];
                let tile_color = self.get_tile_color(tile);
                let (r, g, b) = tile_color.rgb();

                let i = tile_index * PIXEL_FORMAT.byte_size_per_pixel();
                pixel_data[i] = r;
                pixel_data[i + 1] = g;
                pixel_data[i + 2] = b;
            }
        }
    }
}
