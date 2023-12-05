fn main() {
    // Hardcoded input, so I'm only including the test input here
    let seeds: Vec<i64> = vec![79, 14, 55, 13];

    let mut maps: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    maps.push(vec![(50, 98, 2), (52, 50, 48)]);
    maps.push(vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)]);
    maps.push(vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)]);
    maps.push(vec![(88, 18, 7), (18, 25, 70)]);
    maps.push(vec![(45, 77, 23), (81, 45, 19), (68, 64, 13)]);
    maps.push(vec![(0, 69, 1), (1, 0, 69)]);
    maps.push(vec![(60, 56, 37), (56, 93, 4)]);

    let mut t_seeds: Vec<i64> = Vec::new();
    for s in seeds {
        let mut transform = s;
        for map in &maps {
            transform = seed_map(transform, &map);
        }
        t_seeds.push(transform);
        //println!("{transform}");
    }

    let min_loc = t_seeds.iter().min().unwrap();
    println!("Minimum Location: {min_loc} ");
}

fn seed_map(s: i64, maps: &Vec<(i64, i64, i64)>) -> i64 {
    let mut ret_val = s;
    for (dest, src, rng) in maps {
        let offset: i64 = dest - src;
        let check = src..&(src + rng);
        if check.contains(&&s) {
            ret_val = s + offset;
        }
    }
    ret_val
}
