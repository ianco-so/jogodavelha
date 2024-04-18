use std::io;

const SIZE: usize = 3;

fn main() {
    let mut board = [[' '; SIZE]; SIZE];
    let mut current_player = 'X';

    loop {
        print_board(&board);

        println!("Jogador {}: Escolha uma posição (1-9):", current_player); //Zero sai do jogo
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erro ao ler entrada.");
        
        let position: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Posição inválida. Tente novamente.");
                continue;
            }
        };

        if position == 0 {
            println!("Jogo encerrado.");
            break;
        }

        let row = (position - 1) / SIZE;
        let col = (position - 1) % SIZE;

        if position < 1 || position > 9 || board[row][col] != ' ' {
            println!("Posição inválida. Tente novamente.");
            continue;
        }

        board[row][col] = current_player;

        if check_winner(&board, current_player) {
            print_board(&board);
            println!("Jogador {} venceu!", current_player);
            break;
        }

        if board_full(&board) {
            print_board(&board);
            println!("Empate!");
            break;
        }

        current_player = if current_player == 'X' { 'O' } else { 'X' };
    }
}

fn print_board(board: &[[char; SIZE]; SIZE]) {
    for row in board {
        println!(" {} | {} | {}", row[0], row[1], row[2]);
        println!("---|---|---");
    }
}

fn check_winner(board: &[[char; SIZE]; SIZE], player: char) -> bool {
    for i in 0..SIZE {
        if board[i][0] == player && board[i][1] == player && board[i][2] == player {
            return true; // Linhas
        }
        if board[0][i] == player && board[1][i] == player && board[2][i] == player {
            return true; // Colunas
        }
    }
    if board[0][0] == player && board[1][1] == player && board[2][2] == player {
        return true; // Diagonal principal
    }
    if board[0][2] == player && board[1][1] == player && board[2][0] == player {
        return true; // Diagonal secundária
    }
    false
}

fn board_full(board: &[[char; SIZE]; SIZE]) -> bool {
    for row in board {
        for &cell in row {
            if cell == ' ' {
                return false;
            }
        }
    }
    true
}
