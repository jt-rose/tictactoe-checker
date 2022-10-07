fn main() {
    let result = is_solved(&[&[1,0,1], &[1,0,2], &[1,2,2]]);
    println!("The result is {:?}", result);
}

fn is_solved(board: &[&[u8; 3]; 3]) -> i8 {
    // check if a row, column, or diagonal have the same X's (1) or O's (2)
    for row in board {
        for value in 1 .. 3 {
            if check_same_value(row, value) {
                return value as i8
            }
        }
    }

    // check for winning moves in columns
    for idx in 0 .. 3 {
        let mut column: [u8; 3] = [ 0, 0, 0];
        column[0] = board[0][idx];
        column[1] = board[1][idx];
        column[2] = board[2][idx];

        for value in 1 .. 3 {
            if check_same_value(&column, value) {
                return value as i8
            }
        }
    }

    // check for winning moves in diagonal positions
    let diagonal_a = [0,1,2].map(| idx | {
        return board[idx][idx]
    });

    let mut diagonal_b: [u8; 3] = [0,0,0];
    diagonal_b[0] = board[0][2];
    diagonal_b[1] = board[1][1];
    diagonal_b[2] = board[2][0];

    for value in 1 .. 3 {
        if check_same_value(&diagonal_a, value) || check_same_value(&diagonal_b, value) {
            return value as i8
        }
    }

    // return -1 for an unfinished game
    for row in board {
        if row.contains(&0) {
            return -1
        }
    }

    // if no winner found and no spaces left, return 0 for a draw
    0
}

fn check_same_value(list: &[u8; 3], target_value: u8) -> bool {
    list.iter().all(|num| num == &target_value)
}