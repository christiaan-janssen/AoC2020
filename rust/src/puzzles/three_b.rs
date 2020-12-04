use crate::utils::map::*;


pub fn check_map_with_slope(map_vec: &Vec<String>, right: i32, down: i32) -> i64 {
    let map_width = map_vec[0].len();
    let map_height = map_vec.len();
    let mut map_str: String = String::new();
    for line in map_vec {
        map_str.push_str(line.as_str());
    }

    let map: Map = Map::new(map_width as i32, map_str);
    let mut x = right;

    let mut trees = 0;

    for y in (1 .. map_height).step_by(down as usize)  {
        let idx = map.map_idx(x % map_width as i32,y as i32);
        if idx <= map.tiles.len()  {
            if map.tiles[idx] == TileType::Wall {
                trees += 1;
            }
            x += right;
        }
    }
    trees
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_path_1_by_1() {
        assert_eq!(2, check_map_with_slope(&lines_from_file("../3.test"), 1, 1));
    }

    #[test]
    fn test_the_path_3_by_1() {
        assert_eq!(7, check_map_with_slope(&lines_from_file("../3.test"), 3, 1));
    }

    #[test]
    fn test_the_path_5_by_1() {
        assert_eq!(3, check_map_with_slope(&lines_from_file("../3.test"), 5, 1));
    }

    #[test]
    fn test_the_path_7_by_1() {
        assert_eq!(4, check_map_with_slope(&lines_from_file("../3.test"), 7, 1));
    }

    #[test]
    fn test_the_path_1_by_2() {
        assert_eq!(2, check_map_with_slope(&lines_from_file("../3.test"), 1, 2));
    }
}