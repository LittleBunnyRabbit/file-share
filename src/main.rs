#![feature(proc_macro_hygiene, decl_macro)]

use rocket_contrib::serve::StaticFiles;
use rocket::response::content::Html;
use rocket::config::Config;
use glob::glob;

static mut ALL_PATHS: String = String::new();

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Html<&'static str> {
    unsafe { Html(ALL_PATHS.as_str()) }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let port: u16 = if args.len() > 1 { args[1].parse::<u16>().unwrap() } else { 3000 };
    let mut custom_config = Config::active().unwrap();
    custom_config.set_port(port);

    for entry in glob("**/*.*").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                match path.to_str() {
                    Some(path_str) => {
                        let mut new_path_string = String::from("<a href=");
                        new_path_string.push('"');
                        new_path_string.push_str("public/");
                        new_path_string.push_str(&path_str);
                        new_path_string.push('"');
                        new_path_string.push('>');
                        new_path_string.push_str(&path_str);
                        new_path_string.push_str("</a><br />");
                        unsafe {
                            ALL_PATHS.push_str(&new_path_string);
                        }
                    }
                    None => println!("Path error")
                }
            },
            Err(e) => println!("{:?}", e),
        }
    }

    match std::env::current_dir() {
        Ok(dir) => {
            rocket::custom(custom_config)
                .mount("/", routes![index])
                .mount("/public", StaticFiles::from(dir))
                .launch();
        }   
        Err(_) => println!("Failed to read current dir")
    }
}