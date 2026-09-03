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
}

fn main() {
    let mut player = Player::new("Tom".to_string(), 2, 3, 80);
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
