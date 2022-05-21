use std::{collections::HashMap, io};

/// Language codes:
/// en      English
/// hi      Hindi
/// es      Spanish
/// fr      French
/// ru      Russian
/// de      German
/// it      Italian
/// ko      Korean
/// pt-BR   Brazilian Portuguese
/// zh-CN   Chinese (Simplified)
/// ar      Arabic
/// tr      Turkish
const LANG_CODES: [&str; 12] = ["en", "hi", "es", "fr", "ru", "de", "it", "ko", "pt-BR", "zh-CN", "ar", "tr"];

fn send(query: &str) -> Result<String, ureq::Error> {
    Ok(ureq::get(query)
        .call()?
        .into_string()?)
}

fn main() {
    let mut cache: HashMap<&str, HashMap<String, String>> = HashMap::new();

    'outer: loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Fail");

        let mut search = Vec::new();
        let mut lang = LANG_CODES[0];
        let mut expect_lang = false;

        'arg_check: for arg in line.split(' ') {
            if arg.eq("-l") {
                expect_lang = true;
            } else if expect_lang {
                let trim = arg.trim();

                for code in LANG_CODES.iter() {
                    if code.eq(&trim) {
                        lang = code;
                        break 'arg_check;
                    }
                }

                eprintln!("{} is not a valid language code.", trim);
                continue 'outer;
            } else {
                search.push(arg);
            }
        }

        match cache.get(lang) {
            Some(lang_dict) => {
                if let Some(def) = lang_dict.get(&search.join(" ")) {
                    println!("{}", def);
                    continue;
                }
            },
            None => {
                cache.insert(lang, HashMap::new());
            }
        }

        match send(&format!("https://api.dictionaryapi.dev/api/v2/entries/{}/{}", lang, search.join(" "))) {
            Ok(r) => println!("{}", r),
            Err(e) => eprintln!("Query failed: {}", e),
        }
    }
}
