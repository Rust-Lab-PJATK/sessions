mod users;

use std::io;
use crate::users::user::User;

fn main() {
    let us = User::default();
    println!("Default user: {}", us);
    
    let mut users: Vec<User> = vec![
        User::new("jeden_dwa".to_string(), "XD".to_string()),
        User::new("jeden_trzy".to_string(), "xd".to_string())
    ];

    for user in users.iter().clone() {
        println!("Login: {}, Password: {}", user.login, user.password);
    }

    let mut login = "".to_string();
    io::stdin()
        .read_line(&mut login)
        .unwrap();

    let mut password = "".to_string();
    io::stdin()
        .read_line(&mut password)
        .unwrap();

    let user = users.iter()
        .find(|u| u.login == login.trim() && u.password == password.trim());

    match user {
        None => println!("Nie udało się zalogować!"),
        Some(u) => println!("{}", u)
    }
}
