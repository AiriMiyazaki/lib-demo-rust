// 乱数を生成してくれる関数を作る
// 外部クレートから取りこみ
// useとmodの違いがいまいちわかっていない
use rand::Rng;

// thread_rng, genはrandにデフォルトで用意されているもの
pub fn gen_ran() -> u8 {
    // 乱数生成器を作っている
    let mut rng = rand::thread_rng();
    // 乱数生成期rngに対してgen()を実行すると乱数が生成される
    let n: u8 = rng.gen();
    n
}