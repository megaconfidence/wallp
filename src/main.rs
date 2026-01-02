#[macro_use] extern crate rocket;
use std::fs;
use std::env;
use rocket::State;
use dotenv::dotenv;
use std::sync::Mutex;
use rocket::http::Status;
use std::process::Command;

struct App {
    index: Mutex<usize>,
    wallpaper_dir: String
}
#[get("/")]
fn index(app: &State<App>) -> Status  {
    //get next wallpaper index
    let mut index = app.index.lock().unwrap();

    //get list of images
    let mut images = fs::read_dir(&app.wallpaper_dir)
        .expect("invalid wallpaper directory")
        .filter_map(|e:Result<fs::DirEntry, _>| e.ok())
        .map(|e:fs::DirEntry| e.path().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    images.sort();

    //set wallpaper to index
    Command::new("sh")
        .arg("-c")
        .arg(format!("pcmanfm --set-wallpaper  {}", images[*index]))
        .output()
        .expect("failed to change wallpaper");

    //set next wallpaper index
    *index = if *index+1 < images.len() {*index+1} else {0};
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .manage(App { index: 0.into(), wallpaper_dir: env::var("WALLPAPER_DIR").expect("WALLPAPER_DIR env variable not set")})
        .mount("/", routes![index])
}
