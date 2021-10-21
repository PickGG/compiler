mod dfa;
use dfa::DFA;

fn main() {
    let transitions  = 
    [ 
        // vertex, symbol, next vertex, is finite
        // N
        (0, '+', 1),
        (0, '-', 1),

        (0, '0', 2),
        (0, '1', 2),
        (0, '2', 2),
        (0, '3', 2),
        (0, '4', 2),
        (0, '5', 2),
        (0, '6', 2),
        (0, '7', 2),
        (0, '8', 2),
        (0, '9', 2),

        // Q1
        (1, '0', 2),
        (1, '1', 2),
        (1, '2', 2),
        (1, '3', 2),
        (1, '4', 2),
        (1, '5', 2),
        (1, '6', 2),
        (1, '7', 2),
        (1, '8', 2),
        (1, '9', 2),

        // Q2
        (2, '0', 2),
        (2, '1', 2),
        (2, '2', 2),
        (2, '3', 2),
        (2, '4', 2),
        (2, '5', 2),
        (2, '6', 2),
        (2, '7', 2),
        (2, '8', 2),
        (2, '9', 2),
    ];


    let chain = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Argument error: No input string");
        std::process::exit(1);
    });


    let mut stepper = DFA::new(&transitions, &[false, false, true]).unwrap();

    for ch in chain.chars() {
        if stepper.step(ch) == false {
            eprintln!("Compilation error: Syntax error");
            std::process::exit(2);
        }
    }
    if stepper.is_complete() {
        std::process::exit(0);
    }
    else {
        eprintln!("Compilation error: Unexpected end of input string");
        std::process::exit(3);
    }

}
