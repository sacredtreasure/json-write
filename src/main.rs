use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Monotype {
    title: String,
    version: String,
    ratings: String,
}

#[derive(Serialize, Deserialize)]
struct Prototype {
    creator: String,
    version_control: String,
    class: String,
    monotype: Vec<Monotype>,
}

fn main() {
    let creator: Prototype = Prototype {
        creator: String::from("Martin Scorese"),
        version_control: String::from("1.1"),
        class: String::from("A-class"),
        monotype: vec![
            Monotype {
                title: String::from("Gangs of New York"),
                version: String::from("2002"),
                ratings: String::from("4"),
            },
            Monotype {
                title: String::from("The Departed"),
                version: String::from("2006"),
                ratings: String::from("3"),
            },
            Monotype {
                title: String::from("The Wolf of Wall Street"),
                version: String::from("2013"),
                ratings: String::from("5"),
            },
            Monotype {
                title: String::from("The Irishman"),
                version: String::from("2019"),
                ratings: String::from("3.5"),
            },
        ],
    };
    let json = serde_json::to_string(&creator).unwrap();
    println!("Martin Scorese's works are: {}", json)
}
