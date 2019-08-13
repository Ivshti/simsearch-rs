use std::fs::File;
use std::io::{self, Read as _};
use std::time::Instant;

use simsearch::SimSearch;

use serde::Deserialize;
#[derive(Deserialize)]
struct MetaPreview {
    pub id: String,
    pub name: String,
}

fn main() -> io::Result<()> {
    let mut engine = SimSearch::new();

    let mut file = File::open("./cinemeta.json")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let items: Vec<MetaPreview> = serde_json::from_str(&content).unwrap();

    for item in items {
        engine.insert(item.name.to_owned(), &item.name);
    }

    println!("Please input a query string and hit enter (e.g 'old man'):",);

    loop {
        let mut pattern = String::new();
        io::stdin()
            .read_line(&mut pattern)
            .expect("failed to read from stdin");

        let start = Instant::now();
        let res = engine.search(&pattern);
        let end = Instant::now();

        //println!("pattern: {:?}", pattern.trim());
        //println!("results: {:?}", res);
        println!("time: {:?}", end - start);
    }
}
