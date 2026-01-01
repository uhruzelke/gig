use std::{io::{self, BufRead, Read, Stdin, Write}, option};
use crate::gig_lib::{self, Gig, GigEnvironment, GigSelectionScope, GigStatus};

enum Command {
    QUIT,
    ADDTODO(Gig),
    STATUSCHANGE(GigStatus, usize),
    GETGIGS(GigSelectionScope),
}

pub struct Session {
    gig_env: GigEnvironment,
}

fn clear_screen() {
    // Clear the entire screen and move cursor to top-left
    print!("\x1B[2J\x1B[1;1H");
    // Flush to ensure the escape codes are sent immediately
    io::stdout().flush().unwrap();
}

impl Session {
    pub fn initilize(path:Option<String>)-> Self{
        let gigs = match path {
            Some(p) => match GigEnvironment::load(p.clone()) {
                Ok(g) => g,
                Err(_) => GigEnvironment::new_save(p, None),
            },
            None => GigEnvironment::new("".to_string(),None),
        };
        Self{gig_env:gigs}

    } 
    pub fn run(&mut self){
        let mut running = true;
        while running {
            clear_screen();
            self.gig_env.default_list.print_gig_list(GigSelectionScope::new_empty()); 
            print!("░▒▓█🭬 ");
            io::stdout().flush().unwrap();
            let input = Self::get_input();
            let mut args:Vec<&str> = input.split_whitespace().collect();
            args.reverse();
            running =self.interpret_command(&mut args);

            

            
        }
    }
    pub fn interpret_command(&mut self, args: &mut Vec<&str> ) -> bool{ // returns whether the thing is running
            let command = args.pop();
            let mut remove:Option<String> = None;
            match command {
                Some(y) => match y {
                    "q" => {
                        let _ = self.gig_env.save().unwrap();
                        return false;
                    },
                    "add" => self.add_gig(args.to_vec()),
                    "done" => {
                        match self.find_gig(args) {
                            Some(g) => g.0.status = GigStatus::DONE,
                            None => {println!("gig not found" ); return false;},
                        };
                    },
                    "rm" =>{
                        match self.find_gig(args) {
                            Some(g) => remove = Some(g.1),
                            None => {println!("gig not found" ); return false;},
                        };
                    },
                    "save" => {let _ = self.gig_env.save();},
                    _ => {}
                    
                },
                None => {},
            };
            match remove {
                Some(r) =>{ let _ =self.gig_env.default_list.list.remove(&r);},
                None => {},
            };
            return true;

    }
    pub fn find_gig(&mut self, args:&mut Vec<&str>)-> Option<(&mut Gig, String)>{ // returns the key and the reff for the list
        let name = self.take_arg_or_input(args, "what gig do you want?");
        println!(" looking for gig {}.", name);
        let string = self.gig_env.default_list.find_gig_name(name);
        match string.0 {
            Some(g) => Some((g, string.1)), // its jsut the name
            None => None, // lets try somthing else
        }

    }
    pub fn take_arg_or_input(&mut self, args:&mut Vec<&str>, question:&str) -> String{
            match args.pop(){
                Some(n) => n.to_string(),
                None => {
                    println!("{}",question);
                    Self::get_input()
                },
            }

    }
    pub fn add_gig(&mut self,mut args:Vec<&str>){
            let name:String = self.take_arg_or_input(&mut args,"name?");
            let done:bool =match self.take_arg_or_input(&mut args,"done? X = yes, all other imput no").as_str(){
                "x" => true,
                "X" => true,
                _ => false
                
            }; 
            let mut g =Gig::new_empty();
            g.name = name;
            g.status = match done {
                true => GigStatus::DONE,
                false => GigStatus::TODO,
            };
            self.gig_env.default_list.add_gig(g);
    }
    pub fn get_input()-> String{
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_line(&mut buffer);
        buffer.trim().to_string()

    }
    
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
