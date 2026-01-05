mod kitten;
mod utils;

type PT = kitten::PointTypes;

fn main() {
    let mut amber = kitten::Kitten::new("Amber", 19, 69420.0);

    amber.new_relation("key");
    amber.new_relation("dark");
    amber.new_relation("ephy");
    amber.new_relation("greasygirl");
    amber.new_relation("kota");

    amber.set_points  ("key", PT::HATE, 0.69);
    amber.set_points  ("dark", PT::HATE, 0.0);
    amber.set_points  ("ephy", PT::HATE, utils::MAX_FLOAT - 3.0);
    amber.set_points  ("greasygirl", PT::HATE, 0.1);
    amber.set_points  ("kota", PT::HATE, 1.0);

    let names = amber.names.clone();

    for name in names.iter() {
        println!("{} points for: {}", amber.name, name);
        println!("  - hate: {}",    amber.get_points(name, PT::HATE));
        println!("  - silly: {}",   amber.get_points(name, PT::SILLY));
        println!("  - friend: {}",  amber.get_points(name, PT::FRIEND));
        println!("  - love: {}",    amber.get_points(name, PT::LOVE));
    }
}