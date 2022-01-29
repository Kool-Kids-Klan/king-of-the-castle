enum Resource {
    Coins,
    Corn,
    Hat,
    Fiddle,
    Swords,
    Flask,
}

pub struct Token {
    points: u8,
    resource: Resource,
}

impl Token {
    pub fn new(points: u8, resource: Resource) -> Token {
        Token { points, resource }
    }
}
