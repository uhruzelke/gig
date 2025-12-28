use crate::gig_lib::{self, Gig};
use std::io::{self, Write};

fn clear_screen() {
    // Clear the entire screen and move cursor to top-left
    print!("\x1B[2J\x1B[1;1H");
    // Flush to ensure the escape codes are sent immediately
    io::stdout().flush().unwrap();
}

pub fn gig_vec_to_string(Gvec: Vec<&Gig>) -> String {
    let mut s = "".to_string();
    for g in Gvec {
        let mut line = "- [".to_string();
        line.push(match &g.status {
            gig_lib::GigStatus::TODO => ' ',
            gig_lib::GigStatus::DONE => 'X',
            gig_lib::GigStatus::CUSTOM(s) => s.clone().pop().unwrap_or('?'),
        });
        line.push_str("] ");
        line.push_str(&g.name.clone());
        line.push_str("\n");
        s.push_str(&line);
    }
    s
}

impl gig_lib::GigList {
    pub fn print_gig_list(&mut self, scope: gig_lib::GigSelectionScope) {
        clear_screen();
        let s = gig_vec_to_string(self.get_gigs(scope));
        print!("{s}");
    }
}
