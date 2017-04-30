use std::io;

fn main() {
    //Initialization
    println!("Minimal version of 4-wins");
    let mut ground = [[0; 7]; 7];
    let mut player = 1;
    let mut row: usize;
    let mut lane: usize;
    loop{
    //Build-up
        p_ground(ground);
	println!("Player {}, please make your move:", player);
        println!();
    //Action-part
        row = in_row();
	lane = in_lane();
        change(&mut ground, row, lane, player);
	player = if player == 1 {2} else {1};
    }
}

//Iterate and print the whole array row by row
fn p_ground(ground: [[i32; 7]; 7]){

    for x in 0..6 {
        println!("{:?}", ground[x]);
    }
}

fn in_row()-> usize{
//    return 2;
loop{
println!("Please input the row:");
    let mut row = String::new(); //mutable guess and :: associates new of type string
    io::stdin().read_line(&mut row)
	.expect("Failed to read line.");
    let row: usize = match row.trim().parse(){
	Ok(num) => num,
	Err(_) => continue,
    };
    let row = if row <= 7 {row} else {continue;};
    return row-1;
}
}

fn in_lane()-> usize{
//    return 2;
loop{
println!("Please input the lane:");
    let mut lane = String::new(); //mutable guess and :: associates new of type string
    io::stdin().read_line(&mut lane)
        .expect("Failed to read line.");
    let lane: usize = match lane.trim().parse(){
	Ok(num) => num,
	Err(_) => continue,
    };
    let lane = if lane <= 7 {lane} else {continue;};
    return lane-1;
}
}

fn change(ground: &mut [[i32; 7]; 7], row: usize, lane: usize, player: i32){
    ground[row][lane] = player;
}


