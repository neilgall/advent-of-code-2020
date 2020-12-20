#[macro_use]
extern crate lazy_static;

use log::{debug, info};
use std::collections::HashMap;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use parser::*;

// --- model

type TileID = usize;
type EdgePattern = u64;

trait Reversible {
    fn reversed(self) -> Self;
}

#[derive(Debug)]
struct Tile {
    id: TileID,
    top: EdgePattern,
    left: EdgePattern,
    right: EdgePattern,
    bottom: EdgePattern
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, EnumIter)]
enum Orientation {
    R0,
    R90,
    R180,
    R270,
    R0FlipH,
    R0FlipV,
    R90FlipH,
    R90FlipV
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize
}

#[derive(Debug, Eq, PartialEq)]
enum TilePlacement<'a> {
    None,
    Placed {
        orientation: Orientation,
        tile: &'a Tile
    }
}

struct Arrangement<'a> {
    width: usize,
    height: usize,
    fixed_tiles: [[TilePlacement<'a>; 12]; 12],
    available_tiles: HashMap<TileID, &'a Tile>,
}

#[derive(Debug)]
struct Constraint {
    top: Option<EdgePattern>,
    bottom: Option<EdgePattern>,
    left: Option<EdgePattern>,
    right: Option<EdgePattern>
}

impl Eq for Tile {}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a> Default for TilePlacement<'a> {
    fn default() -> Self { TilePlacement::None }
}

impl Default for Constraint {
    fn default() -> Self {
        Constraint {
            top: None,
            bottom: None,
            left: None,
            right: None
        }
    }
}

lazy_static! {
    static ref REVERSE_EDGE_PATTERNS: Vec<EdgePattern> = {
        let mut reversed = Vec::with_capacity(1 << 10);
        for i in 0..(1<<10) {
            let mut rev = 0;
            for bit in 0..10 {
                if i & (1 << bit) != 0 {
                    rev |= 0x200 >> bit;
                }
            }
            reversed.push(rev);
        }
        reversed
    };
}

impl Reversible for EdgePattern {
    fn reversed(self) -> Self {
        REVERSE_EDGE_PATTERNS[self as usize]
    }
}

impl Tile {
    fn top_edge_in_orientation(&self, orientation: Orientation) -> EdgePattern {
        match orientation {
            Orientation::R0 => self.top,
            Orientation::R90 => self.right,
            Orientation::R180 => self.bottom.reversed(),
            Orientation::R270  => self.left.reversed(),
            Orientation::R0FlipH => self.top.reversed(),
            Orientation::R0FlipV => self.bottom,
            Orientation::R90FlipH => self.right.reversed(),
            Orientation::R90FlipV => self.left
        }
    }

    fn bottom_edge_in_orientation(&self, orientation: Orientation) -> EdgePattern {
        match orientation {
            Orientation::R0 => self.bottom,
            Orientation::R90 => self.left,
            Orientation::R180 => self.top.reversed(),
            Orientation::R270  => self.right.reversed(),
            Orientation::R0FlipH => self.bottom.reversed(),
            Orientation::R0FlipV => self.top,
            Orientation::R90FlipH => self.left.reversed(),
            Orientation::R90FlipV => self.right
        }
    }

    fn left_edge_in_orientation(&self, orientation: Orientation) -> EdgePattern {
        match orientation {
            Orientation::R0 => self.left,
            Orientation::R90 => self.top.reversed(),
            Orientation::R180 => self.right.reversed(),
            Orientation::R270  => self.bottom,
            Orientation::R0FlipH => self.right,
            Orientation::R0FlipV => self.left.reversed(),
            Orientation::R90FlipH => self.bottom.reversed(),
            Orientation::R90FlipV =>self.top
        }
    }

    fn right_edge_in_orientation(&self, orientation: Orientation) -> EdgePattern {
        match orientation {
            Orientation::R0 => self.right,
            Orientation::R90 => self.bottom.reversed(),
            Orientation::R180 => self.left.reversed(),
            Orientation::R270  => self.top,
            Orientation::R0FlipH => self.left,
            Orientation::R0FlipV => self.right.reversed(),
            Orientation::R90FlipH => self.top.reversed(),
            Orientation::R90FlipV => self.bottom
        }
    }
}

impl<'a> Arrangement<'a> {
    fn new(width: usize, height: usize, tiles: &[&'a Tile]) -> Self {
        Arrangement {
            width,
            height,
            fixed_tiles: Default::default(),
            available_tiles: tiles.iter().map(|tile| (tile.id, *tile)).collect()
        }
    }

    fn place(&mut self, pos: &Pos, orientation: Orientation, tile_id: TileID) {
        if let Some(tile) = self.available_tiles.remove(&tile_id) {
            self.fixed_tiles[pos.y][pos.x] = TilePlacement::Placed { orientation, tile };
            debug!("place {} {:?} at {:?}", tile_id, orientation, pos);
            debug!("{:?}", self);
        } else {
            panic!("trying to place unavailable tile");
        }
    }

    fn remove(&mut self, pos: &Pos) {
        match self.fixed_tiles[pos.y][pos.x] {
            TilePlacement::None => {},
            TilePlacement::Placed { orientation: _, tile } => {
                self.available_tiles.insert(tile.id, tile);
                self.fixed_tiles[pos.y][pos.x] = TilePlacement::None;
                debug!("remove {} from {:?}", tile.id, pos);
                debug!("{:?}", self);
            }
        }
    }

    fn tile_id_at(&self, pos: &Pos) -> Option<TileID> {
        match self.fixed_tiles[pos.y][pos.x] {
            TilePlacement::None => None,
            TilePlacement::Placed { orientation: _, tile } => Some(tile.id)
        }
    }

    fn has_placed_neighbour(&self, x: usize, y: usize) -> bool {
        (x > 0 && self.fixed_tiles[y][x-1] != TilePlacement::None)
            || (y > 0 && self.fixed_tiles[y-1][x] != TilePlacement::None)
            || (x < self.width-1 && self.fixed_tiles[y][x+1] != TilePlacement::None)
            || (y < self.height-1 && self.fixed_tiles[y+1][x] != TilePlacement::None)
    }  

    fn placement_positions(&self) -> impl Iterator<Item = Pos> + '_ {
        (0..self.width).flat_map(move |y|
            (0..self.height).filter_map(move |x|
                if self.fixed_tiles[y][x] == TilePlacement::None && self.has_placed_neighbour(x, y) {
                    Some(Pos { x, y })
                } else {
                    None
                }
            )
        )
    }

    fn constraints_at_position(&self, pos: &Pos) -> Constraint {
        use TilePlacement::*;

        let mut constraints = Constraint::default();

        if pos.y > 0 {
            if let Placed { orientation, tile } = self.fixed_tiles[pos.y-1][pos.x] {
                constraints.top = Some(tile.bottom_edge_in_orientation(orientation));
            }
        }
        if pos.x > 0 {
            if let Placed { orientation, tile } = self.fixed_tiles[pos.y][pos.x-1] {
                constraints.left = Some(tile.right_edge_in_orientation(orientation));
            }
        }
        if pos.y < self.width - 1 {
            if let Placed { orientation, tile } = self.fixed_tiles[pos.y+1][pos.x] {
                constraints.bottom = Some(tile.top_edge_in_orientation(orientation));
            }
        }
        if pos.x < self.height - 1 {
            if let Placed { orientation, tile } = self.fixed_tiles[pos.y][pos.x+1] {
                constraints.right = Some(tile.left_edge_in_orientation(orientation));
            }
        }
        constraints
    }

    fn can_place(&self, tile_id: TileID, pos: &Pos, orientation: Orientation) -> bool {
        if let Some(tile) = self.available_tiles.get(&tile_id) {
            let constraints = self.constraints_at_position(pos);
            if let Some(top) = constraints.top {
                if tile.top_edge_in_orientation(orientation) != top {
                    debug!("can't place {} {:?} at {:?}, top={:x} must be {:x}", tile_id, orientation, pos, tile.top_edge_in_orientation(orientation), top);
                    return false;
                }
            }
            if let Some(bottom) = constraints.bottom {
                if tile.bottom_edge_in_orientation(orientation) != bottom {
                    debug!("can't place {} {:?} at {:?}, bottom={:x} must be {:x}", tile_id, orientation, pos, tile.bottom_edge_in_orientation(orientation), bottom);
                    return false;
                }
            }
            if let Some(left) = constraints.left {
                if tile.left_edge_in_orientation(orientation) != left {
                    debug!("can't place {} {:?} at {:?}, left={:x} must be {:x}", tile_id, orientation, pos, tile.left_edge_in_orientation(orientation), left);
                    return false;
                }
            }
            if let Some(right) = constraints.right {
                if tile.right_edge_in_orientation(orientation) != right {
                    debug!("can't place {} {:?} at {:?}, top={:x} must be {:x}", tile_id, orientation, pos, tile.right_edge_in_orientation(orientation), right);
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    fn try_arrange(&mut self) -> bool {
        if self.available_tiles.is_empty() {
            true
        } else {
            let positions: Vec<Pos> = self.placement_positions().collect();
            debug!("available positions {:?}", &positions);

            let available_ids: Vec<TileID> = self.available_tiles.values().map(|tile| tile.id).collect();
            available_ids.into_iter().find(|tile_id| {
                Orientation::iter().find(|orientation| {
                    positions.iter().find(|position| 
                        if !self.can_place(*tile_id, position, *orientation) {
                            false
                        } else {
                            self.place(position, *orientation, *tile_id);
                            if self.try_arrange() {
                                true
                            } else {
                                self.remove(position);
                                false
                            }
                        }
                    ).is_some()
                }).is_some()
            }).is_some()
        }
    }
}

impl<'a> fmt::Debug for Arrangement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.fixed_tiles[y][x] {
                    TilePlacement::None => write!(f, "---- ")?,
                    TilePlacement::Placed { orientation: _, tile } => write!(f, "{:4} ", tile.id)?
                }
            }
            write!(f, "\n")?;
        };
        Ok(())
    }
}

fn arrange_tiles<'a>(width: usize, height: usize, tiles: &Vec<&'a Tile>) -> Option<Arrangement<'a>> {
    tiles.iter().filter_map(|tile|
        Orientation::iter().filter_map(|orientation| {
            info!("trying {} {:?} in start position", tile.id, orientation);
            let mut arrangement = Arrangement::new(width, height, tiles);
            arrangement.place(&Pos { x: 0, y: 0 }, orientation, tile.id);
            if arrangement.try_arrange() {
                Some(arrangement)
            } else {
                None
            }
        }).next()
    ).next()
}

// -- parser

fn decode_row(cells: &Vec<EdgePattern>) -> EdgePattern {
    cells.iter().fold(0, |pattern, cell|
        (pattern << 1) | cell
    )
}

fn decode_column(cells: &Vec<Vec<EdgePattern>>, column: usize) -> EdgePattern {
    cells.iter().fold(0, |pattern, row|
        (pattern << 1) | row[column]
    ) 
}

fn parse_input(input: &str) -> ParseResult<Vec<Tile>> {
    let tile_id = integer
        .between(match_literal("Tile "), match_literal(":\n"))
        .map(|i| i as TileID);

    let tile_char = any_char.pred(|c| *c == '#').means(1).or(any_char.pred(|c| *c == '.').means(0));
    let tile_row = whitespace_wrap(one_or_more(tile_char));
    let tile = pair(tile_id, one_or_more(tile_row), |id, cells| 
        Tile {
            id,
            top: decode_row(&cells[0]),
            bottom: decode_row(&cells[cells.len()-1]),
            left: decode_column(&cells, 0),
            right: decode_column(&cells, cells[0].len()-1)
        }
    );

    one_or_more(tile).parse(input)
}

// -- problems

fn part1(tiles: &Vec<&Tile>) -> Option<usize> {
    let corners = vec![
        Pos { x:  0, y:  0 },
        Pos { x:  0, y: 11 },
        Pos { x: 11, y:  0 },
        Pos { x: 11, y: 11 }
    ];

    arrange_tiles(12, 12, tiles).map(|arrangement|
        corners.iter().filter_map(|c| arrangement.tile_id_at(c)).product()
    )
}

fn main() {
    env_logger::init();
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let tiles = parse_input(&input).unwrap().1;
    let tiles_by_ref: Vec<&Tile> = tiles.iter().collect();
    println!("part 1 {:?}", part1(&tiles_by_ref));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> String {
        std::fs::read_to_string("./example.txt").unwrap()
    }

    fn example_tiles() -> Vec<Tile> {
        let input = example_input();
        let tiles = parse_input(input.as_str());
        assert!(tiles.is_ok());
        tiles.unwrap().1        
    }

    #[test]
    fn test_parser() {
        assert_eq!(example_tiles()[0], Tile {
            id: 2311,
            top: 0x0d2,
            bottom: 0x0e7,
            left: 0x1f2,
            right: 0x059
        });
    }

    #[test]
    fn test_orientations_iter() {
        let ors: Vec<Orientation> = Orientation::iter().collect();
        assert_eq!(ors.len(), 8);
    }

    #[test]
    fn test_orientations() {
        use Orientation::*;
        let tile = Tile {
            id: 1,
            top: 0x2F9,
            bottom: 0x077,
            left: 0x325,
            right: 0x16D
        };
        
        assert_eq!(tile.top_edge_in_orientation(R0), 0x2F9);
        assert_eq!(tile.bottom_edge_in_orientation(R0), 0x077);
        assert_eq!(tile.left_edge_in_orientation(R0), 0x325);
        assert_eq!(tile.right_edge_in_orientation(R0), 0x16D);

        assert_eq!(tile.top_edge_in_orientation(R90), 0x16D);
        assert_eq!(tile.bottom_edge_in_orientation(R90), 0x325);
        assert_eq!(tile.left_edge_in_orientation(R90), 0x27D);
        assert_eq!(tile.right_edge_in_orientation(R90), 0x3B8);

        assert_eq!(tile.top_edge_in_orientation(R180), 0x3B8);
        assert_eq!(tile.bottom_edge_in_orientation(R180), 0x27D);
        assert_eq!(tile.left_edge_in_orientation(R180), 0x2DA);
        assert_eq!(tile.right_edge_in_orientation(R180), 0x293);

        assert_eq!(tile.top_edge_in_orientation(R270), 0x293);
        assert_eq!(tile.bottom_edge_in_orientation(R270), 0x2DA);
        assert_eq!(tile.left_edge_in_orientation(R270), 0x077);
        assert_eq!(tile.right_edge_in_orientation(R270), 0x2F9);
    }

    #[test]
    fn test_placement_positions() {
        let tiles = example_tiles();
        let tiles_by_ref: Vec<&Tile> = tiles.iter().collect();
        let mut arrangement = Arrangement::new(3, 3, &tiles_by_ref);
        arrangement.place(&Pos { x: 0, y: 0 }, Orientation::R0, 2311);
        let positions: Vec<Pos> = arrangement.placement_positions().collect();
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&Pos { x: 1, y: 0 }));
        assert!(positions.contains(&Pos { x: 0, y: 1 }));
    }

    #[test]
    fn test_constraints_r0() {
        let tiles = example_tiles();
        let tiles_by_ref: Vec<&Tile> = tiles.iter().collect();
        let mut arrangement = Arrangement::new(3, 3, &tiles_by_ref);
        arrangement.place(&Pos { x: 0, y: 0 }, Orientation::R0, 2311);

        let constraints = arrangement.constraints_at_position(&Pos { x: 1, y: 0 });
        assert_eq!(constraints.top, None);
        assert_eq!(constraints.bottom, None);
        assert_eq!(constraints.right, None);
        assert_eq!(constraints.left, Some(0x059));
    }

    #[test]
    fn test_constraints_r180() {
        let tiles = example_tiles();
        let tiles_by_ref: Vec<&Tile> = tiles.iter().collect();
        let mut arrangement = Arrangement::new(3, 3, &tiles_by_ref);

        arrangement.place(&Pos { x: 0, y: 0 }, Orientation::R180, 1951);
        let constraints = arrangement.constraints_at_position(&Pos { x: 1, y: 0 });
        assert_eq!(constraints.top, None);
        assert_eq!(constraints.bottom, None);
        assert_eq!(constraints.right, None);
        assert_eq!(constraints.left, Some(0x24b));

        let constraints = arrangement.constraints_at_position(&Pos { x: 0, y: 1 });
        assert_eq!(constraints.top, Some(0x18d));
        assert_eq!(constraints.bottom, None);
        assert_eq!(constraints.right, None);
        assert_eq!(constraints.left, None);

        arrangement.place(&Pos { x: 1, y: 0 }, Orientation::R180, 2311);
        let constraints = arrangement.constraints_at_position(&Pos { x: 2, y: 0 });
        assert_eq!(constraints.top, None);
        assert_eq!(constraints.bottom, None);
        assert_eq!(constraints.right, None);
        assert_eq!(constraints.left, Some(0x13e));
    }

    #[test]
    fn test_can_place() {
        let tiles = example_tiles();
        let tiles_by_ref: Vec<&Tile> = tiles.iter().collect();
        let mut arrangement = Arrangement::new(3, 3, &tiles_by_ref);

        arrangement.place(&Pos { x: 0, y: 0 }, Orientation::R0FlipV, 1951);
        let constraints = arrangement.constraints_at_position(&Pos { x: 1, y: 0 });
        assert_eq!(constraints.top, None);
        assert_eq!(constraints.bottom, None);
        assert_eq!(constraints.right, None);
        assert_eq!(constraints.left, Some(0x13e));
    }

    #[test]
    fn test_arrangement() {
        let tiles = example_tiles();
        let tiles_by_ref: Vec<&Tile> = tiles.iter().collect();
        let arrangement = arrange_tiles(3, 3, &tiles_by_ref);
        assert!(arrangement.is_some());
        let arrangement = arrangement.unwrap();
        let corners = vec![
            arrangement.tile_id_at(&Pos { x: 0, y: 0 }),
            arrangement.tile_id_at(&Pos { x: 2, y: 0 }),
            arrangement.tile_id_at(&Pos { x: 0, y: 2 }),
            arrangement.tile_id_at(&Pos { x: 2, y: 2 })
        ];
        assert!(corners.contains(&Some(1951)));
        assert!(corners.contains(&Some(3079)));
        assert!(corners.contains(&Some(2971)));
        assert!(corners.contains(&Some(1171)));
    }
}
