mod sudoku_solver_dancinglinks;
mod sudoku_solver_backtrack;

fn main() {
    let board1 = "200005709900070000070001050008200000067000520000008100040300080000060002609800003";
    let board2 = "000700002200001030006000091100247060000000000070169008640000300080900005900008000";
    let board3 = "104000800850070000000004500000207003900050004200601000002800000000010035007000108";
    let board4 = "074000000200080030009265000450300002000000000900007065000873900090040007000000120";
    let board5 = "080300015000090640001060900070004000002000400000100080007010800069030000820005030";

    println!("----------Dancing Links Algorithm----------");
    println!("{:?}", sudoku_solver_dancinglinks::solve_sudoku(board1));
    println!("{:?}", sudoku_solver_dancinglinks::solve_sudoku(board2));
    println!("{:?}", sudoku_solver_dancinglinks::solve_sudoku(board3));
    println!("{:?}", sudoku_solver_dancinglinks::solve_sudoku(board4));
    println!("{:?}", sudoku_solver_dancinglinks::solve_sudoku(board5));
    println!("----------Backtrack Algorithm----------");
    println!("{:?}", sudoku_solver_backtrack::Sudoku::new(board1).solve());
    println!("{:?}", sudoku_solver_backtrack::Sudoku::new(board2).solve());
    println!("{:?}", sudoku_solver_backtrack::Sudoku::new(board3).solve());
    println!("{:?}", sudoku_solver_backtrack::Sudoku::new(board4).solve());
    println!("{:?}", sudoku_solver_backtrack::Sudoku::new(board5).solve());

}
