use std::fs::read_to_string;

const N: i32 = 10000;

fn read_lines(file_name: &str) -> Vec<String> {
    read_to_string(file_name)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn read_input(line: &str) -> (i32, i32) {
    let mut substr_iter = line.split_whitespace();
    let mut next_num = || -> i32 {
        substr_iter
            .next()
            .expect("Not enough input numbers")
            .parse()
            .expect("Input is not a number")
    };

    (next_num(), next_num())
}

fn print_result(pair: (i32, i32), id: &mut [i32]) {
    print!("{} {}\t", pair.0, pair.1);
    for i in 0..10 {
        print!(" {} ", id[i]);
    }
    println!();
}

fn main() {
    let mut id: [i32; N as usize] = [0; N as usize];
    for i in 0..N {
        id[i as usize] = i;
    }
    let lines = read_lines("./resources/userinput.txt");
    for input_line in lines {
        let pair = read_input(&input_line);
        let mut i = pair.0;
        while i != id[i as usize] {
            i = id[i as usize];
        }
        let mut j = pair.1;
        while j != id[j as usize] {
            j = id[j as usize];
        }
        if i == j {
            print_result(pair, &mut id);
            continue;
        };
        id[i as usize] = j;

        print_result(pair, &mut id);
    }
}
