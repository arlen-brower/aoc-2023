use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let binding = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let contents = binding.trim_end();

    let v: Vec<&str> = contents.split('\n').collect();

    let mut id = 0;
    let mut sumid = 0;
    let mut sumpow = 0;

    for line in v {
        let revealed: Vec<&str> = line.split(':').collect();
        let cubes: Vec<&str> = revealed[1].split(';').collect();
        //Game ID
        id = id + 1;
        
        let mut g = 0;
        let mut r = 0;
        let mut b = 0;

        println!("{}", revealed[0]);
        for round in cubes {
            println! {"\t{round}"};
            let colours: Vec<&str> = round.split(',').collect();
            for colour in colours {
                let n: Vec<&str> = colour.trim().split(' ').collect();
               

                let num = n[0].parse::<i32>().unwrap();
                let col = n[1];

                if col.contains("green") && g < num {
                    g = num; 
                }
                if col.contains("red") && r < num {
                    r = num;
                }
                if col.contains("blue") && b < num {
                    b = num;
                }
            }
        }

        sumpow = sumpow + (r * g * b);
        
        if r <= 12 && g <= 13 && b <= 14 {
            sumid = sumid + id;
        }
    }

    println!("P1) Sum of IDs: {sumid}");
    println!("P2) Sum of Powers: {sumpow}");
}
