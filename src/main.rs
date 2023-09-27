use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::io;

struct Trie {
    is_end_of_word: bool,
    children: Vec<Option<Box<Trie>>>,
}

#[derive(Serialize, Deserialize)]
struct Stats {
    total: u32,
    won: u32,
    won_per: f32,
    best_try: u32,
    current_streek: u32,
    best_streek: u32,
    try_1: u32,
    try_2: u32,
    try_3: u32,
    try_4: u32,
    try_5: u32,
    try_6: u32,
}

impl Trie {
    fn new() -> Self {
        Trie {
            is_end_of_word: false,
            children: (0..256).map(|_| None).collect(),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = self;
        for byte in word.bytes() {
            let child = &mut node.children[usize::from(byte)];
            if child.is_none() {
                *child = Some(Box::new(Trie::new()));
            }
            node = child.as_mut().unwrap();
        }
        node.is_end_of_word = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut node = self;
        for byte in word.bytes() {
            match node.children[usize::from(byte)].as_ref() {
                Some(child) => node = child,
                None => return false,
            }
        }
        node.is_end_of_word
    }

    fn traverse_and_collect(&self, prefix: String, words: &mut Vec<String>) {
        if self.is_end_of_word {
            words.push(prefix.clone());
        }
        for (i, child) in self.children.iter().enumerate() {
            if let Some(node) = child {
                let mut new_prefix = prefix.clone();
                new_prefix.push(i as u8 as char);
                node.traverse_and_collect(new_prefix, words);
            }
        }
    }

    fn get_all_words(&self) -> Vec<String> {
        let mut words = Vec::new();
        self.traverse_and_collect(String::new(), &mut words);
        return words;
    }
    /*
        fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
            let words = self.get_all_words();
            let serialized_words = serde_json::to_string(&words)?;
            std::fs::write(filename, serialized_words)?;
            return Ok(());
        }
    */
    fn load_from_file(&mut self, filename: &str) -> std::io::Result<()> {
        let data = std::fs::read_to_string(filename)?;
        let words: Vec<String> = serde_json::from_str(&data)?;
        for word in words {
            self.insert(&word.to_lowercase());
        }
        return Ok(());
    }
    /*
    fn insert_and_save(&mut self, word: &str, filename: &str) -> std::io::Result<()> {
        if self.search(word) {
            return Ok(());
        }

        self.insert(word);
        //println!("saved new word");
        return self.save_to_file(filename);
    }
    */
}

fn ler_inteiros() -> usize {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("ERRO NA LEITURA DO VALOR");

    let trimmed = input_text.trim();
    match trimmed.parse::<usize>() {
        Ok(i) => return i,
        Err(..) => (println!(
            "Isto não é um numero inteiro: {}\nTente de novo: ",
            trimmed
        ),),
    };
    return ler_inteiros();
}

fn ler_string(lookup: &Trie) -> String {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("ERRO NA LEITURA DO VALOR");

    let trimmed = input_text.trim();
    match trimmed.parse::<String>() {
        Ok(i) => {
            if i.chars().count() == 5 {
                if lookup.search(&i) {
                    return i;
                } else {
                    println!("A palavra não está no dicionário\nTente de novo: ");
                    return ler_string(lookup);
                }
            } else {
                println!("A palavra deve ter 5 caracteres");
                return ler_string(lookup);
            }
        }
        Err(..) => (println!("Isto não é string: {}\nTente de novo: ", trimmed),),
    };
    return ler_string(lookup);
}

fn validar_palavra(key: String, word: Vec<char>, alphabet: &mut Vec<char>) -> Vec<Vec<char>> {
    let key_vec: Vec<char> = key.chars().collect();
    let mut tmp = 0;
    let vec_copy = key_vec.clone();

    let mut validos: Vec<char> = vec![];
    let mut fora_pos: Vec<char> = vec![];
    let mut invalidos: Vec<char> = vec![];

    for letra_key in key_vec {
        if word[tmp] == letra_key {
            println!("letra valida {}", word[tmp]);
            validos.push(word[tmp]);
            alphabet.retain(|&c| c != word[tmp]);
            tmp += 1;
        } else if vec_copy.contains(&word[tmp]) {
            if word[tmp] != letra_key {
                println!("letra fora de posição {}", word[tmp]);
                alphabet.retain(|&c| c != word[tmp]);
                fora_pos.push(word[tmp]);
            }
            tmp += 1;
        } else {
            println!("letra invalida {}", word[tmp]);
            alphabet.retain(|&c| c != word[tmp]);
            invalidos.push(word[tmp]);
            tmp += 1;
        }
    }
    return vec![validos, fora_pos, invalidos, alphabet.to_vec()];
}

fn jogar(dicionario: &Vec<String>, lookup: &Trie, stats_s: &mut Stats) -> bool {
    let mut validos: Vec<char> = vec![];
    let mut fora_pos: Vec<char> = vec![];
    let mut invalidos: Vec<char> = vec![];
    let key = dicionario[rand::thread_rng().gen_range(0..dicionario.len())].clone();
    let mut alphabet: Vec<char> = ('a'..='z').collect();
    let mut vitoria = false;
    let mut tentativa = 6;

    //Remover na versão final
    println!("Palavra: {}", key);
    for _ in 0..6 {
        println!("Inserir palavra:");
        let palavra = ler_string(lookup);
        let chars: Vec<char> = palavra.chars().collect();
        println!("Palavra inserida: {}", palavra);
        println!("{:?}", chars);
        let res = validar_palavra(key.clone(), chars, &mut alphabet.clone());
        validos.extend(res[0].clone());
        fora_pos.extend(res[1].clone());
        invalidos.extend(res[2].clone());
        validos.dedup();
        fora_pos.dedup();
        invalidos.dedup();
        alphabet = res[3].clone();

        println!("Letras na posição correta: {:?}", validos);
        println!("Letras em posição incorreta: {:?}", fora_pos);
        println!("Letras incorreta: {:?}", invalidos);
        println!("Letras restantes: {:?}", alphabet);
        fora_pos.clear();
        if res[0].clone().len() == 5 {
            println!("Acertou na tentativa: {}", 6 - tentativa + 1);

            if 6 - tentativa + 1 > stats_s.best_try {
                stats_s.best_try = 6 - tentativa + 1
            }

            let tentativa_index: u32 = 6 - tentativa + 6;

            if tentativa_index == 6 {
                stats_s.try_1 += 1;
            } else if tentativa_index == 7 {
                stats_s.try_2 += 1;
            } else if tentativa_index == 8 {
                stats_s.try_3 += 1;
            } else if tentativa_index == 9 {
                stats_s.try_4 += 1;
            } else if tentativa_index == 10 {
                stats_s.try_5 += 1;
            } else if tentativa_index == 11 {
                stats_s.try_6 += 1;
            }

            vitoria = true;
            return vitoria;
        }
        tentativa -= 1;
        println!("tentativas restantes: {}", tentativa)
    }
    println!("Palavra: {}", key);
    println!("vitoria {}", vitoria);
    println!("O jogo acabou");
    return vitoria;
}

fn save_stats_to_file_s(stats: &Stats, filename: &str) -> std::io::Result<()> {
    let serialized_stats = serde_json::to_string(stats)?;
    fs::write(filename, serialized_stats)?;
    Ok(())
}

fn import_stats_from_file_s(filename: &str) -> std::io::Result<Stats> {
    let data = fs::read_to_string(filename)?;
    let stats: Stats = serde_json::from_str(&data)?;
    Ok(stats)
}

fn main() {
    println!("A Carregar o dicionario(EN)...");
    let mut loaded_dic_en = Trie::new();

    if let Err(e) = loaded_dic_en.load_from_file("words5.txt") {
        println!("Failed to load from file: {}", e);
    }

    let todas_en = loaded_dic_en.get_all_words();

    println!("{}", todas_en.len());

    println!("A Carregar o dicionario(PT)...");
    let mut loaded_dic_pt = Trie::new();

    if let Err(e) = loaded_dic_pt.load_from_file("palavras5.txt") {
        println!("Failed to load from file: {}", e);
    }

    let todas_pt = loaded_dic_pt.get_all_words();

    println!("{}", todas_pt.len());

    let mut stats_s = import_stats_from_file_s("stats-s.txt").unwrap();

    let mut opt: usize = usize::MAX;
    let mut streek = stats_s.current_streek;
    while opt != 0 {
        stats_s.current_streek = streek;
        if stats_s.current_streek > stats_s.best_streek {
            stats_s.best_streek = streek;
        }
        if stats_s.total != 0 {
            stats_s.won_per = (stats_s.won as f32 / stats_s.total as f32) * 100.0;
        }

        if let Err(e) = save_stats_to_file_s(&stats_s, "stats-s.txt") {
            println!("Erro a guardar as estatisticas: {}", e);
        }
        println!("1. Jogar (EN)");
        println!("2. Jogar (PT-BR)");
        println!("3. Estatisticas");
        println!("4. Limpar Estatisticas");
        println!("0. Sair");
        println!("Escolha uma opção: ");
        opt = ler_inteiros();
        match opt {
            1 => {
                if jogar(&todas_en, &loaded_dic_en, &mut stats_s) {
                    streek += 1;
                    stats_s.total += 1;
                    stats_s.won += 1;
                } else {
                    streek = 0;
                    stats_s.total += 1;
                }
            }
            2 => {
                if jogar(&todas_pt, &loaded_dic_pt, &mut stats_s) {
                    streek += 1;
                    stats_s.total += 1;
                    stats_s.won += 1;
                } else {
                    streek = 0;
                    stats_s.total += 1;
                }
            }
            3 => {
                println!(
                    "Jogos: {} , Vitorias: {} , %Vitorias: {}%",
                    stats_s.total, stats_s.won, stats_s.won_per
                );
                println!(
                    "Melhor Tentativa: {} , Sequência Atual: {} , Melhor Sequência: {}",
                    stats_s.best_try, stats_s.current_streek, stats_s.best_streek
                );
                println!(
                    "#1 {}% ({})",
                    (stats_s.try_1 as f32 / stats_s.won as f32) * 100.0,
                    stats_s.try_1
                );
                println!(
                    "#2 {}% ({})",
                    (stats_s.try_2 as f32 / stats_s.won as f32) * 100.0,
                    stats_s.try_2
                );
                println!(
                    "#3 {}% ({})",
                    (stats_s.try_3 as f32 / stats_s.won as f32) * 100.0,
                    stats_s.try_3
                );
                println!(
                    "#4 {}% ({})",
                    (stats_s.try_4 as f32 / stats_s.won as f32) * 100.0,
                    stats_s.try_4
                );
                println!(
                    "#5 {}% ({})",
                    (stats_s.try_5 as f32 / stats_s.won as f32) * 100.0,
                    stats_s.try_5
                );
                println!(
                    "#6 {}% ({})",
                    (stats_s.try_6 as f32 / stats_s.won as f32) * 100.0,
                    stats_s.try_6
                );
            }
            4 => {
                stats_s = Stats {
                    total: 0,
                    won: 0,
                    won_per: 0.0,
                    best_try: 0,
                    current_streek: 0,
                    best_streek: 0,
                    try_1: 0,
                    try_2: 0,
                    try_3: 0,
                    try_4: 0,
                    try_5: 0,
                    try_6: 0,
                };
                streek = 0;
            }
            0 => println!(""),
            _ => println!("Opção inválida"),
        }
    }
}
