use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./sample-01.txt").expect("Something went wrong reading the file");

    let seat_linex: Vec<String> = contents.split_whitespace().map(|s| s.to_string()).collect();

    println!("{}", solution(seat_linex));
}

fn get_new_half(range: &mut (i32, i32), action: char) {
    println!("{:?}", range);
    let middle = (((range.0 + range.1) / 2) as f32).ceil() as i32;
    match action {
        'F' | 'L' => {
            range.1 = middle;
        }
        'B' | 'R' => range.0 = middle + 1,
        _ => {}
    };
}

fn solution(seat_lines: Vec<String>) -> i32 {
    let mut max_seat_id = i32::MIN;

    for line in seat_lines {
        let rows = &line[..line.len() - 3];
        let cols = &line[line.len() - 3..];
        let mut row = (0, 127);
        let mut col = (0, 7);

        for l in rows.chars() {
            get_new_half(&mut row, l);
        }

        for l in cols.chars() {
            get_new_half(&mut col, l);
        }
        let seat_id = row.0 * 8 + col.0;
        println!("row: {:?} col: {:?} ID: {}", row, col, seat_id);

        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
    }

    return max_seat_id;
}
