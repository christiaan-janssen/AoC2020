
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Debug)]
pub struct Map {
    pub map_width: i32,
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new(map_width: i32, map_str: String) -> Self {


        fn map_from_str(map_string: String) -> Vec<TileType>{
            let mut map = Vec::new();
            for c in map_string.chars() {
                if c == '.' {
                    map.push(TileType::Floor);
                } else {
                    map.push(TileType::Wall);
                }
            }
            map
        }
        Self {
            map_width,
            tiles: map_from_str(map_str),
        }
    }

    /// Convert (x,y) coords to map idx
    pub fn map_idx(&self, x: i32, y: i32) -> usize {
        ((y * self.map_width) + x) as usize
    }
}
