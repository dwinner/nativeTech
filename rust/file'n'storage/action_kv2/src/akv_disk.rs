use libactionkv::ActionKV;
use std::collections::HashMap;

#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    akv_disk.exe FILE get KEY
    akv_disk.exe FILE delete KEY
    akv_disk.exe FILE insert KEY VALUE
    akv_disk.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    akv_disk FILE get KEY
    akv_disk FILE delete KEY
    akv_disk FILE insert KEY VALUE
    akv_disk FILE update KEY VALUE
";

type ByteStr = [u8];
type ByteString = Vec<u8>;

fn store_index_on_disk(a_kv_action: &mut ActionKV, index_key: &ByteStr) {
   a_kv_action.index.remove(index_key);
   let index_as_bytes = bincode::serialize(&a_kv_action.index).unwrap();
   a_kv_action.index = std::collections::HashMap::new();
   a_kv_action.insert(index_key, &index_as_bytes).unwrap();
}

fn main() {
   const INDEX_KEY: &ByteStr = b"+index";

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
         let index_as_bytes = kv_action.get(&INDEX_KEY).unwrap().unwrap();
         let index_decoded = bincode::deserialize(&index_as_bytes);
         let index: HashMap<ByteString, u64> = index_decoded.unwrap();
         match index.get(key) {
            None => eprintln!("{:?} not found", key),
            Some(&i) => {
               let kv = kv_action.get_at(i).unwrap();
               println!("{:?}", kv.value)
            }
         }
      }

      "delete" => kv_action.delete(key).unwrap(),

      "insert" => {
         let value = opt_value.expect(&USAGE).as_ref();
         kv_action.insert(key, value).unwrap();
         store_index_on_disk(&mut kv_action, INDEX_KEY);
      }

      "update" => {
         let value = opt_value.expect(&USAGE).as_ref();
         kv_action.update(key, value).unwrap();
         store_index_on_disk(&mut kv_action, INDEX_KEY);
      }
      _ => eprintln!("{}", &USAGE),
   }
}
