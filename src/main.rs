struct GraphStepper<'a> {
    transitions: &'a [(usize, char, usize)],
    finite_info: &'a [bool],
    current_vertex : usize,
}
impl<'a> GraphStepper<'a> {
    fn new(transitions: &'a [(usize, char, usize)], finite_info: &'a [bool] ) 
        -> std::result::Result<Self, &'static str> {

        // Checking graph
        if !finite_info.iter().any(|&f| { f == true }) {
            return Err("Graph has not any finite vertex");
        }

        Ok( Self { transitions, finite_info, current_vertex: 0 } )
    }
    fn step(&mut self, symbol: char) -> bool 
    {
        let transition = self.transitions.iter()
        .find(|x| { &&x.0 == &&self.current_vertex && &&x.1 == &&symbol });

        if let None = transition {
            return false;
        }

        let transition = transition.unwrap();

        self.current_vertex = transition.2; // next vertex

        true
    }
    fn is_complete(&self) -> bool {
        self.finite_info[self.current_vertex]
    }
}

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


    let mut stepper = GraphStepper::new(&transitions, &[false, false, true]).unwrap();

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
