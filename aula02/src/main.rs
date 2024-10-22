fn aula_tupla(){
    let tupla = (12, "valores", 3.14, (1,2,3));
    println!("O valor de  é {:?}", (tupla.3).1);
    let (a,b,c,d) = tupla;
    println!("O valor de a é {}", a);
    println!("O valor de b é {}", b);
    println!("O valor de c é {}", c);
    println!("O valor de d é {:?}", d);
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}
#[derive(Debug)]
enum Gender {
    Male,Female
}

#[derive(Debug)]
enum CarType {
    Fiat, Ford, Renault
}

fn enums(){
    let player:Direction = Direction::Right;
    match player {
        Direction::Up => println!("O jogador foi para cima!"),
        Direction::Down => println!("O jogador foi para baixo!"),
        Direction::Left => println!("O jogador foi para esquerda!"),
        Direction::Right => println!("O jogador foi para direita!"),
    }
    let player_male:Gender = Gender::Male;
    let player_female:Gender = Gender::Female;
    println!("{:?}", player_male);
    println!("{:?}", player_female);
}

fn nacionalidade_carro(car:CarType){
    match car {
        CarType::Fiat => println!("O carro é italiano!"),
        CarType::Ford => println!("O carro é americano!"),
        CarType::Renault => println!("O carro é frances!"),
    }
}
fn main() {
    aula_tupla();
    enums();
    nacionalidade_carro(CarType::Fiat);
    nacionalidade_carro(CarType::Ford);
    nacionalidade_carro(CarType::Renault);
}
