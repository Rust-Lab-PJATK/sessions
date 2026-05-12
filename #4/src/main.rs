use anyhow::Context;

fn main() {
    //part_0_for_loop_desugared();
    part_1_map_filter_collect();
    //part_2_chaining_take_skip();
    //part_3_enumerate_zip();
    // part_4_option_filter_map();
    //part_5_result_collect();
    //part_6_result_propagation();
    //part_7_error_types_overview();
    //part_8_error_mapping();
    // println!("{:?}", part_9_custom_error_and_question_mark());
    //println!("{:?}",part_10_anyhow_context());
    //part_11_custom_iterator();
}


fn part_0_for_loop_desugared() {
    let numbers = vec![10, 20, 30];

    println!("--- Tradycyjna pętla for ---");
    // Klonujemy wektor tylko po to, żeby móc go użyć drugi raz poniżej.
    // W normalnym kodzie 'for n in numbers' po prostu by go skonsumowało.
    for n in numbers.clone() {
        println!("Element: {}", n);
    }

    println!("--- To samo, ale 'pod maską' (rozwinięcie pętli) ---");
    // Kompilator zamienia powyższą pętlę na coś w tym stylu:
    let mut iter = numbers.into_iter();

    // Dopóki next() zwraca Some(wartość), pętla trwa.
    // Gdy zwróci None, pętla się kończy.
    while let Some(n) = iter.next() {
        println!("Element z next(): {}", n);
    }

    // Funkcje i mechaniki:
    // into_iter (tworzy iterator z kolekcji),
    // next (pobiera kolejny element jako Option<T>),
    // while let (pattern matching, który rozpakowuje Some i przerywa na None)
}

fn part_1_map_filter_collect() {
    let scores = vec![3, 8, 12, 4, 10, 7, 1];
    let doubled_passing: Vec<i32> = scores
        .into_iter()
        .filter(|score| *score >= 5)
        .map(|score| score * 2)
        .step_by(2)
        .collect();
    println!("wynik: {:?}", doubled_passing);
    // Funkcje:
    // map (przeksztalca elementy),
    // filter (odrzuca elementy),
    // collect (tworzy kolekcje)
}

fn part_2_chaining_take_skip() {
    let values = vec![2, 4, 6, 8, 10, 12, 14, 16];
    let mid_slice: Vec<i32> = values.into_iter().skip(2).take(5).collect();
    let sum: i32 = mid_slice.iter().sum();
    println!("srodek: {:?}, suma: {}", mid_slice, sum);
    // Funkcje: skip (pomija N), take (bierze N), sum (sumuje), collect (tworzy kolekcje)
}

fn part_3_enumerate_zip() {
    let names = vec!["Ala", "Ola", "Ewa"];
    let scores = vec![10, 12, 9];
    let labeled: Vec<String> = names
        .into_iter()
        .enumerate()
        .zip(scores.into_iter())
        .map(|((index, name), score)| format!("{}: {} ({})", index + 1, name, score))
        .collect();
    println!("oceny: {:?}", labeled);
    // Funkcje: enumerate (indeksy), zip (laczy pary), map (tworzy napisy)
}

fn part_4_option_filter_map() {
    let tokens = vec!["-42", "", "7", "alpha", "19", "0"];
    let parsed_numbers: Vec<i32> = tokens
        .into_iter()
        .filter_map(|token| token.parse::<i32>().ok())
        .collect();
    let sum: i32 = parsed_numbers.iter().sum();
    println!("liczby: {:?}, suma: {}", parsed_numbers, sum);
    // Funkcje: filter_map (pomin bledy), parse (konwersja), collect (tworzy kolekcje)
}

fn part_5_result_collect() {
    let raw_inputs = vec!["8", "15", "asdads", "23"];
    let parsed: Result<Vec<i32>, std::num::ParseIntError> = raw_inputs
        .into_iter()
        .map(|input| input.parse::<i32>())
        .collect();
    println!("wynik: {:?}", parsed);
    // Funkcje: map (tworzy Result), collect (zbiera do Result<Vec<_>, _>)
}

fn part_6_result_propagation() {
    let inputs = vec!["3000", "8080"];
    let endpoints: Result<Vec<String>, std::num::ParseIntError> =
        inputs.into_iter().map(build_endpoint).collect();
    println!("endpointy: {:?}", endpoints);
    // Funkcje: ? (propaguje blad), parse (konwersja), map (tworzy napisy), collect (zbiera do Result)
}

fn build_endpoint(port_text: &str) -> Result<String, std::num::ParseIntError> {
    let port = parse_port(port_text)?;
    Ok(format!("127.0.0.1:{}", port))
}

fn parse_port(port_text: &str) -> Result<u16, std::num::ParseIntError> {
    port_text.parse::<u16>()
}

#[derive(Debug)]
enum SettingsError {
    MissingKey(String),
    InvalidNumber(String),
    InvalidName(String),
}

#[derive(Debug, Clone)]
struct Settings {
    port: u16,
    threads: usize,
}

fn part_7_error_types_overview() {
    let invalid = validate_name(Some(""));
    let kay = validate_name(Some("Ada"));
    let missing = validate_name(None);
    println!("brak: {:?}", invalid);
    println!("poprawne: {:?}", kay);
    println!("none: {:?}", missing);
    // Funkcje: ok_or (Option -> Result), map_err (zmienia blad), to_string (tekst)
}

fn validate_name(value: Option<&str>) -> Result<String, SettingsError> {
    let name = value.ok_or(SettingsError::MissingKey("name".to_string()))?;
    let trimmed = name.trim();
    if trimmed.len() < 3 {
        return Err(SettingsError::InvalidName(trimmed.to_string()));
    }
    Ok(trimmed.to_string())
}

fn part_8_error_mapping() {
    let inputs = vec!["12", "sx", "dd", "7"];
    let mapped: Result<Vec<u8>, SettingsError> = inputs
        .into_iter()
        .map(|text| {
            text.parse::<u8>()
                .map_err(|_| SettingsError::InvalidNumber(text.to_string()))
        })
        .collect();
    println!("mapowanie: {:?}", mapped);
    // Funkcje: map_err (zamiana bledu), parse (konwersja), collect (zbiera do Result)
}

fn part_9_custom_error_and_question_mark() -> Result<(), SettingsError> {
    let config = vec!["port=8080", "threads=2"];
    let settings = load_settings(&config)?;
    let connection = connect_service(settings)?;
    println!("polaczenie: {}", connection);
    // Funkcje: ok_or (Option -> Result), ? (propaguje blad), map_err (zmienia blad)
    Ok(())
}

fn load_settings(lines: &[&str]) -> Result<Settings, SettingsError> {
    let port_text =
        find_value(lines, "port").ok_or(SettingsError::MissingKey("port".to_string()))?;
    let threads_text =
        find_value(lines, "threads").ok_or(SettingsError::MissingKey("threads".to_string()))?;
    let port = port_text
        .parse::<u16>()
        .map_err(|_| SettingsError::InvalidNumber(port_text.to_string()))?;
    let threads = threads_text
        .parse::<usize>()
        .map_err(|_| SettingsError::InvalidNumber(threads_text.to_string()))?;
    Ok(Settings { port, threads })
}

fn find_value<'a>(lines: &'a [&'a str], key: &str) -> Option<&'a str> {
    lines.iter().find_map(|line| {
        let (left, right) = line.split_once('=')?;
        if left.trim() == key {
            Some(right.trim())
        } else {
            None
        }
    })
}

fn connect_service(settings: Settings) -> Result<String, SettingsError> {
    if settings.port == 0 || settings.threads == 0 {
        return Err(SettingsError::InvalidNumber("port/threads".to_string()));
    }
    Ok(format!(
        "port={}, threads={}",
        settings.port, settings.threads
    ))
}

fn part_10_anyhow_context() -> anyhow::Result<()> {
    let value = read_config_value("missing.toml")?;
    println!("konfiguracja: {}", value);
    // Funkcje: anyhow::Context (dodaje kontekst), ? (propaguje blad)
    Ok(())
}

fn read_config_value(path: &str) -> anyhow::Result<String> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("brak pliku konfiguracji: {}", path))?;
    let mut lines = contents.lines();
    let value = lines
        .find(|line: &&str| line.trim_start().starts_with("name="))
        .map(|line| line.trim_start_matches("name=").trim().to_string())
        .context("brak pola name w konfiguracji")?;
    Ok(value)
}

fn part_11_custom_iterator() {
    let team = Team::new("Aurora", vec!["Mia", "Noah", "Ava", "Liam"]);
    let roster: Vec<String> = team.iter().step_by(2).collect();
    println!("zespol: {}, sklad: {:?}", team.name, roster);
    // Funkcje: iter (tworzy iterator), collect (tworzy kolekcje)
}

struct Team {
    name: String,
    members: Vec<String>,
}

impl Team {
    fn new(name: &str, members: Vec<&str>) -> Self {
        let members = members.into_iter().map(String::from).collect();
        Self {
            name: name.to_string(),
            members,
        }
    }

    fn iter(&self) -> TeamIter {
        TeamIter {
            team: self,
            index: 0,
        }
    }
}

struct TeamIter<'a> {
    team: &'a Team,
    index: usize,
}

impl<'a> Iterator for TeamIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.team.members.len() {
            return None;
        }
        let member = self.team.members[self.index].clone();
        self.index += 1;
        Some(member)
    }
}
