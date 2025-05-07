use libactionkv::ActionKV;

#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    akv_mem.exe FILE get KEY
    akv_mem.exe FILE delete KEY
    akv_mem.exe FILE insert KEY VALUE
    akv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    akv_mem FILE get KEY
    akv_mem FILE delete KEY
    akv_mem FILE insert KEY VALUE
    akv_mem FILE update KEY VALUE
";

fn main() {
   let args: Vec<String> = std::env::args().collect();
   let file_name = args.get(1).expect(&USAGE);
   let action = args.get(2).expect(&USAGE).as_ref();
   let key = args.get(3).expect(&USAGE).as_ref();
   let opt_value = args.get(4);
   let path = std::path::Path::new(&file_name);
   let mut kv_action = ActionKV::open(path).expect("unable to open file");
   kv_action.load().expect("unable to load data");

   match action {
      "get" => {
         match kv_action.get(key).unwrap() {
            None => eprintln!("{:?} not found", key),
            // needs to use Debug as [u8] is arbitrary bytes
            Some(value) => println!("{:?}", value),
         }
      }
      "delete" => kv_action.delete(key).unwrap(),
      "insert" => {
         // an update that could be added for compatibility with Rust's HashMap
         // would be for insert to return the old
         // value, if it exists
         let value = opt_value.expect(&USAGE).as_ref();
         kv_action.insert(key, value).unwrap()
      }
      "update" => {
         let value = opt_value.expect(&USAGE).as_ref();
         kv_action.update(key, value).unwrap()
      }
      _ => eprintln!("{}", &USAGE),
   }
}
