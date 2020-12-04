use crate::utils::map::*;


pub fn check_map(map_vec: &Vec<String>) -> i64 {
    let map_width = map_vec[0].len();
    let map_height = map_vec.len();
    let mut map_str: String = String::new();
    for line in map_vec {
        map_str.push_str(line.as_str());
    }

    let map: Map = Map::new(map_width as i32, map_str);
    let mut x = 3;
    let mut trees = 0;

    for y in 1 .. map_height  {
        let idx = map.map_idx(x % map_width as i32,y as i32);
        if idx <= map.tiles.len()  {
            if map.tiles[idx] == TileType::Wall {
                trees += 1;
            }
            x += 3;
        }
    }
    trees
}

#[test]
fn test_the_path() {
    assert_eq!(7, check_map(&lines_from_file("../3.test")));
}