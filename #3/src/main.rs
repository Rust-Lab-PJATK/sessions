use std::collections::{BTreeMap, HashMap, HashSet};

fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    vec.push(6);
    vec.pop();

    let val = vec.get(2);
    println!("wartość indeksu 2: {}", val.unwrap());
    vec.swap(0, 4);
    println!("Istnieje 3?: {}", vec.contains(&40));
    vec.remove(2);

    // Java:
    // () -> {
    // sout("test")
    // }
    // void print() { sout("test") }

    let vec2: Vec<_> = vec.iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| format!("Parzysta liczba: {}", x))
        .collect();

    let vec3 = vec.iter()
        .filter(|x| *x % 2 == 0)
        .count();

    println!("Parzystych liczb jest: {}", vec3);

    for i in vec2 {
        println!("{}", i);
    }

    let pl = vec!["jabłko", "banan"];
    let en = vec!["apple", "banana"];

    let mut dict = HashMap::<String, String>::new();

    dict.insert("jabłko".to_string(), "apple".to_string());
    dict.insert("banan".to_string(), "banana".to_string());

    let keys: Vec<_> = dict.keys().map(|k| format!("{}", k)).collect();
    println!("Klucze (iter) {}", keys.join(", "));

    dict.iter()
        .filter(|(k, v)| k.len() > 5 || v.len() > 5)
        .for_each(|(k, v)| println!("{} => {}", k, v));

    for (key, val) in dict.clone() {
        println!("{} => {}", key, val)
    }

    dict.keys().for_each(|k| println!("klucz: {}", k));
    dict.values().for_each(|v| println!("wartość: {}", v));

    for (p, e) in pl.iter().zip(en.iter()) {
        println!("{} - {}", p, e);
    }

    let mut set = HashSet::<i32>::new();
    set.insert(20);
    set.insert(30);
    set.insert(30);

    for i in set {
        println!("{}", i);
    }

    let mut btreemap = BTreeMap::new();
    btreemap.insert(3, "trzy");
    btreemap.insert(1, "jeden");
    btreemap.insert(2, "dwa");

    btreemap.iter().for_each(|(k, v)| println!("{} => {}", k, v));
}
