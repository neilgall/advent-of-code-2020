#[macro_use]
extern crate lazy_static;

use log::{debug, info};
use std::collections::{HashMap, HashSet};
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

#[derive(Debug)]
struct Tile {
    id: TileID,
    top: EdgePattern,
    left: EdgePattern,
    right: EdgePattern,
    bottom: EdgePattern
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, EnumIter, Hash)]
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

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct OrientedTile {
    tile_id: TileID,
    orientation: Orientation
}

impl Eq for Tile {}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Relationship {
    Above,
    Below,
    LeftOf,
    RightOf
}

#[derive(Debug)]
struct AllowedOrientedTiles {
    neighbours: HashMap<(TileID, Orientation, Relationship), HashSet<OrientedTile>>,
    empty: HashSet<OrientedTile>
}

impl AllowedOrientedTiles {
    fn new(tiles: &Vec<&Tile>) -> Self {
        let mut allowed = HashMap::new();
        for tile in tiles.iter() {
            for orientation in Orientation::iter() {
                let mut above = HashSet::new();
                let mut below = HashSet::new();
                let mut left_of = HashSet::new();
                let mut right_of = HashSet::new();

                for candidate in tiles.iter().filter(|t| t.id != tile.id) {
                    for candidate_orientation in Orientation::iter() {
                        if candidate.bottom_edge_in_orientation(candidate_orientation) == tile.top_edge_in_orientation(orientation) {
                            above.insert(OrientedTile { tile_id: candidate.id, orientation: candidate_orientation });
                        }
                        if candidate.top_edge_in_orientation(candidate_orientation) == tile.bottom_edge_in_orientation(orientation) {
                            below.insert(OrientedTile { tile_id: candidate.id, orientation: candidate_orientation });
                        }
                        if candidate.left_edge_in_orientation(candidate_orientation) == tile.right_edge_in_orientation(orientation) {
                            right_of.insert(OrientedTile { tile_id: candidate.id, orientation: candidate_orientation });
                        }
                        if candidate.right_edge_in_orientation(candidate_orientation) == tile.left_edge_in_orientation(orientation) {
                            left_of.insert(OrientedTile { tile_id: candidate.id, orientation: candidate_orientation });
                        }
                    }
                }

                allowed.insert((tile.id, orientation, Relationship::Above), above);
                allowed.insert((tile.id, orientation, Relationship::Below), below);
                allowed.insert((tile.id, orientation, Relationship::LeftOf), left_of);
                allowed.insert((tile.id, orientation, Relationship::RightOf), right_of);
            }
        }

        AllowedOrientedTiles {
            neighbours: allowed,
            empty: HashSet::new()
        }
    }

    fn get(&self, tile_id: TileID, orientation: Orientation, relationship: Relationship) -> &'_ HashSet<OrientedTile> {
        self.neighbours.get(&(tile_id, orientation, relationship)).unwrap_or(&self.empty)
    }
} 

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Pos {
    x: i64,
    y: i64
}

impl Pos {
    fn up(&self) -> Pos {
        Pos { x: self.x, y: self.y - 1 }
    }

    fn down(&self) -> Pos {
        Pos { x: self.x, y: self. y + 1 }
    }

    fn left(&self) -> Pos {
        Pos { x: self.x - 1, y: self.y }
    }

    fn right(&self) -> Pos {
        Pos { x: self.x + 1, y: self.y }
    }

    fn neighbours(&self) -> impl Iterator<Item=Pos> + '_ {
        let mut i = 0;
        std::iter::from_fn(move || {
            let n = i;
            i += 1;
            match n {
                0 => Some(self.up()),
                1 => Some(self.down()),
                2 => Some(self.left()),
                3 => Some(self.right()),
                _ => None
            }
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
enum TilePlacement<'a> {
    None,
    Placed {
        orientation: Orientation,
        tile: &'a Tile
    }
}

impl<'a> Default for TilePlacement<'a> {
    fn default() -> Self { TilePlacement::None }
}

struct OrientedTileSet {
    initialised: bool,
    oriented_tiles: HashSet<OrientedTile>
}

impl OrientedTileSet {
    fn new() -> Self {
        OrientedTileSet {
            initialised: false,
            oriented_tiles: HashSet::new()
        }
    }

    fn restrict_to(&mut self, neighbours: &HashSet<OrientedTile>) {
        if self.initialised {
            self.oriented_tiles = self.oriented_tiles.intersection(neighbours).cloned().collect();
        } else {
            self.oriented_tiles = neighbours.clone();
            self.initialised = true;
        }
    }

    fn is_empty(&self) -> bool {
        self.initialised && self.oriented_tiles.is_empty()
    }
}

#[derive(Debug)]
enum SearchResult<T> {
    Empty,
    Found(T),
    InvalidPlacement(TileID)
}

struct Arrangement<'a> {
    width: i64,
    height: i64,
    allowed_neighbours: &'a AllowedOrientedTiles,
    fixed_tiles: [[TilePlacement<'a>; 12]; 12],
    available_tiles: HashMap<TileID, &'a Tile>,
    next_positions: HashSet<Pos>
}


impl<'a> Arrangement<'a> {
    fn new(width: i64, height: i64, tiles: &[&'a Tile], allowed_neighbours: &'a AllowedOrientedTiles) -> Self {
        Arrangement {
            width,
            height,
            allowed_neighbours,
            fixed_tiles: Default::default(),
            available_tiles: tiles.iter().map(|tile| (tile.id, *tile)).collect(),
            next_positions: HashSet::new()
        }
    }

    fn place(&mut self, pos: &Pos, orientation: Orientation, tile_id: TileID) {
        if let Some(tile) = self.available_tiles.remove(&tile_id) {
            self.fixed_tiles[pos.y as usize][pos.x as usize] = TilePlacement::Placed { orientation, tile };
            self.next_positions.remove(pos);
            for n in pos.neighbours() {
                if self.valid(&n) && self.tile_at(&n) == &TilePlacement::None {
                    self.next_positions.insert(n);
                }
            }
            debug!("place {} {:?} at {:?}", tile_id, orientation, pos);
            debug!("{:?}", self);
        } else {
            panic!("trying to place unavailable tile");
        }
    }

    fn remove(&mut self, pos: &Pos) {
        match self.fixed_tiles[pos.y as usize][pos.x as usize] {
            TilePlacement::None => {},
            TilePlacement::Placed { orientation: _, tile } => {
                self.available_tiles.insert(tile.id, tile);
                self.fixed_tiles[pos.y as usize][pos.x as usize] = TilePlacement::None;
                self.next_positions.insert(*pos);
                debug!("remove {} from {:?}", tile.id, pos);
                debug!("{:?}", self);
            }
        }
    }

    fn valid(&self, pos: &Pos) -> bool {
        0 <= pos.x && 0 <= pos.y && pos.x < self.width && pos.y < self.height
        && (
            // edges only
            pos.x == 0 || pos.y == 0 || pos.x == self.width-1 || pos.y == self.height-1
        )
    }

    fn tile_at(&self, pos: &Pos) -> &'a TilePlacement {
        if self.valid(pos) {
            &self.fixed_tiles[pos.y as usize][pos.x as usize]
        } else {
            &TilePlacement::None
        }
    }

    fn tile_id_at(&self, pos: &Pos) -> Option<TileID> {
        match self.tile_at(pos) {
            TilePlacement::None => None,
            TilePlacement::Placed { orientation: _, tile } => Some(tile.id)
        }
    }

    fn possible_orientations(&self, pos: &Pos) -> SearchResult<HashSet<OrientedTile>> {
        let mut possible = OrientedTileSet::new();

        if let TilePlacement::Placed { tile, orientation } = self.tile_at(&pos.left()) {
            possible.restrict_to(self.allowed_neighbours.get(tile.id, *orientation, Relationship::RightOf));
        }

        if let TilePlacement::Placed { tile, orientation } = self.tile_at(&pos.up()) {
            possible.restrict_to(self.allowed_neighbours.get(tile.id, *orientation, Relationship::Below));
            if possible.is_empty() {
                debug!("invalid tile {:?} above {:?}", tile.id, pos);
                return SearchResult::InvalidPlacement(tile.id);
            }
        }

        if let TilePlacement::Placed { tile, orientation } = self.tile_at(&pos.right()) {
            possible.restrict_to(self.allowed_neighbours.get(tile.id, *orientation, Relationship::LeftOf));
            if possible.is_empty() {
                debug!("invalid tile {:?} right of {:?}", tile.id, pos);
                return SearchResult::InvalidPlacement(tile.id);
            }
        }

        if let TilePlacement::Placed { tile, orientation } = self.tile_at(&pos.down()) {
            possible.restrict_to(self.allowed_neighbours.get(tile.id, *orientation, Relationship::Above));
            if possible.is_empty() {
                debug!("invalid tile {:?} below {:?}", tile.id, pos);
                return SearchResult::InvalidPlacement(tile.id);
            }
        }

        if possible.initialised {
            SearchResult::Found(possible.oriented_tiles)
        } else {
            SearchResult::Empty
        }
    } 

    fn possible_placements(&self) -> SearchResult<Vec<(Pos, HashSet<OrientedTile>)>> {
        let mut placements = vec![];
        for pos in self.next_positions.iter() {
            let search = self.possible_orientations(pos);
            // debug!("possible placements {:?} {:?}", pos, search);
            match search {
                SearchResult::Empty => {},
                SearchResult::Found(tile_set) => placements.push((*pos, tile_set)),
                SearchResult::InvalidPlacement(tile_id) => return SearchResult::InvalidPlacement(tile_id)
           }
        }
        SearchResult::Found(placements)
    }

    fn try_arrange(&mut self) -> SearchResult<()> {
        if self.next_positions.is_empty() {
            SearchResult::Found(())
        } else {
            match self.possible_placements() {
                SearchResult::Empty => SearchResult::Empty,
                SearchResult::InvalidPlacement(tile_id) => SearchResult::InvalidPlacement(tile_id),
                SearchResult::Found(placements) => {
                    for (position, oriented_tiles) in placements.iter() {
                        for tile in oriented_tiles.iter() {
                            self.place(position, tile.orientation, tile.tile_id);
                            match self.try_arrange() {
                                SearchResult::InvalidPlacement(tile_id) if tile_id != tile.tile_id => {
                                    self.remove(position);
                                    return SearchResult::InvalidPlacement(tile_id);
                                }
                                SearchResult::Found(_) => {
                                    return SearchResult::Found(());
                                }
                                SearchResult::Empty | SearchResult::InvalidPlacement(_) => {
                                    self.remove(position);
                                }
                            }
                        }
                    }
                    SearchResult::Empty
                }
            }
        }
    }
}

impl<'a> fmt::Debug for Arrangement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            write!(f, "\n")?;
            for x in 0..self.width {
                match self.tile_at(&Pos { x, y }) {
                    TilePlacement::None => write!(f, "---- ")?,
                    TilePlacement::Placed { orientation: _, tile } => write!(f, "{:4} ", tile.id)?
                }
            }
        };
        Ok(())
    }
}

fn arrange_tiles<'a>(width: i64, height: i64, tiles: &Vec<&'a Tile>, neighbours: &'a AllowedOrientedTiles) -> Option<Arrangement<'a>> {
    tiles.iter().filter_map(|tile|
        Orientation::iter().filter_map(|orientation| {
            info!("trying {} {:?} in start position", tile.id, orientation);
            let mut arrangement = Arrangement::new(width, height, tiles, &neighbours);
            arrangement.place(&Pos { x: 0, y: 0 }, orientation, tile.id);
            match arrangement.try_arrange() {
                SearchResult::Found(()) => Some(arrangement),
                _ => None
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

    let allowed_neighbours = AllowedOrientedTiles::new(tiles);

    arrange_tiles(12, 12, tiles, &allowed_neighbours).map(|arrangement|
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

    fn init_logging() {
         let _ = env_logger::builder().is_test(true).try_init();
    }

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
    fn test_allowed_neighbours() {
        use Orientation::*;
        use Relationship::*;

        let tiles = example_tiles();
        let tiles_by_ref: Vec<&Tile> = tiles.iter().collect();
        let allowed_neighbours = AllowedOrientedTiles::new(&tiles_by_ref);

        assert!(allowed_neighbours.get(1951, R0FlipV, Below).contains(&OrientedTile { tile_id: 2729, orientation: R0FlipV }));
        assert!(allowed_neighbours.get(1951, R0FlipV, RightOf).contains(&OrientedTile { tile_id: 2311, orientation: R0FlipV }));
        assert!(allowed_neighbours.get(2729, R0FlipV, Below).contains(&OrientedTile { tile_id: 2971, orientation: R0FlipV }));
        assert!(allowed_neighbours.get(2311, R0FlipV, RightOf).contains(&OrientedTile { tile_id: 3079, orientation: R0 }));
    }

    #[test]
    fn test_possible_placements() {
        let tiles = example_tiles();
        let tiles_by_ref: Vec<&Tile> = tiles.iter().collect();
        let allowed_neighbours = AllowedOrientedTiles::new(&tiles_by_ref);
        let mut arrangement = Arrangement::new(3, 3, &tiles_by_ref, &allowed_neighbours);
        arrangement.place(&Pos { x: 0, y: 0 }, Orientation::R0FlipV, 1951);
        match arrangement.possible_placements() {
            SearchResult::InvalidPlacement(_) => panic!("invalid placement"),
            SearchResult::Empty => panic!("nothing found"),
            SearchResult::Found(positions) => {
                let positions: Vec<Pos> = positions.into_iter().map(|(p, _)| p).collect();
                assert_eq!(positions.len(), 2);
                assert!(positions.contains(&Pos { x: 1, y: 0 }));
                assert!(positions.contains(&Pos { x: 0, y: 1 }));
            }
        }
    }

    #[test]
    fn test_arrangement() {
        init_logging();

        let tiles = example_tiles();
        let tiles_by_ref: Vec<&Tile> = tiles.iter().collect();
        let allowed_neighbours = AllowedOrientedTiles::new(&tiles_by_ref);
        let arrangement = arrange_tiles(3, 3, &tiles_by_ref, &allowed_neighbours);
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
