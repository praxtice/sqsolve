extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn print_board_ans(board: [[u32;3];3]) {
    println!("\n\n\n");
    println!("\t[{}][{}][{}]",board[0][0],board[1][0],board[2][0]);
    println!("\t[{}][{}][{}]",board[0][1],"X",board[2][1]);
    println!("\t[{}][{}][{}]",board[0][2],board[1][2],board[1][2]);
    println!("\n\n\n");
}

fn get_board_total(board: [[u32;3];3]) -> [u32;4] {

    let total: [u32; 4];
    total = [
        board[0][0]*board[0][1]*board[0][2],
        board[0][0]*board[1][0]*board[2][0],
        board[2][0]*board[2][1]*board[2][2],
        board[0][2]*board[1][2]*board[2][2],
    ];
    return total;
}

fn random_board() -> [[u32;3];3] {
    let mut nums = vec![1,2,3,4,5,6,7,8,9];
    let mut boa: [[u32;3];3] = [
        [0,0,0],
        [0,0,0],
        [0,0,0],
    ];
    for x in 0..3 {
        for y in 0..3 {
            let r = rand::thread_rng().gen_range(0,8) % nums.len();
            boa[x][y] = nums[r];
            nums.remove(r);
        }
    }
    // print_board_ans(boa);
    return boa;
}

fn main() {
    // answers
    let mut ans: [u32; 4] = [0,0,0,0];

    let mut count = 0;

    println!("\n\n\n\t   \t[A]    ");
    println!("\t   \t[::][::][::][B]");
    println!("\t   \t[::][::][::]");
    println!("\t     [D][::][::][::]");
    println!("\t   \t         [C]");
    println!("\nEnter A, B, C, and D");

    loop {
        if count >= ans.len() {
            break;
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let inp: u32 = input.trim().parse()
            .expect("type a number");
        ans[count] = inp;
        count += 1;
    }

    let ans = ans;

    let mut r_board: [[u32;3];3] = [
        [1,1,1],
        [1,1,1],
        [1,1,1]
    ];
    count = 0;

    loop {
        r_board = random_board();
        match get_board_total(r_board).cmp(&ans) {
            Ordering::Equal => {
                println!(" Solution found:");
                print_board_ans(r_board);
                println!("attemts: {}", count);
                break;
            },
            Ordering::Less  => {print!("\t");},
            Ordering::Greater => {print!("\t");},
        }
        count+= 1;
        if count >= 362880 {
            println!("{}", count);
            println!("impossible board");
            break;
        }
    }
}
