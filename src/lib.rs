// 作成時にデフォルトであるやつ。消してOK
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

// #https://crates.io/crates/rand

mod generator;

pub fn print_random_number () {
    let n = generator::gen_ran();
    println!("Random u8: {}", n);
}