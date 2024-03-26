use rand::Rng;



pub fn gen_random_usize(max_limit: usize) -> usize {
    // let max_limit = 10_usize.pow(5);
    let mut rng = rand::thread_rng();
    let num: usize = rng.gen_range(0..max_limit);
    return num;
}

pub fn gen_random_vec_i32(size: usize, upper_limit: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng(); // 乱数生成器を初期化
    (0..size).map(|_| rng.gen_range(0..upper_limit)).collect()
}
