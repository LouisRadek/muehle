use crate::game_state::Token;

pub fn print_board(positions: [Token; 24]) {
    println!("{}------------{}------------{}                    0------------1------------2", positions[0], positions[1], positions[2]);
    println!("|            |            |                    |            |            |");
    println!("|   {}--------{}--------{}   |                    |   8--------9-------10   |", positions[8], positions[9], positions[10]);
    println!("|   |        |        |   |                    |   |        |        |   |");
    println!("|   |   {}----{}----{}   |   |                    |   |  16---17---18   |   |", positions[16], positions[17], positions[18]);
    println!("|   |   |         |   |   |                    |   |   |         |   |   |");
    println!("{}---{}---{}         {}---{}---{}                    7--15--23        19--11---3", positions[7], positions[15], positions[23], positions[19], positions[11], positions[3]);
    println!("|   |   |         |   |   |                    |   |   |         |   |   |");
    println!("|   |   {}----{}----{}   |   |                    |   |  22---21---20   |   |", positions[22], positions[21], positions[20]);
    println!("|   |        |        |   |                    |   |        |        |   |");
    println!("|   {}--------{}--------{}   |                    |  14-------13-------12   |", positions[14], positions[13], positions[12]);
    println!("|            |            |                    |            |            |");
    println!("{}------------{}------------{}                    6------------5------------4", positions[6], positions[5], positions[4]);
}

pub fn print_introduction_text() {
    println!("Welcome to my small mill console game.");
    println!("For the moves please use the Syntax and the indices printed beside the game board.");
    println!("Syntax: start_position,end_position or in the set phase: end_position (for the position use the indices).");
    println!("Have fun playing and may the odds be ever in your favor!\n");
}
