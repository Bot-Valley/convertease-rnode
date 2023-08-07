use rocket::fs::TempFile;
use converter_buddy::io::ConvertibleFile;
use converter_buddy::format::Format;
#[macro_use] extern crate rocket;
use rocket::fs::NamedFile;
use rocket::response::status;
use filename::file_name;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[post("/<filename>", format = "image/png", data = "<file>")]
async fn upload(mut file: TempFile<'_>, filename: String) -> NamedFile {
    file.persist_to(&filename).await;
    let target_format = Format::Jpeg;
    let file = ConvertibleFile::new(&filename);
    let format = file.format().expect("No format found");
    // println!("Conversion successful for {:?}", file_name(&a).unwrap())
    let mut fname = String::new();
    match file.convert(target_format) {
        Ok(a) => fname = file_name(&a).unwrap().display().to_string(),
        Err(e) => println!("Conversion failed: {:?}", e),
    }
    println!("Converted to {}", fname);
    let nfr = NamedFile::open(&fname).await.unwrap();
    nfr
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).mount("/upload", routes![upload])
}
