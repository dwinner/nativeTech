use std::fs;

fn main()
{
   println!("cargo:rerun-if-changed=build.rs"); // Перезапуск при изменении скрипта
   if std::env::var("PROFILE").unwrap() == "release"
   {
      let binary_name = "data-processor"; // Замените на имя вашего проекта
      let source = format!("target/release/{}", binary_name);
      let dest_dir = "dist";
      fs::create_dir_all(dest_dir).unwrap();
      fs::copy(&source, format!("{}/{}", dest_dir, binary_name)).unwrap();
   }
}
