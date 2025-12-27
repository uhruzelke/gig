use std::io::{self, Write};
use crate::gig_lib::{self, Gig};

fn clear_screen() {
    // Clear the entire screen and move cursor to top-left
    print!("\x1B[2J\x1B[1;1H");
    // Flush to ensure the escape codes are sent immediately
    io::stdout().flush().unwrap();
}

pub fn gig_vec_to_string(vec:Vec<&Gig>) -> String{
    let s ="".to_string();
    for g in vec{
        let mut line= "- [".to_string();
        line.push(match &g.status {
            gig_lib::GigStatus::TODO => ' ',
            gig_lib::GigStatus::DONE => 'X',
            gig_lib::GigStatus::CUSTOM(s) => s.clone().pop().unwrap_or(' '),
        });
        line.push_str("] ");
        line.push_str(&g.name.clone());

    }
    todo!("a bit");

}


impl gig_lib::GigList {
    pub fn print_gig_list(mut self,scope: gig_lib::GigSelectionScope) {
        clear_screen();
    }
    
}