pub struct DFA<'a> {
    transitions: &'a [(usize, char, usize)],
    finite_info: &'a [bool],
    current_vertex : usize,
}
impl<'a> DFA<'a> {
    pub fn new(transitions: &'a [(usize, char, usize)], finite_info: &'a [bool] ) 
        -> std::result::Result<Self, &'static str> {

        // Checking graph
        if !finite_info.iter().any(|&f| { f == true }) {
            return Err("Graph has not any finite vertex");
        }

        Ok( Self { transitions, finite_info, current_vertex: 0 } )
    }
    pub fn step(&mut self, symbol: char) -> bool 
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
    pub fn is_complete(&self) -> bool {
        self.finite_info[self.current_vertex]
    }
}