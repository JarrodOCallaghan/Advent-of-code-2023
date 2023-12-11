pub fn solve(data: &Vec<String>){
    let mut pile_sum: i32 = 0;
    for line in data{
        // need to split the items
        let line = line.replace("Card ", "").replace("  ", " ").replace(":", "|").replace(" | ", "|").replace("| ", "|");
        let game = line.split("|").collect::<Vec<_>>();
        println!("{line}");
        let mut matching: i32 = 0;

        // index 0 = id
        let id = game[0];
        // index 1 is winning numbers
        let winning_numbers = game[1].split_whitespace().collect::<Vec<_>>();

        // index 2 is selected numbers
        let selected_numbers = game[2].split_whitespace().collect::<Vec<_>>();

        for number in selected_numbers{
            if winning_numbers.contains(&number){
                matching += 1;
                // println!("{number}");
            }
        }

        let mut sum: i32 = 0;
        if matching > 0 {
            for i in 0..matching {
                if sum == 0 {
                    sum = 1;
                } else {
                    sum += sum;    
                }
                
            }
        }
        pile_sum += sum;
        println!("Game {id}: matching {matching}");
        println!("{sum}");
    }
    println!("Pile sum: {pile_sum}");
}