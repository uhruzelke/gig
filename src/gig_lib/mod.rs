use serde::{Deserialize, Serialize};
use std::collections::{HashMap, hash_map};
use std::fs::{self, File};
use std::io::{Error, prelude::*};

/*
- [ ] timer need to be done
- [ ] to do list needs to be done
- [ ] to do list needs to be able to be saved and loaded
- [ ] default configuration with .conf dir
- [ ] ui needs to be done


# plan
    - [ ] todo lists can be saved and loaded at any time with all the items contained.
*/

#[derive(Default, Clone,Serialize,Deserialize)]
pub enum GigStatus {
    #[default]
    TODO,
    DONE,
    CUSTOM(String),
}

/// the Gig struct represents one ToDo item/ one thing that needs to be done. it will have a name description, duration (stored as i32 seconds), due date, and a list of attributes, or strings which exit only to add extra functionality
/// in addition to this there  exists a field named status which is either TODO, DONE or a custom user defined state

#[derive(Default,Clone,Serialize,Deserialize)]
pub struct Gig {
    pub name: String,
    pub description: String,
    pub status: GigStatus,
    pub duration: i32,
    pub due_date: i64,
    /// unix timestamp
    pub attributes: Vec<String>,
}

// a list of Gigs, indexed as a hashmap by their name
// the list has an optional name.
#[derive(Default,Serialize,Deserialize)]
pub struct GigList {
    list: HashMap<String, Gig>,
    name: Option<String>,
}

// defines how the GigList sorts Items, all self explanatory other than Attributes, basically, each gig is ordered by whetter they have certain atriums, the vector defines the order of the desired attributes
#[derive(Default)]
pub enum GigOrder {
    #[default]
    DUEDATE,
    ALPHANUMERIC,
    DURATION,
    ATTRIBUTES(Vec<String>),
    STATUS,
}

pub struct GigSelectionScope {
    order: Option<GigOrder>,
    filter: Option<Gig>,
    need_a: Option<Vec<String>>,
    exclude_a: Option<Vec<String>>,
    need_one_of_each_a: Option<Vec<Vec<String>>>,
}

impl GigSelectionScope {
    pub fn new_empty() -> Self {
        GigSelectionScope {
            order: None,
            filter: None,
            need_a: None,
            exclude_a: None,
            need_one_of_each_a: None,
        }
    }
}

// a gig environment by default holds one gig list and nothing else, however it can hold a array of gig environment extras, which will be applied and used accordingly.
// it handles saving and loading
#[derive(Serialize,Deserialize)]
pub struct GigEnvironment {
    pub default_list: GigList,
    save_path: String,
    config_dir: Option<String>,
}

impl Gig {
    pub fn new_empty() -> Self {
        Gig::default()
    }
    pub fn new(
        name: Option<String>,
        description: Option<String>,
        status: Option<GigStatus>,
        duration: Option<i32>,
        due_date: Option<i64>,
        attributes: Option<Vec<String>>,
    ) -> Self {
        Gig {
            name: name.unwrap_or_default(),
            description: description.unwrap_or_default(),
            status: status.unwrap_or_default(),
            duration: duration.unwrap_or_default(),
            due_date: due_date.unwrap_or_default(),
            attributes: attributes.unwrap_or_default(),
        }
    }
    pub fn change_status(&mut self, new_status: GigStatus) {
        self.status = new_status
    }
    pub fn minimal_gig_from_string(mut string: String) -> Self {
        //- [ ]
        let status_symbol = string.clone().chars().nth(3).unwrap_or(' ');
        let s: GigStatus = match status_symbol {
            'X' => GigStatus::DONE,
            ' ' => GigStatus::TODO,
            S => GigStatus::CUSTOM(S.to_string()),
        };
        let ling = string.split_off(5);

        Gig {
            name: ling,
            description: "".to_string(),
            status: s,
            duration: 0,
            due_date: 0,
            attributes: vec![],
        }
    }
    pub fn compare_with_filter_gig(
        &self,
        filter: Option<Gig>,
        need_a: Option<Vec<String>>,
        exclude_a: Option<Vec<String>>,
        need_one_of_each_a: Option<Vec<Vec<String>>>,
    ) -> bool {
        let mut definitive: bool = true;
        match filter {
            Some(filter) => {
                definitive = self.name.contains(&filter.name)
                    && self.description.contains(&filter.description);
                if filter.due_date != 0 {
                    definitive = definitive && self.due_date == filter.due_date;
                }
                if filter.duration != 0 {
                    definitive = definitive && self.duration == filter.duration;
                }
                definitive = definitive
                    && match &filter.status {
                        GigStatus::TODO => matches!(self.status, GigStatus::TODO),
                        GigStatus::DONE => matches!(self.status, GigStatus::DONE),
                        GigStatus::CUSTOM(k) => match &self.status {
                            GigStatus::CUSTOM(s) => k == s,
                            _ => false,
                        },
                    };
            }
            None => {}
        }
        let new_a = self.attributes.clone();
        definitive = definitive
            && match need_a {
                Some(at) => {
                    let mut y = true;
                    for a in at {
                        if !new_a.contains(&a) {
                            y = false;
                            break;
                        }
                    }
                    y
                }
                None => true,
            };
        definitive = definitive
            && match exclude_a {
                Some(at) => {
                    let mut y = true;
                    for a in at {
                        if new_a.contains(&a) {
                            y = false;
                            break;
                        }
                    }
                    y
                }
                None => true,
            };
        definitive = definitive
            && match need_one_of_each_a {
                Some(at) => {
                    let mut y = true;
                    for a in at {
                        y = false;
                        for s in a {
                            if new_a.contains(&s) {
                                y = true;
                                break;
                            }
                        }
                    }
                    y
                }

                None => true,
            };

        definitive
    }
}

impl GigList {
    pub fn new_empty(name: Option<String>) -> Self {
        Self {
            list: HashMap::new(),
            name: name,
        }
    }
    pub fn get_gigs(&self, scope: GigSelectionScope) -> Vec<&Gig> {
        // clones the gig-list because I do NOT want to deal with that shit
        let mut vec: Vec<&Gig> = self.list.values().collect();
        vec = vec
            .into_iter()
            .filter(|&x| {
                x.compare_with_filter_gig(
                    scope.filter.clone(),
                    scope.need_a.clone(),
                    scope.exclude_a.clone(),
                    scope.need_one_of_each_a.clone(),
                )
            })
            .collect();
        match scope.order {
            Some(O) => todo!(),
            None => {}
        }
        vec
    }
    pub fn add_gig(&mut self, g: Gig) {
        self.list.insert(g.name.clone(), g);
    }
}

impl GigEnvironment {
    pub fn new(save_path: String, config_dir: Option<String>) -> Self { // makes a new list and opens it
        let list = GigList::new_empty(Some("TODO".to_string()));
        Self { default_list: list, save_path: save_path, config_dir: config_dir }

        
    }
    pub fn new_save(save_path: String, config_dir: Option<String>) -> Self { // makes a new list and opens it
        let list = GigList::new_empty(Some("TODO".to_string()));
        let mut d = Self { default_list: list, save_path: save_path, config_dir: config_dir };
        d.save();
        return d;

        
    }
    pub fn save(&mut self) -> std::io::Result<()>{
        let file_name = self.save_path.clone() +"/content.json";
        let mut file = File::create(file_name )?;
        let serialized = serde_json::to_string(&self).unwrap();
        println!("serialized = {}", serialized);
        file.write_all(serialized.as_bytes())?;
        Ok(())



    }
    pub fn load(save_path: String) -> Result<Self,Error>{
        let file_name = save_path +"/content.json";
        let message: String = fs::read_to_string(file_name)?;
        let serialized:Self = serde_json::from_str(&message)?;
        Ok(serialized)

    }

}
