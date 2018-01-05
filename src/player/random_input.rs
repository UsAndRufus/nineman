use rand::{thread_rng, Rng};

use player::InputHandler;

pub struct RandomInput {}

impl RandomInput {
    //TODO: information about currently available nodes??
    fn get_node(&self) -> String {
        let nodes = vec!["0nw","0n","0ne","0e","0se","0s","0sw","0w",
                         "1nw","1n","1ne","1e","1se","1s","1sw","1w",
                         "2nw","2n","2ne","2e","2se","2s","2sw","2w"];
        thread_rng().choose(&nodes).unwrap().to_string()
    }
}

impl InputHandler for RandomInput {
    fn get_placement(&self) -> String {
        self.get_node()
    }

    fn get_move(&self) -> (String, String) {
        (self.get_node(), self.get_node())
    }

    fn get_mill(&self) -> String {
        self.get_node()
    }
}
