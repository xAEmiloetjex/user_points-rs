mod kitten;
mod utils;

type PT = kitten::PointTypes;

fn main() {
    let mut amber = kitten::Kitten::new("Amber", 19, 69420.0);

// Add relations
    amber.new_relation("key");
    amber.new_relation("dark");
    amber.new_relation("ephy");
    amber.new_relation("greasygirl");
    amber.new_relation("kota");

// Set HATE points
    amber.set_points  ("key", PT::HATE, 0.69);
    amber.set_points  ("dark", PT::HATE, 0.0);
    amber.set_points  ("ephy", PT::HATE, utils::MAX_FLOAT - 3.0);
    amber.set_points  ("greasygirl", PT::HATE, 0.1);
    amber.set_points  ("kota", PT::HATE, 1.0);
    
// Set SILLY points.
    amber.set_points("ephy", PT::SILLY, (utils::MAX_FLOAT / 256.0) / 256.0);

// Set FRIEND points.
    amber.set_points("key", PT::FRIEND, 69.0);
    amber.set_points("greasygirl", PT::FRIEND, 69.0);
    amber.set_points("ephy", PT::FRIEND, 69.0);

// Set CRUSH points.

// Set LOVE points.    

// Handle console output
    let names = amber.names.clone();
    let verbose = false;

    for name in names.iter() {
        let hate_p = amber.get_points(name, PT::HATE);
        let silly_p = amber.get_points(name, PT::SILLY);
        let friend_p = amber.get_points(name, PT::FRIEND);
        let crush_p = amber.get_points(name, PT::CRUSH);
        let love_p = amber.get_points(name, PT::LOVE);

        println!("{} points for: {}", amber.name, name);
              println!("  - hate: {}",    hate_p);

        if verbose == true || silly_p > 0.0 ||  silly_p < 0.0  
            { println!("  - silly: {}",   silly_p);  }
            
        if verbose == true || friend_p > 0.0 || friend_p < 0.0 
            { println!("  - friend: {}",  friend_p); }

        if verbose == true || crush_p > 0.0 ||  crush_p  < 0.0 
            { println!("  - crush: {}",   crush_p);  }

        if verbose == true || love_p > 0.0 ||   love_p < 0.0   
            { println!("  - love: {}",    love_p);   } 
    }
}