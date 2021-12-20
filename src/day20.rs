use std::fs;

fn pad_image(imagev: &mut Vec<Vec<usize>>, val: usize) -> Vec<Vec<usize>> {
    imagev.iter_mut()
        .for_each(|row| {
            row.insert(0, val);
            row.push(val);
        });

    // pad with zeros top and bottom
    let mut padded_image = Vec::new();
    padded_image.push(vec![val; imagev[0].len()]);
    let mut padding_bot = padded_image.clone();
    padded_image.append(imagev);
    padded_image.append(&mut padding_bot);
    padded_image
}

fn part1(path: &str, steps: usize) -> usize {
    let contents = fs::read_to_string(path).unwrap();
    calc(&contents[..], steps)
}

fn calc(contents: &str, steps: usize) -> usize {
    // eprintln!("contents = {:?}", contents);
    let parts = contents.split("\n\n").collect::<Vec<&str>>();
    let algo = parts[0];
    let image = parts[1];

    let algov = algo.trim().lines()
    .fold(String::from(""), |acc, line| format!("{}{}", acc, line));

    let mut imagev = image.trim().lines()
        .map(|line| {
            line.chars()
            .map(|c| {
                match c {
                    '.' => 0,
                    '#' => 1,
                    inv => panic!("Invalid image char: {}", inv)
                }
            }).collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut zeroval = 0;

    imagev = pad_image(&mut imagev, zeroval);
    imagev = pad_image(&mut imagev, zeroval);


    // println!("{}", render(&imagev));
    for step in 1..=steps {
        println!("step {}", step);

        imagev = pad_image(&mut imagev, zeroval);
        imagev = pad_image(&mut imagev, zeroval);
        imagev = pad_image(&mut imagev, zeroval);
        imagev = pad_image(&mut imagev, zeroval);
        imagev = pad_image(&mut imagev, zeroval);
        imagev = pad_image(&mut imagev, zeroval);
        imagev = pad_image(&mut imagev, zeroval);
        imagev = pad_image(&mut imagev, zeroval);
        imagev = pad_image(&mut imagev, zeroval);
        zeroval = 1 - zeroval;
        // println!("PAD\n{}", render(&imagev));
        // pad with zeros top and bottom
        let image_height = imagev.len();
        let image_width = imagev[0].len();

        // 3x3 window
        imagev = (1..image_height-1)
            .map(|h| {
                (1..image_width-1)
                    .map(move |w| (h, w))
                    .map(|(h, w)| {
                        let idx = read_window(&imagev, h, w);
                        match algov.chars().nth(idx).unwrap() {
                            '.' => 0,
                            '#' => 1,
                            inv => panic!("Invalid image char: {}", inv)
                        }
                    })
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        
        let shrink = 7;
        imagev = imagev[shrink..imagev.len()-shrink].iter()
            .map(|row| row[shrink..row.len()-shrink].iter().cloned().collect::<Vec<usize>>() )
            .collect::<Vec<Vec<usize>>>();    
        // println!("{}", render(&imagev));
    }
    println!("{}", render(&imagev));
    let shrink = 2;
    imagev = imagev[shrink..imagev.len()-shrink].iter()
            .map(|row| row[shrink..row.len()-shrink].iter().cloned().collect::<Vec<usize>>() )
            .collect::<Vec<Vec<usize>>>();    
    println!("{}", render(&imagev));
    imagev.iter()
        .flat_map(|row| row.iter() )
        .sum()
}

fn read_window(imagev: &Vec<Vec<usize>>, h: usize, w: usize) -> usize {
    let sum = (h-1..=h+1)
        .flat_map(|r| (w-1..=w+1).map(move |c| (r, c)))
        .enumerate()
        .map(|(idx, (r, c))| imagev[r][c] << (8-idx))
        .sum();
    sum
}

fn render(imagev: &Vec<Vec<usize>>) -> String {
    imagev.iter()
        .map(|row| {
            let mut s = row.iter()
                .map(|c| {
                    match c {
                        0 => ".",
                        1 => "#",
                        _ => panic!("invalid")
                    }
                }).collect::<String>();
            s.push_str("\n");
            s
        }).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_day20() {
        assert_eq!(5425, part1("data/input20.txt", 2));
    }

    #[test]
    fn part1_day20_example() {
        assert_eq!(35, part1("data/input20_example.txt", 2));
    }
    #[test]
    fn part2_day20() {
        assert_eq!(14052, part1("data/input20.txt", 50));
    }

    #[test]
    fn part2_day20_example() {
        assert_eq!(3351, part1("data/input20_example.txt", 50));
    }
}