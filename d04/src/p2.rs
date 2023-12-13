use std::collections::HashMap;

pub fn solve(data: &Vec<String>){
    let mut card_pile: HashMap<i32, i32> = Default::default();

    for line in data {
        let line = line.replace("   ", " ").replace("  ", " ").replace("Card ", "").replace(":", "|").replace(" | ", "|").replace("| ", "|");
        let game = line.split("|").collect::<Vec<_>>();
        // println!("{line}");
        // for item in &game{
        //     println!("{item}");
        // }

        let id: i32 = game[0].parse::<i32>().unwrap();
        let selected_numbers: Vec<i32> = game[1].split_whitespace().map(|i| i.parse::<i32>()).map(Result::unwrap).collect::<Vec<_>>();
        let winning_numbers: Vec<i32> = game[2].split_whitespace().map(|i| i.parse::<i32>()).map(Result::unwrap).collect::<Vec<_>>();

        // If the card doesn't exist yet, we need to create it
        if card_pile.get(&id) == None{
            card_pile.insert(id, 1);
        }

        // calc score for 1 card
        let mut score: i32 = 0;
        for number in selected_numbers{
            if winning_numbers.contains(&number){
                score += 1;
            }
        }
        // println!("{id} : {score}");

        // Work out what next set of cards need to be updated:
        let next_id: i32 = id + 1;
        let instances = if card_pile.get(&id) != None {*card_pile.get(&id).unwrap()} else {1};
        // println!("INSTANCES: {instances}");
        for n in next_id..(next_id+score){
            *card_pile.entry(n).or_insert(1) += instances;
            // print!("{n},");
        }
        // println!("\n");

        // Now we need to add this into our hashmap

    }

    let mut total: i32 = 0;
    for (k, v) in card_pile{
        println!("k: {k}, v: {v}");
        total += v;
    }
    println!("{total}");

}