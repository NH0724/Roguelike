/*use std::string;*/

struct Player {
    name: String,
    x: i32,
    y: i32,
    hp: i32,
}
impl Player {
    fn new(name: String, x: i32, y: i32, hp: i32) -> Player {
        Player { name, x, y, hp }
    }
}
fn main() {
    let player = Player::new("Tom".to_string(), 2, 3, 1000);
    println!(
        "Player {}, positiion:({},{})and HP is {}",
        player.name, player.x, player.y, player.hp
    )
}
