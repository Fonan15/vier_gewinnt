use std::io;

fn main() {
    //Initialization
    println!("Minimal version of 4-wins");
    let mut ground = [[0; 7]; 7];
    let mut player = 1;
    game(&mut ground, mut player);
}


fn game(mut ground: &mut[[i32;7];7], mut player: i32){
let mut win = false;
while win == false {
    //Game board
        p_ground(&mut ground);
        println!("Player {}, please make your move:", player);
    //Input
        change(&mut ground, player);
	player = if player == 1 {2} else {1};
    //Check for win
        
    }
}


//Iterate and print the whole array row by row
fn p_ground(ground: &mut[[i32; 7]; 7]){

    for x in 0..7 {
        println!("{:?}", ground[x]);
    }
}

//Input for the lane which is parsed and returned (only existing lanes accepted)
fn change(ground: &mut[[i32; 7]; 7], player: i32){
//ground[1][1] = 2;
//return;
loop{
println!("Please provide a lane:");
//Eingabemechanik
    let mut lane = String::new(); //mutable guess and :: associates new of type string
    io::stdin().read_line(&mut lane)
        .expect("Failed to read line.");
    let lane: usize = match lane.trim().parse(){
	Ok(num) => num,
	Err(_) => continue, //If error occures go to the beginning of input
    };
//Kontrollmechanik
    let lane = if lane <= 7 {lane-1} else {println!("You must provide an existing lane!");
    					continue;}; //If lane is out of bounds go to beginning of input
    let mut row: usize;
    row = 6;
    while ground[row][lane] != 0 && row > 0{
    row = row -1;
    }
    if ground[row][lane] == 0 {
    ground[row][lane] = player; //If lane is legit set player to board
    return;} else {
    println!("Lane is full."); //If lane is full return to beginning of input
    continue;};
}
}

