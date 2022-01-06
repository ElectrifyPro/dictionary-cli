use std::{collections::HashMap, io};
use reqwest::Client;
use tokio;

/** Language codes:
 * en      English
 * hi      Hindi
 * es      Spanish
 * fr      French
 * ru      Russian
 * de      German
 * it      Italian
 * ko      Korean
 * pt-BR   Brazilian Portuguese
 * zh-CN   Chinese (Simplified)
 * ar      Arabic
 * tr      Turkish
**/
const LANG_CODES: [&str; 12] = ["en", "hi", "es", "fr", "ru", "de", "it", "ko", "pt-BR", "zh-CN", "ar", "tr"];

async fn send(client: &Client, query: String) {
    match client.get(query).send().await {
        Ok(res) => {
            match res.text().await {
                Ok(t) => println!("{}", t),
                Err(e) => eprintln!("Query failed: {}", e)
            }
        },
        Err(e) => eprintln!("Query failed: {}", e)
    }
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let mut cache: HashMap<&str, HashMap<String, String>> = HashMap::new();

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Fail");

        let mut search = Vec::new();
        let mut lang = LANG_CODES[0];
        let mut expect_lang = false;

        for arg in line.split(' ') {
            if arg.eq("-l") {
                expect_lang = true;
            } else if expect_lang {
                expect_lang = false;
                let trim = arg.trim();

                for code in LANG_CODES.iter() {
                    if code.eq(&trim) {
                        lang = code;
                        break;
                    }
                }

                eprintln!("{} is not a valid language code.", lang);
                continue;
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

        send(&client, format!("https://api.dictionaryapi.dev/api/v2/entries/{}/{}", lang, search.join(" "))).await;
    }
}
