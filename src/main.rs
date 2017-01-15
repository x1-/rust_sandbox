extern crate rustc_serialize;
extern crate time;

mod rustc_json;
mod fibonacci;

// 時間計測のコードを挿入するマクロ
// http://qiita.com/tatsuya6502/items/7ffc623fc60be0220409
macro_rules! timeit {
    ($label: expr, $code: expr) => ({
        let start = time::get_time();
        let ret = $code;
        let end = time::get_time();

        let microsecs = (end.sec - start.sec) as f64 * 1_000_000.0 +
            (end.nsec - start.nsec) as f64 / 1_000.0;

        println!("{}: {} micro-seconds", $label, microsecs);

        ret
    })
}

fn main() {
    // JSON
    timeit!( "parse_str"     , rustc_json::parse_str() );
    timeit!( "serialize"     , rustc_json::serialize() );
    timeit!( "custom_mapping", rustc_json::custom_mapping() );

    // Iterator printing
    println!("\n*---*---*---*---*---*");
    println!("Iterator 0..3");
    println!("*---*---*---*---*---*");
    for i in 0..3 {
        println!("> {}", i);
    }

    timeit!( "take4"      , fibonacci::take4() );
    timeit!( "skip4take10", fibonacci::skip4take10() );
}
