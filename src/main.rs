use std::collections::HashMap;
use std::env;
use std::fs;
use std::result::Result;

fn main() {
    let mut args = env::args().skip(1);
    let func = args.next().unwrap();

    let key = match args.next() {
        Some(val) => val,
        None => "".to_owned(),
    };
    let value = match args.next() {
        Some(val) => val,
        None => "".to_owned(),
    };

    let mut db = Database::new().unwrap();

    match func.as_str() {
        "get" => db.get(&key),
        "set" => db.set(&key, &value),
        "delete" => db.delete(&key),
        "show" => db.show(),
        "clear" => db.clear(),
        "help" => {
            println!("Avaliable commands: \r\n\tget\r\n\tset\r\n\tshow\r\n\tdelete\r\n\tclear")
        }
        _ => println!("Invalid command"),
    }
}

struct Database {
    pub inner: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let content = fs::read_to_string("./kv.db")?;
        let mut inner = HashMap::<String, String>::new();

        for line in content.lines() {
            let chunks: Vec<&str> = line.split("\t").collect();

            if chunks.len() != 2 {
                todo!("Return error!")
            }

            let key = chunks[0];
            let value = chunks[1];
            inner.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database { inner })
    }

    fn set(&mut self, key: &str, value: &str) {
        let equal = match &self.inner.get(key) {
            Some(_) => true,
            None => false,
        };

        let mut insert = true;

        if equal {
            insert = confirm_overwrite(&key);
        }

        if insert {
            println!("Value inserted in DB");

            self.inner.insert(key.to_owned(), value.to_owned());
            self.update();
        }
    }

    fn get(&self, key: &str) {
        let _value = match self.inner.get(key) {
            Some(val) => println!("{}", val),
            None => return println!("Value \"{}\" not in DB", key),
        };
    }

    fn update(&self) {
        let mut new_content = String::new();

        for i in self.inner.iter() {
            new_content.push_str(format!("{}\t{}\r\n", i.0, i.1).as_str());
        }

        fs::write("./kv.db", new_content).expect("Error writing file");
    }

    fn show(&self) {
        if self.inner.len() == 0 {
            return println!("No values in DB");
        }
        println!("Values in DB:");
        println!("---");
        for n in self.inner.iter() {
            println!("{}: {}", n.0, n.1);
        }
    }

    fn delete(&mut self, key: &str) {
        if key.is_empty() {
            return println!("Please insert a key value");
        }
        let confirm = confirm_delete();

        if confirm {
            println!("Value deleted");
            self.inner.remove(key);
            self.update();
        } else {
            println!("Value not deleted");
        }
    }

    fn clear(&mut self) {
        let confirm = confirm_delete_all();

        if confirm {
            println!("DB cleared");
            self.inner = HashMap::new();
            self.update()
        } else {
            println!("DB not cleared");
        }
    }
}

fn confirm_delete_all() -> bool {
    let mut response: bool = false;
    let mut looping = true;
    while looping {
        println!("Are you sure? All data will be deleted. Cannot be undone.\r\n[Y]es \\ [N]o");

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if input.trim().to_uppercase().eq("Y") {
            response = true;
            looping = false;
        } else if input.trim().to_uppercase().eq("N") {
            response = false;
            looping = false;
        } else {
            println!("Invalid input");
        }
    }
    return response;
}

fn confirm_delete() -> bool {
    let mut response: bool = false;
    let mut looping = true;
    while looping {
        println!("Are you sure? Data will be deleted.\r\n[Y]es \\ [N]o");

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if input.trim().to_uppercase().eq("Y") {
            response = true;
            looping = false;
        } else if input.trim().to_uppercase().eq("N") {
            response = false;
            looping = false;
        } else {
            println!("Invalid input");
        }
    }
    return response;
}

fn confirm_overwrite(key: &str) -> bool {
    let mut response: bool = false;
    let mut looping = true;
    while looping {
        let mut input = String::new();
        println!(
            "Value \"{}\" already exists in DB. Overwrite?\r\n[Y]es \\ [N]o",
            key
        );
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().to_uppercase().eq("Y") {
            println!("Value overwritten");
            response = true;
            looping = false;
        } else if input.trim().to_uppercase().eq("N") {
            println!("Value not overwritten. Consider using another key");
            response = false;
            looping = false;
        } else {
            println!("Invalid input");
        }
    }
    return response;
}
