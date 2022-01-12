// https://www.codewars.com/kata/5a5db0f580eba84589000979/train/rust

#[cfg(test)]
mod example_tests {
    use super::*;

    #[test]
    fn cell_move_test() {
        use pnz::field::*;

        let mut cell = Cell(0, 0);
        let field = Field::new(&vec!["2  ", "  S "]);

        assert!(cell.can_move_by_direction(&field, &DIRECTION_RIGHT));
        assert_eq!(
            cell.can_move_by_direction(&field, &DIRECTION_UP_RIGHT),
            false
        );
        assert_eq!(
            cell.can_move_by_direction(&field, &DIRECTION_DOWN_RIGHT),
            true
        );

        cell.move_by_direction(&DIRECTION_RIGHT);

        assert_eq!(cell, Cell(0, 1));

        cell.move_by_direction(&DIRECTION_DOWN_RIGHT);

        assert_eq!(cell, Cell(1, 2));

        assert_eq!(cell.can_move_by_direction(&field, &DIRECTION_RIGHT), false);
        assert_eq!(
            cell.can_move_by_direction(&field, &DIRECTION_UP_RIGHT),
            false
        );
        assert_eq!(
            cell.can_move_by_direction(&field, &DIRECTION_DOWN_RIGHT),
            false
        );
    }

    #[test]
    fn example_tests() {
        let example_tests: Vec<(Vec<&str>, Vec<Vec<usize>>, usize)> = vec![
            (
                vec!["2       ", "  S     ", "21  S   ", "13      ", "2 3     "],
                vec![
                    vec![0, 4, 28],
                    vec![1, 1, 6],
                    vec![2, 0, 10],
                    vec![2, 4, 15],
                    vec![3, 2, 16],
                    vec![3, 3, 13],
                ],
                10,
            ),
            (
                vec!["11      ", " 2S     ", "11S     ", "3       ", "13      "],
                vec![
                    vec![0, 3, 16],
                    vec![2, 2, 15],
                    vec![2, 1, 16],
                    vec![4, 4, 30],
                    vec![4, 2, 12],
                    vec![5, 0, 14],
                    vec![7, 3, 16],
                    vec![7, 0, 13],
                ],
                12,
            ),
            (
                vec![
                    "12        ",
                    "3S        ",
                    "2S        ",
                    "1S        ",
                    "2         ",
                    "3         ",
                ],
                vec![
                    vec![0, 0, 18],
                    vec![2, 3, 12],
                    vec![2, 5, 25],
                    vec![4, 2, 21],
                    vec![6, 1, 35],
                    vec![6, 4, 9],
                    vec![8, 0, 22],
                    vec![8, 1, 8],
                    vec![8, 2, 17],
                    vec![10, 3, 18],
                    vec![11, 0, 15],
                    vec![12, 4, 21],
                ],
                20,
            ),
            (
                vec!["12      ", "2S      ", "1S      ", "2S      ", "3       "],
                vec![
                    vec![0, 0, 15],
                    vec![1, 1, 18],
                    vec![2, 2, 14],
                    vec![3, 3, 15],
                    vec![4, 4, 13],
                    vec![5, 0, 12],
                    vec![6, 1, 19],
                    vec![7, 2, 11],
                    vec![8, 3, 17],
                    vec![9, 4, 18],
                    vec![10, 0, 15],
                    vec![11, 4, 14],
                ],
                19,
            ),
            (
                vec![
                    "1         ",
                    "SS        ",
                    "SSS       ",
                    "SSS       ",
                    "SS        ",
                    "1         ",
                ],
                vec![
                    vec![0, 2, 16],
                    vec![1, 3, 19],
                    vec![2, 0, 18],
                    vec![4, 2, 21],
                    vec![6, 3, 20],
                    vec![7, 5, 17],
                    vec![8, 1, 21],
                    vec![8, 2, 11],
                    vec![9, 0, 10],
                    vec![11, 4, 23],
                    vec![12, 1, 15],
                    vec![13, 3, 22],
                ],
                0,
            ),
        ];

        example_tests.into_iter().for_each(|(grid, zqueue, sol)| {
            assert_eq!(pnz::plants_and_zombies(&grid, &zqueue), sol)
        });
    }
}

mod pnz {
    pub mod field {
        #[derive(PartialEq, Eq, Hash, Clone, Debug)]
        pub struct Cell(pub usize, pub usize);

        impl Cell {
            pub fn can_move_by_direction(
                &self,
                field: &Field,
                &Direction(x, y): &Direction,
            ) -> bool {
                if x < 0 && self.0 == 0 {
                    return false;
                }

                if y < 0 && self.1 == 0 {
                    return false;
                }

                if x > 0 && self.0 == field.height() - 1 {
                    return false;
                }

                if y > 0 && self.1 == field.width() - 1 {
                    return false;
                }

                true
            }

            pub fn move_by_direction(&mut self, direction: &Direction) {
                self.0 = ((self.0 as isize) + direction.0) as usize;
                self.1 = ((self.1 as isize) + direction.1) as usize;
            }
        }

        pub struct Direction(isize, isize);

        #[derive(Debug)]
        pub struct Field {
            height: usize,
            width: usize,
        }

        impl Field {
            pub fn new(lawn: &Vec<&str>) -> Field {
                let height = lawn.len();

                let width = if height > 0 { lawn[0].len() } else { 0 };

                Field { height, width }
            }

            pub fn height(&self) -> usize {
                self.height
            }

            pub fn width(&self) -> usize {
                self.width
            }
        }

        pub const DIRECTION_RIGHT: Direction = Direction(0, 1);
        pub const DIRECTION_UP_RIGHT: Direction = Direction(-1, 1);
        pub const DIRECTION_DOWN_RIGHT: Direction = Direction(1, 1);
    }

    mod plants {
        use super::field::*;
        use super::game::GameState;
        use super::zombies::Zombies;
        use std::cmp::Ordering;
        use std::collections::BTreeMap;
        use std::collections::HashMap;

        impl Ord for Cell {
            fn cmp(&self, other: &Self) -> Ordering {
                let cmp = self.1.cmp(&other.1);

                if let Ordering::Equal = cmp {
                    return self.0.cmp(&other.0);
                }

                cmp.reverse()
            }
        }

        impl PartialOrd for Cell {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        #[derive(Debug)]
        struct NPlant {
            power: u8,
        }

        #[derive(Debug)]
        struct SPlant;

        #[derive(Debug)]
        pub struct Plants {
            n_plants: HashMap<Cell, NPlant>,
            n_plants_power_by_row: Vec<u32>,
            s_plants: BTreeMap<Cell, SPlant>,
        }

        impl Plants {
            pub fn new(lawn: &Vec<&str>) -> Self {
                let mut n_plants = HashMap::new();
                let mut n_plants_power_by_row = vec![0; lawn.len()];
                let mut s_plants: BTreeMap<Cell, SPlant> = BTreeMap::new();

                for (x, row) in lawn.iter().enumerate() {
                    for (y, c) in row.chars().enumerate() {
                        let cell = Cell(x, y);

                        match c {
                            ' ' => continue,
                            'S' => {
                                s_plants.insert(cell, SPlant);
                            }
                            _ => {
                                let power = c.to_digit(10).expect("value should be digit") as u8;

                                n_plants.insert(cell, NPlant { power });
                                n_plants_power_by_row[x] += power as u32;
                            }
                        }
                    }
                }

                Self {
                    n_plants,
                    n_plants_power_by_row,
                    s_plants,
                }
            }

            fn shoot_by_direction(
                game_state: &GameState,
                zombies: &mut Zombies,
                cell: Cell,
                power: u32,
                direction: Direction,
            ) {
                let mut power = power;
                let mut moving_cell = cell;
                let field = game_state.field();

                while power > 0 && moving_cell.can_move_by_direction(field, &direction) {
                    moving_cell.move_by_direction(&direction);

                    power -= zombies.hit(game_state, &moving_cell, power);
                }
            }

            pub fn turn(&self, game_state: &GameState, zombies: &mut Zombies) {
                let turn = game_state.turn();
                let width = game_state.field().width() - 2;

                for (row, &power) in self.n_plants_power_by_row.iter().enumerate() {
                    if power == 0 {
                        continue;
                    }

                    let column = if width > turn as usize {
                        width - turn as usize
                    } else {
                        0
                    };

                    let cell = Cell(row, column);
                    Self::shoot_by_direction(game_state, zombies, cell, power, DIRECTION_RIGHT);
                }

                for (cell, _) in &self.s_plants {
                    Self::shoot_by_direction(
                        game_state,
                        zombies,
                        cell.clone(),
                        1,
                        DIRECTION_UP_RIGHT,
                    );
                    Self::shoot_by_direction(game_state, zombies, cell.clone(), 1, DIRECTION_RIGHT);
                    Self::shoot_by_direction(
                        game_state,
                        zombies,
                        cell.clone(),
                        1,
                        DIRECTION_DOWN_RIGHT,
                    );
                }
            }

            pub fn hit(&mut self, cell: &Cell) {
                if let Some(plant) = self.n_plants.remove(cell) {
                    self.n_plants_power_by_row[cell.0] -= plant.power as u32;
                    return;
                }

                self.s_plants.remove(cell);
            }
        }
    }

    mod zombies {
        use super::{field::*, game::GameState, plants::Plants};
        use std::collections::HashMap;

        #[derive(Debug)]
        struct Zombie {
            health: u32,
        }

        #[derive(PartialEq, Eq, Hash, Debug)]
        struct ZombieAppearance(pub u32, pub usize);

        #[derive(Debug)]
        pub struct Zombies {
            zombies: HashMap<ZombieAppearance, Zombie>,
        }

        fn get_zombie_position(game_state: &GameState, zombie: &ZombieAppearance) -> Option<Cell> {
            let current_turn = game_state.turn();
            let width = game_state.field().width() - 1;

            if current_turn < zombie.0 {
                return None;
            }

            Some(Cell(zombie.1, width - (current_turn - zombie.0) as usize))
        }

        impl Zombies {
            pub fn new(list: &Vec<Vec<usize>>) -> Self {
                let mut zombies: HashMap<ZombieAppearance, Zombie> = HashMap::new();

                for zombie_desc in list {
                    let appear_turn = zombie_desc[0] as u32;
                    let row = zombie_desc[1];
                    let health = zombie_desc[2] as u32;

                    let appearance = ZombieAppearance(appear_turn, row);

                    zombies.insert(appearance, Zombie { health });
                }

                Self { zombies }
            }

            pub fn turn(&self, game_state: &GameState, plants: &mut Plants) {
                for (zombie, _) in &self.zombies {
                    let maybe_cell = get_zombie_position(game_state, zombie);

                    if let Some(cell) = maybe_cell {
                        plants.hit(&cell);
                    }
                }
            }

            pub fn hit(&mut self, game_state: &GameState, cell: &Cell, power: u32) -> u32 {
                let current_turn = game_state.turn();
                let width = game_state.field().width() - 1;

                if (current_turn as usize) < width - cell.1 {
                    return 0;
                }

                let turn_to_appear = current_turn - (width - cell.1) as u32;
                let appearance = ZombieAppearance(turn_to_appear, cell.0);
                let maybe_zombie = self.zombies.get_mut(&appearance);

                if let Some(zombie) = maybe_zombie {
                    if zombie.health > power as u32 {
                        zombie.health -= power as u32;

                        return power;
                    } else {
                        let health = zombie.health;
                        self.zombies.remove(&appearance);

                        return health;
                    }
                };
                0
            }

            pub fn is_any_zombie_left(&self) -> bool {
                self.zombies.len() > 0
            }

            pub fn is_any_zombie_at_column(&self, game_state: &GameState, column: usize) -> bool {
                for (zombie, _) in &self.zombies {
                    let maybe_cell = get_zombie_position(game_state, zombie);

                    if let Some(cell) = maybe_cell {
                        if cell.1 == column {
                            return true;
                        }
                    }
                }

                false
            }
        }
    }

    mod game {
        use super::{field::Field, plants::Plants, zombies::Zombies};

        #[derive(Debug)]
        #[allow(unused_lifetimes)]
        pub struct GameState {
            turn: u32,
            field: Field,
        }

        impl GameState {
            pub fn new(field: Field) -> Self {
                GameState { turn: 0, field }
            }

            pub fn turn(&self) -> u32 {
                self.turn
            }

            pub fn field(&self) -> &Field {
                &self.field
            }

            pub fn next_turn(&mut self) {
                self.turn += 1;
            }
        }

        #[derive(Debug)]
        pub struct Game {
            game_state: GameState,
            plants: Plants,
            zombies: Zombies,
        }

        impl Game {
            pub fn new(lawn: &Vec<&str>, zombies_list: &Vec<Vec<usize>>) -> Self {
                let field = Field::new(lawn);
                let game_state = GameState::new(field);

                let plants = Plants::new(lawn);
                let zombies = Zombies::new(zombies_list);

                Self {
                    game_state,
                    plants,
                    zombies,
                }
            }

            pub fn play(&mut self) -> u32 {
                loop {
                    // println!("{:#?}", self);
                    // println!("__________");
                    // println!("");

                    self.zombies.turn(&self.game_state, &mut self.plants);

                    if self.zombies.is_any_zombie_at_column(&self.game_state, 0) {
                        return self.game_state.turn() + 1;
                    }

                    self.plants.turn(&self.game_state, &mut self.zombies);

                    if !self.zombies.is_any_zombie_left() {
                        return 0;
                    }

                    self.game_state.next_turn();
                }
            }
        }
    }

    use self::game::Game;

    pub fn plants_and_zombies(lawn: &Vec<&str>, zombies: &Vec<Vec<usize>>) -> usize {
        let mut game = Game::new(lawn, zombies);

        game.play() as usize
    }
}

fn main() {
    assert_eq!(
        pnz::plants_and_zombies(
            &vec!["2       ", "  S     ", "21  S   ", "13      ", "2 3     "],
            &vec![
                vec![0, 4, 28],
                vec![1, 1, 6],
                vec![2, 0, 10],
                vec![2, 4, 15],
                vec![3, 2, 16],
                vec![3, 3, 13],
            ],
        ),
        10
    );
}
