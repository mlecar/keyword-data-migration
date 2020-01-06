extern crate config;

use reqwest::StatusCode;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("settings")).unwrap();

    let keyword_url:String = settings.get::<String>("keywordurl").unwrap();

    println!("Settings {:?}", &keyword_url);

    let resp = reqwest::blocking::get(&keyword_url)?;

    match resp.status() {
        StatusCode::OK => println!("{:#?}", resp.text()?),
        s => {
            println!("Received response status: {:?}", s);
        },
    };

    Ok(())
}