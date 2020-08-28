mod iciba;
use clap::{App, Arg};

const APP_NAME: &'static str = "fanyi-rs";

#[tokio::main]
async fn main() {
    let args = App::new(APP_NAME)
        .version("0.1.0")
        .author("3vilive <ba0ch3ng@foxmail.com>")
        .about("commend line tool for translate english word into chinese")
        .arg(
            Arg::with_name("input")
                .help("content to translate")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let input = args.value_of("input").unwrap();
    let translate_content = String::from(input.trim());

    match iciba::get_translate_result(&translate_content).await {
        Ok(result) => println!("{}", result),
        Err(err) => println!("{}: {}", APP_NAME, err),
    }
}
