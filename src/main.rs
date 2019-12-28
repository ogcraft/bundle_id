// --------------------------
use std::env;
use std::path::Path;
use std::path::PathBuf;


fn main() {
    println!("Hello, world!");
    let _a: u32 = 11;
    let aa = 11.0 + 3.5;
    let s = "sAS".as_bytes();
    println!("aaa: {}, {:?}", aa,s);
    let ss = "aaaaaa".to_owned();
    println!("ss = {:?}", ss) 
}

fn find_file_in_dir(root_path: &Path) -> PathBuf {

    let root = PathBuf::from(root_path);

    root
}

mod tests {
    use super::*;

    #[test]
    fn test_find_file_in_dir() {
        let current_dir = env::current_dir().expect("Cannot get current dir");
        println!("current_dir: {:?}", current_dir);

        let file_found = find_file_in_dir(&current_dir);
        println!("file_found: {:?}", file_found);

        assert_eq!(true, true);
    }


}
