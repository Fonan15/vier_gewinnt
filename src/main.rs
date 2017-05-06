use std::io;
//use std::io::stdout;
//use std::io::Write;

fn main() {
//Initialization
    println!("Minimal version of 4-wins");
	println!("==========================");
	println!();
    let mut ground = [[0; 7]; 7];
    let player = 1;
//Actual game
    game(&mut ground, player);
}

//Iterate and print the whole array row by row
fn p_ground(ground: &[[i32; 7]; 7]){
    for x in 0..7 {
        println!("{:?}", ground[x]);
    }
}

//The game itself after initialization
fn game(mut ground: &mut[[i32;7];7], mut player: i32){
let mut win = false;
while win == false {
//Game board
        p_ground(&ground);
        println!("Player {}, please make your move:", player);
//Input
        let (row, lane) = change(&mut ground, player);
//		println!("row: {}, lane: {}", row, lane);//Debug print
//Check for win
		win = winning(&ground, player);
//New player set
		player = if player == 1 {2} else {1};
//		stdout().flush();
    }
}

//Input for the lane which is parsed, checked and returned (only existing lanes accepted)
fn change(ground: &mut[[i32; 7]; 7], player: i32)-> (usize, usize){
//ground[1][1] = 2;//debug-set
//return;
println!("Please provide a lane:");
loop{
//stdout().flush();
//Input itself
    let mut lane = String::new(); //mutable guess and :: associates new of type string
    io::stdin().read_line(&mut lane)
        .expect("Failed to read line.");
    let lane: usize = match lane.trim().parse(){
	Ok(num) => num,
	Err(_) => continue, //If error occures go to the beginning of input
    };
//Validity check
    let lane = if lane <= 7 {lane-1} else {println!("You must provide an existing lane!\nPlease provide another lane.");
    					continue;}; //If lane is out of bounds go to beginning of input
    let mut row: usize;
    row = 6;
    while ground[row][lane] != 0 && row > 0{
    row = row -1;
    }
    if ground[row][lane] == 0 {
    ground[row][lane] = player; //If lane is legit set player to board
    return (row, lane);} else {
    println!("Lane is full.\nPlease provide another lane."); //If lane is full return to beginning of input
    continue;};
}
}

//Check if any of the given conditions leads to a win
fn winning(ground : &[[i32;7];7], player: i32)-> bool{
//		if (ground[row][lane] == player){ 
//			p_ground(&mut ground);
//			println!("Player {}, you win!", player);
//			return true;}
//			else {return false};//debug-win-check
for x in 0..7{
	for y in 0..4{
//checks horizontal win
        if(ground[x][y] != 0 &&
		ground[x][y]==ground[x][y+1] &&
		ground[x][y]==ground[x][y+2] && 
		ground[x][y]==ground[x][y+3]){
		p_ground(&ground);
		println!("Player {}, you win!", player);
        return true;}
		}
    }
for x in 0..4{
    for y in 0..7{
//checks vertical win
        if(ground[x][y] != 0 &&
		ground[x][y]==ground[x+1][y] &&
		ground[x][y]==ground[x+2][y] &&
		ground[x][y]==ground[x+3][y]){
		p_ground(&ground);
		println!("Player {}, you win!", player);
		return true;}
        }
    }
for x in 0..4{
    for y in 0..4{
//checks right diagonal
        if(ground[x][y] != 0 &&
		ground[x][y]==ground[x+1][y+1] &&
		ground[x][y]==ground[x+2][y+2] &&
		ground[x][y]==ground[x+3][y+3]){
		p_ground(&ground);
		println!("Player {}, you win!", player);
		return true;}
        }
    }
for x in 0..4{
    for y in 3..7{
//checks left diagonal
        if(ground[x][y] != 0 &&
		ground[x][y]==ground[x+1][y-1] &&
		ground[x][y]==ground[x+2][y-2] &&
		ground[x][y]==ground[x+3][y-3]){
		p_ground(&ground);
		println!("Player {}, you win!", player);
		return true;}
        }
    }
return false;
}
