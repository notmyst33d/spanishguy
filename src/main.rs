use std::env;
use std::fs::write;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;
use reqwest::blocking::Client;
use regex::Regex;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let text = match args.get(1) {
        Some(value) => value,
        None => {
            println!("You need to specify the text for TTS");
            return;
        }
    };

    let mut data: Vec<u8> = vec![];
    let mut params: HashMap<&str, &str> = HashMap::new();
    let client = Client::new();

    params.insert("text", text);
    params.insert("chr", "Jason");
    params.insert("speed", "0");
    params.insert("voice", "");
    params.insert("dir", "es");
    params.insert("B", "1");

    let mut response = client
        .post("https://imtranslator.net/translate-and-speak/sockets/tts.asp?FA=0&dir=es&speed=0&B=1&ID=&chr=Jason&vc=Jorge")
        .form(&params)
        .header("Referer", "e") // Dont touch or shit will break
        .send()
        .unwrap();

    response.read_to_end(&mut data).unwrap();
    let string_data = String::from_utf8(data.clone()).unwrap();

    let re = Regex::new(r"(loader-wav\.asp\?)(.*)(cs_[0-9_]*_)(.*)(ID=)([0-9]*)(.*)(servercode=)([0-9]*)").unwrap();
    let caps = re.captures(&string_data).unwrap();
    let name = &caps[3];
    let id = &caps[6];
    let param = &caps[9];

    let mut wav_response = client
        .get(format!("https://cs1.imtranslator.net/SL/Free_Projects/{}/{}1.wav?param1={}", id, name, param))
        .send()
        .unwrap();

    data.clear();
    wav_response.read_to_end(&mut data).unwrap();

    let mut collision_counter = 0;
    let mut path_string = format!("audio{}.wav", collision_counter);
    let mut path = Path::new(&path_string);
    while Path::new(&path).exists() {
        collision_counter += 1;
        path_string = format!("audio{}.wav", collision_counter);
        path = Path::new(&path_string);
    }

    write(path, data).unwrap();

    println!("Audio written to {}", path.display());
}
