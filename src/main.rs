const MAP_WIDTH: i32 = 10;
const MAP_HEIGHT: i32 = 10;
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
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

    fn heal(&mut self, amount: i32) {
        self.hp = self.hp + amount;
        if self.hp >= 100 {
            self.hp = 100
        }
    }

    fn damage(&mut self, amount: i32) {
        self.hp = self.hp - amount;
        if self.hp <= 0 {
            self.hp = 0
        }
    }
    fn move_player(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => {
                if self.y > 0 {
                    self.y = self.y - 1;
                } else {
                    self.y = 0;
                }
            }
            Direction::Down => {
                if self.y < MAP_HEIGHT - 1 {
                    self.y = self.y + 1;
                } else {
                    self.y = MAP_HEIGHT - 1;
                }
            }
            Direction::Left => {
                if self.x > 0 {
                    self.x = self.x - 1;
                } else {
                    self.x = 0;
                }
            }
            Direction::Right => {
                if self.x < MAP_WIDTH - 1 {
                    self.x = self.x + 1;
                } else {
                    self.x = MAP_WIDTH - 1;
                }
            }
        }
    }
}

fn main() {
    let mut player = Player::new("Tom".to_string(), 2, 3, 80);
    player.move_player(&Direction::Down);
    player.move_player(&Direction::Left);
    player.move_player(&Direction::Left);
    println!(
        "Player {}, positiion:({},{}) and HP is {}",
        player.name, player.x, player.y, player.hp
    );
    player.heal(50);
    println!(
        "Player {}, positiion:({},{}) and HP is {}",
        player.name, player.x, player.y, player.hp
    );
    player.damage(10);
    println!(
        "Player {}, positiion:({},{}) and HP is {}",
        player.name, player.x, player.y, player.hp
    );
}
