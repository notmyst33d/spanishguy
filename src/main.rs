use regex::Regex;
use std::env;
use std::fs::write;
use std::io::Read;
use std::path::Path;
use ureq::AgentBuilder;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let text = match args.get(1) {
        Some(value) => value,
        None => {
            println!("You need to specify the text for TTS");
            return;
        }
    };

    let client = AgentBuilder::new().build();
    let response = client
        .post("http://imtranslator.net/translate-and-speak/sockets/tts.asp?FA=0&dir=es&speed=0&B=1&ID=&chr=Jason&vc=Jorge")
        .set("Referer", "e") // Dont touch or shit will break
        .send_form(&[
            ("text", text),
            ("chr", "Jason"),
            ("speed", "0"),
            ("voice", ""),
            ("dir", "es"),
            ("B", "1"),
        ]).unwrap()
        .into_string().unwrap();

    let re = Regex::new(
        r"(loader-wav\.asp\?)(.*)(cs_[0-9_]*_)(.*)(ID=)([0-9]*)(.*)(servercode=)([0-9]*)",
    )
    .unwrap();
    let caps = re.captures(&response).unwrap();
    let name = &caps[3];
    let id = &caps[6];
    let param = &caps[9];

    let mut bytes = vec![];
    client
        .get(&format!(
            "http://cs1.imtranslator.net/SL/Free_Projects/{}/{}1.wav?param1={}",
            id, name, param
        ))
        .call()
        .unwrap()
        .into_reader()
        .read_to_end(&mut bytes)
        .unwrap();

    let mut collision_counter = 0;
    let mut path_string = format!("audio{}.wav", collision_counter);
    let mut path = Path::new(&path_string);
    while Path::new(&path).exists() {
        collision_counter += 1;
        path_string = format!("audio{}.wav", collision_counter);
        path = Path::new(&path_string);
    }

    write(path, bytes).unwrap();

    println!("Audio written to {}", path.display());
}
