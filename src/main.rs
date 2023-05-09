use clap::Parser;
use std::time::SystemTime;

use sudoku_stefan2502::sudoku::Sudoku;


#[derive(Parser)]
#[clap(version = "1.0.0", author = "Stefan L. <stefan.lang@med.lu.se>")]
struct Opts {
    /// the difficulty of the generated sudoku (1=easy, 2=medium, 3=hard)
    #[clap(default_value_t= 1 ,short, long)]
    difficulty: usize,
    /// the column separator for the file
    #[clap(short, long)]
    outfile: String,
}

fn main() {
    let now = SystemTime::now();

    let opts: Opts = Opts::parse();

    let mut sudoku = Sudoku::new();
    let filled_cells:usize;

    // sudoku.print_help();
    // panic!("This should now have shown all options!");

    match &opts.difficulty {
        3 => {
            println!("sudoku_stefan2502 creating a hard sudoku");
            filled_cells = 20;
        },
        2 => {
            println!("sudoku_stefan2502 creating a medium sudoku");
            filled_cells = 40;
        },
        1 => {
            println!("sudoku_stefan2502 creating an easy sudoku");
            filled_cells = 60;
        },
        _ => panic!("Sorry - difficulty only accepts one of [1,2,3] - not {}", &opts.difficulty)
    };

    for it in 1..5 {
        if sudoku.rand_init( 20 ) {
            println!("Filling the Sudoku with 20 random entries has worked in it {it}");
            break;
        }else {
           println!("The initial filling failed!");
           sudoku.print_help();
           sudoku = Sudoku::new(); 
        }
    }
    println!("Filled an initial set of values");
    let sudoku_save = sudoku.clone();
    sudoku.print();

    while sudoku.is_incomplete() {
        sudoku.solve();
        if sudoku.is_incomplete(){
            println!("Failed to fill a random starter {}", sudoku.solved );
            //sudoku.print();
            sudoku.reset();
            for _it in 1..100 {
                if sudoku.rand_init( 20 ) {
                    break;
                }
            }
        }
    }

    let sudoku_save = sudoku.clone();


    
    if sudoku.purge( 9*9 - filled_cells ){
        println!("This solved sudoku:" );
        sudoku_save.print();
        println!("A help system:");
        sudoku.print_help();
        println!("To be filled in:");
        sudoku.print();
    }
    else {
        println!("I could not purge this sudoku but possibly it is good enough?");
        println!("This solved sudoku:" );
        sudoku_save.print();

        println!("To be filled in:");
        sudoku.print();

        sudoku.reset_help();
        println!("A help system:");
        sudoku.print_help();
    }

    
    //println!("The solved sudoku:");

    // sudoku_save.print();

    // println!("This sudoku had {} solution(s):", sudoku.solved );
    // sudoku.print();

    match now.elapsed() {
        Ok(elapsed) => {
            let mut milli = elapsed.as_millis();

            let mil = milli % 1000;
            milli= (milli - mil) /1000;

            let sec = milli % 60;
            milli= (milli -sec) /60;

            let min = milli % 60;
            milli= (milli -min) /60;

            println!("finished in {milli} h {min} min {sec} sec {mil} milli sec");
        },
        Err(e) => {println!("Error: {e:?}");}
    }
}
