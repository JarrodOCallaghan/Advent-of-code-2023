use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn solve(data: &Vec<String>){
    // Going to store the ID of all the games that meet requirements
    let mut stored_game_ids: Vec<u32> = Vec::new();
    let mut games: HashMap<u32, Game>  = HashMap::new();

    let game_requirements = Game {
        red: 12,
        green: 13,
        blue: 14,
    };

    for line in data{
        // let game = line.split([':',';',',',]);
        let data = line.split([':',';',',',]).collect::<Vec<&str>>();
        let mut id: u32 = 0;
        let mut game = Game {
            red: 0,
            green: 0,
            blue: 0,
        };

        for item in &data{
            if item.contains("Game"){
                id = item.replace("Game ", "").parse().unwrap();
            };

            if item.contains(" blue"){
                let blue: u32 = item.replace(" blue", "").replace(" ", "").parse().unwrap();
                if game.blue < blue { game.blue = blue };
            };
            if item.contains(" red"){
                let red: u32 = item.replace(" red", "").replace(" ", "").parse().unwrap();
                if game.red < red { game.red = red };
            };
            if item.contains(" green"){
                let green: u32 = item.replace(" green", "").replace(" ", "").parse().unwrap();
                if game.green < green { game.green = green };
            };
        }
        if game.red <= game_requirements.red && game.blue <= game_requirements.blue && game.green <= game_requirements.green{
            stored_game_ids.push(id);
        }
        games.insert(id, game);
    }
    let mut sum: u32 = 0;
    for (k,v) in games{
        sum += v.red * v.blue * v.green;
    };

    // println!("{:?}", stored_game_ids);

    println!("Sum of pw: {}", sum);
}