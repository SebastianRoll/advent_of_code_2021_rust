use std::fs;
use std::collections::HashMap;

pub fn part1(path: &str) -> u32 {
    let string = fs::read_to_string(path).unwrap();
    let bf = BasinFinder::from(string.as_str());

    let part1 = bf.part1();
    part1
}

pub fn part2(path: &str) -> usize {
    let string = fs::read_to_string(path).unwrap();
    let bf = BasinFinder::from(string.as_str());
    
    let part2 = bf.part2();
    part2

}
type Coordinate = (usize, usize);


struct BasinFinder {
    contents: Vec<Vec<u8>>,
}

impl From<&str> for BasinFinder {
    fn from(s: &str) -> Self {
        let contents = s.lines()
        .map(|l| l.chars()
        .map(|c| c as u8 - 48) // ASCII
        .collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
        
        BasinFinder{contents}
    }
}

impl BasinFinder {
    fn part1(&self) -> u32 {
        let mut low = Vec::new();
        let contents = &self.contents;
        for (r, row) in contents.iter().enumerate() {
            for (c, v) in row.iter().enumerate() {
                if r != 0 && contents[r-1][c] <= *v 
                || c != 0 && contents[r][c-1] <= *v
                || r != contents.len()-1 && contents[r+1][c] <= *v
                || c != row.len()-1 && contents[r][c+1] <= *v {
                    continue
                } else {
                    low.push(*v + 1);
                }
            }
        }
        low.iter().map(|&v| v as u32).sum()
        }

    fn part2(self) -> usize {
        let mut basin_id = 1;
        let mut basins: HashMap<Coordinate, usize> = HashMap::new();
        for (r, row) in self.contents.iter().enumerate() {
            for (c, _v) in row.iter().enumerate() {
                &self.search(r, c, &mut basins, basin_id);
                basin_id += 1;
            }
        }

        let mut basin_sizes: HashMap<usize, usize> = HashMap::new();
        basins.iter()
        .for_each(|(_key, basin_id)| *basin_sizes.entry(*basin_id).or_insert(0) += 1 );
        let mut biggest_basin_sizes = basin_sizes.into_values().collect::<Vec<usize>>();
        biggest_basin_sizes.sort();
        biggest_basin_sizes
        .iter()
        .rev()
        .take(3)
        .cloned()
        .product()
    }

    fn search(&self, r: usize, c:usize, basins: &mut HashMap<Coordinate, usize>, basin_id: usize) {
        if self.contents[r][c] == 9
        || basins.contains_key(&(r,c)) {
            return
        }
        basins.insert((r, c), basin_id);
        if r != 0 {
            self.search(r-1, c, basins, basin_id)
        }
        if r != self.contents.len() -1 {
            self.search(r+1, c, basins, basin_id)
        }
        if c != 0 {
            self.search(r, c-1, basins, basin_id)
        }
        if c != self.contents[0].len() -1 {
            self.search(r, c+1, basins, basin_id)
        }
    }
}