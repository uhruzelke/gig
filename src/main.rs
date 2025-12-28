use crate::gig_lib::{Gig, GigSelectionScope};
use std::io::{self, BufRead, Read, Stdin, Write};
mod gig_lib;
mod ui;



// main need minimal argument processing code

fn main() {
    let mut list = gig_lib::GigList::new_empty(Some("TODO".to_string()));
    list.add_gig(Gig::minimal_gig_from_string("- [X] do work".to_string()));
    list.add_gig(Gig::minimal_gig_from_string("- [ ] don't work".to_string()));
    list.add_gig(Gig::minimal_gig_from_string("- [X] pet the cat".to_string()));
    list.add_gig(Gig::minimal_gig_from_string("- [S] don't pet the dog".to_string()));
    list.add_gig(Gig::minimal_gig_from_string("- [ ] do coding".to_string()));
    list.add_gig(Gig::minimal_gig_from_string("- [X] hate on js programers".to_string()));
    list.add_gig(Gig::minimal_gig_from_string("- [ ] play games".to_string()));
    list.add_gig(Gig::minimal_gig_from_string("- [?] do school work".to_string()));
    let mut buffer = String::new();
    while  buffer.trim() != "q".to_string(){
        list.print_gig_list(GigSelectionScope::new_empty()); 
        print!("░▒▓█🭬 ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_line(&mut buffer);
    }
}
