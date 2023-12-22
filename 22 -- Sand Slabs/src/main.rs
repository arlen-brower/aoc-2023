use core::cmp::Ordering;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Block {
    x: i32,
    y: i32,
    z: i32,
    label: usize,
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        self.z
            .cmp(&other.z)
            .then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.x.cmp(&other.x))
    }
}

impl Block {
    fn from_str(coords: &str, l: usize) -> Block {
        let v = coords
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        Block {
            x: v[0],
            y: v[1],
            z: v[2],
            label: l,
        }
    }
}

fn main() {
    let file_path = env::args().nth(1).unwrap_or("test_input".to_string());
    let binding = fs::read_to_string(&file_path).expect("Should have been able to read the file");
    let contents = binding.trim();

    let start = Instant::now();

    solve(contents);

    println!("---\ntime: {:?}", Instant::now().duration_since(start));
}

fn solve(contents: &str) {
    let mut brick_map: HashMap<usize, Vec<Block>> = HashMap::new();
    let mut occupied: HashMap<(i32, i32, i32), usize> = HashMap::new();

    let mut brick_list: Vec<(Block, Block)> = Vec::new();
    let mut id = 1;
    for line in contents.lines() {
        let (start, end) = line
            .split('~')
            .map(|s| Block::from_str(s, id))
            .collect_tuple::<(Block, Block)>()
            .unwrap();

        brick_list.push((start, end));
        id += 1;
    }

    let max_id = id;
    brick_list.sort_by(
        |(Block { z: sz1, .. }, Block { z: ez1, .. }),
         (Block { z: sz2, .. }, Block { z: ez2, .. })| {
            sz1.cmp(sz2).then_with(|| ez1.cmp(ez2))
        },
    );
    for (b, _) in &brick_list {
        println!("{}", b.label);
    }
    id = 1;
    for (start, end) in brick_list {
        let mut brick: Vec<Block> = Vec::new();
        let diff: (i32, i32, i32) = (
            (end.x - start.x).abs(),
            (end.y - start.y).abs(),
            (end.z - start.z).abs(),
        );

        let mut to_add: Block;

        match diff {
            (_, 0, 0) => {
                for i in start.x..=end.x {
                    to_add = Block {
                        x: i,
                        y: start.y,
                        z: start.z,
                        label: id,
                    };
                    brick.push(to_add);
                    occupied.insert((to_add.x, to_add.y, to_add.z), to_add.label);
                }
            }
            (0, _, 0) => {
                for i in start.y..=end.y {
                    to_add = Block {
                        x: start.x,
                        y: i,
                        z: start.z,
                        label: id,
                    };
                    brick.push(to_add);
                    occupied.insert((to_add.x, to_add.y, to_add.z), to_add.label);
                }
            }
            (0, 0, _) => {
                for i in start.z..=end.z {
                    to_add = Block {
                        x: start.x,
                        y: start.y,
                        z: i,
                        label: id,
                    };
                    brick.push(to_add);
                    occupied.insert((to_add.x, to_add.y, to_add.z), to_add.label);
                }
            }
            _ => panic!(),
        }
        brick_map.insert(id, brick);
        id += 1;
    }
    let mut support_map: Vec<Vec<usize>> = vec![vec![0; max_id]; max_id];

    // debug_print(&mut occupied);
    //
    for i in 1..max_id {
        let mut collision = false;
        while !collision {
            let supports = move_brick(&mut brick_map, &mut occupied, i);

            for s in supports {
                collision = true;
                // 0 is code for ground; ignore it
                if s == 0 {
                    // continue;
                    // Actually, this caused an off-by-one error
                }
                // i is being supported by s
                support_map[i][s] = 1;
            }
        }
    }

    // println!("---");
    debug_print(&mut occupied);
    //
    // print!("   1 ");
    // for i in 2..max_id {
    //     print!("{} ", i);
    // }
    // println!();
    //
    // for i in 1..max_id {
    //     print!("{} |", i);
    //     for s in 1..max_id {
    //         print!("{} ", support_map[i][s]);
    //     }
    //     println!();
    // }

    let mut dis_count = 0;
    for c in 1..max_id {
        let mut dis = true;
        for r in 1..max_id {
            if support_map[r][c] == 1 {
                // if we are supporting something
                let check = &support_map[r];
                // println!("{} supporting {}", c, r);
                if check.iter().sum::<usize>() <= 1 {
                    dis = false;
                }
            }
        }
        if dis {
            dis_count += 1;
            // println!("Can disintegrate block {}", c);
        }
    }
    println!("Part 1) {}", dis_count);

    let mut p2 = 0;
    for i in 1..max_id {
        p2 += num_destroyed(&support_map, i, &mut HashSet::new(), max_id);
    }
    println!("Part 2) {}", p2);
    // for i in 1..max_id {
    //     for s in 1..max_id {
    //         if support_map[i][s] == 1 {
    //             println!("{} supported by {}", i, s);
    //         }
    //     }
    // }
    // println!("---");
    //
    // for i in 1..max_id {
    //     for s in 1..max_id {
    //         if support_map[s][i] == 1 {
    //             println!("{} supports {}", i, s);
    //         }
    //     }
    // }
}

fn num_destroyed(
    support_map: &Vec<Vec<usize>>,
    target: usize,
    destroyed: &mut HashSet<usize>,
    max_id: usize,
) -> usize {
    let mut bricks_fall = 0;
    let mut new_dest: Vec<usize> = Vec::new();
    destroyed.insert(target);
    for r in 1..max_id {
        if support_map[r][target] == 1 {
            // if we are supporting something
            let check = &support_map[r];
            let mut filter = true;
            for (id, ch) in check.iter().enumerate() {
                if *ch == 1 && !destroyed.contains(&id) {
                    filter = false;
                }
            }
            if !destroyed.contains(&r) && filter {
                new_dest.push(r);
                destroyed.insert(r);
                bricks_fall += 1;
            }
        }
    }

    for n in new_dest {
        bricks_fall += num_destroyed(&support_map, n, destroyed, max_id);
    }

    bricks_fall
}

fn debug_print(occupied: &mut HashMap<(i32, i32, i32), usize>) {
    for z in (1..178).rev() {
        print!("{:>3}|", z);
        for y in 0..10 {
            let mut ch = '.';
            for x in 0..10 {
                ch = match occupied.get(&(x, y, z)) {
                    Some(id) => {
                        let nid = id.to_string().chars().nth(0).unwrap();
                        if ch != '.' && ch != nid {
                            '?'
                        } else {
                            nid
                        }
                    }
                    None => {
                        if ch != '.' {
                            ch
                        } else {
                            '.'
                        }
                    }
                };
            }
            print!("{ch}");
        }
        println!();
    }
}

fn move_brick(
    brick_map: &mut HashMap<usize, Vec<Block>>,
    occupied: &mut HashMap<(i32, i32, i32), usize>,
    id: usize,
) -> Vec<usize> {
    let blocks: &Vec<Block> = brick_map.get(&id).unwrap();
    let mut new_brick: Vec<Block> = Vec::new();

    let mut supports: Vec<usize> = Vec::new();
    let mut collides = false;
    for block in blocks {
        let new_block = Block {
            x: block.x,
            y: block.y,
            z: block.z - 1,
            label: block.label,
        };

        if new_block.z < 1 {
            collides = true;
            supports.push(0); // 0 means ground
        }

        if occupied.contains_key(&(new_block.x, new_block.y, new_block.z)) {
            let conflict = occupied
                .get(&(new_block.x, new_block.y, new_block.z))
                .unwrap();

            if id != *conflict {
                supports.push(*conflict);
                collides = true;
            }
        }

        new_brick.push(new_block);
    }

    if !collides {
        for ob in blocks {
            occupied.remove(&(ob.x, ob.y, ob.z));
        }

        for nb in &new_brick {
            occupied.insert((nb.x, nb.y, nb.z), nb.label);
        }

        brick_map.insert(id, new_brick);
    }
    supports
}

