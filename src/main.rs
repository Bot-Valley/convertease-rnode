use rocket::fs::TempFile;
use converter_buddy::io::ConvertibleFile;
use converter_buddy::format::Format;
#[macro_use] extern crate rocket;
use rocket::fs::NamedFile;
use rocket::response::status;
use filename::file_name;
use std::fs;
use either::*;

#[get("/")]
fn index() -> &'static str {
    "NO FUCK OFF"
}
#[post("/<filename>/<format>", data = "<file>")]
async fn upload(mut file: TempFile<'_>, filename: String, format: String) -> Either<NamedFile,String> {
    file.persist_to(&filename).await;
    let target_format;
    if format == "jpeg" {
        target_format = Format::Jpeg;
    } else if format == "png" {
        target_format = Format::Png;
    } else if format == "webp" {
        target_format = Format::WebP;
    } else if format == "tiff" {
        target_format = Format::Tiff;
    } else if format == "bmp" {
        target_format = Format::Bmp;
    } else if format == "pdf" {
        target_format = Format::Pdf;
    } else if format == "svg" {
        target_format = Format::Svg;
    } else {
        target_format = Format::Png;
    }
    let file = ConvertibleFile::new(&filename);
    let format = file.format().expect("No format found");
    // println!("Conversion successful for {:?}", file_name(&a).unwrap())
    let mut fname = String::new();
    let mut failed = false;
    let mut error = false;
    match file.convert(target_format) {
        Ok(a) => fname = file_name(&a).unwrap().display().to_string(),
        Err(e) => {
            failed = true;
            error = true;
        },
    }
    if error {
        return Right("Error".to_string());
    }
    println!("Converted to {}", fname);
    let nfr = NamedFile::open(&fname).await.unwrap();
    fs::remove_file(&fname);
    fs::remove_file(&filename);
    Left(nfr)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).mount("/upload", routes![upload])
}
