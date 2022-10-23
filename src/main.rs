use std::collections::HashMap;
use std::env;
// use std::io::Read;
// use std::str::FromStr;

fn main() {
    let action = env::args().nth(1).expect("Please specify an action");
    let item = env::args().nth(2).expect("Please specify an item");

    println!("{:?} {:?}", action, item);

    let mut todo = Todo::new().expect("Initializationof db failed");
    // let mut todo = Todo {
    //     map: HashMap::new(),
    // };

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(error) => println!("An error occurred: {}", error),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in de list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(error) => println!("An  error occurred: {}", error),
            },
        }
    }
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }

    fn new() -> Result<Todo, std::io::Error> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(file) {
            Ok(map) => Ok(Todo { map }),
            Err(error) if error.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(error) => panic!("An error occurred: {}", error),
        }
    }

    // fn new() -> Result<Todo, std::io::Error> {
    //     let mut file = std::fs::OpenOptions::new()
    //         .write(true)
    //         .create(true)
    //         .read(true)
    //         .open("db.txt")?;

    //     let mut content = String::new();
    //     file.read_to_string(&mut content)?;

    //     let mut map = HashMap::new();

    //     for entries in content.lines() {
    //         let mut values = entries.split('\t');
    //         let key = values.next().expect("No Key");
    //         let value = values.next().expect("No Value");

    //         map.insert(String::from(key), bool::from_str(value).unwrap());
    //     }

    //     return Ok(Todo { map });

    //     // let map: HashMap<String, bool> = content
    //     //     .lines()
    //     //     .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
    //     //     .map(|v| (v[0], v[1]))
    //     //     .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
    //     //     .collect();
    //     // Ok(Todo { map })
    // }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;

        serde_json::to_writer_pretty(file, &self.map)?;

        Ok(())
    }

    // fn save(self) -> Result<(), std::io::Error> {
    //     let mut content = String::new();

    //     for (key, value) in self.map {
    //         let record = format!("{}\t{}\n", key, value);
    //         content.push_str(&record)
    //     }

    //     std::fs::write("db.txt", content)
    // }
}
