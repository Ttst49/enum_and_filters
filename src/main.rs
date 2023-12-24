use crate::UsPieces::{Penny, Quarter};

fn main(){
    let info = launch_app_with_match();
}


enum Message{
    Exit,
    Move { x:i64, y: i64 },
    Write(String),
    ChangeColor(i64,i64,i64),
}

impl Message {
    fn call(&self){

    }
}


enum IpAddressTypes {
    V4,
    V6,
}

struct AdressIp{
    address_ip_type: IpAddressTypes,
    address: String
}

pub fn launch_app(){

    let four = IpAddressTypes::V4;
    let six = IpAddressTypes::V6;
    router(four);
    router(six);

    let local = AdressIp {
        address_ip_type: IpAddressTypes::V4,
        address: String::from("127.0.0.1"),
    };

    let rebouclage = AdressIp {
        address_ip_type: IpAddressTypes::V6,
        address: String::from("::1"),
    };

    let mess = Message::Write(String::from("I'm writing to u"));
    mess.call();


    let a_number = Some(56);
    let string = Some("a string");

    let no_number: Option<i64> = None;

}

pub fn router(ip_type: IpAddressTypes){
    //do nothing
}

//start the enum and match part

#[derive(Debug)]
enum UsState{
    Alaska,
    Alabama,
    NewYork,
}


enum UsPieces{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn french_cents_value(piece: UsPieces)->u64{
    match piece {
        UsPieces::Penny =>{
            println!("Un centime porte bonheur");
            1
        },
        UsPieces::Nickel=>5,
        UsPieces::Dime=>10,
        UsPieces::Quarter(state) =>{
            println!("Ce quarter vient de l'etat de {:?}",state);
            25
        }
    }
}

pub fn launch_app_with_match() -> u64 {
    french_cents_value(Penny);
    french_cents_value(Quarter(UsState::NewYork))
}

pub fn plus_one(x:Option<i64>)->Option<i64>{

    match x {
        None=>None,
        Some(i)=>Some(i+1)
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

}


pub fn throw_of_the_dices(){
    let jete_de_de = 9;
    match jete_de_de {
        3 => add_hat(),
        7 => remove_hat(),
        other => move_player(other),
    }

    fn add_hat() {}
    fn remove_hat() {}
    fn move_player(nombre_cases: u8) {}
}