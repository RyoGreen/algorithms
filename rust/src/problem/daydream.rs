use proconio::input;

pub fn day_dream() {
    input! {
        s: String,
    };
    let keywords = vec!["dream", "dreamer", "erase", "eraser"];
    for i in 0..keywords.len() {
        if s == keywords[i] {
            println!("YES");
            return;
        }
        for j in i..keywords.len() {
            if s == keywords[i].to_string() + keywords[j] {
                println!("YES");
                return;
            }
            for k in j..keywords.len() {
                if s == keywords[i].to_string() + keywords[j] + keywords[k] {
                    println!("YES");
                    return;
                }
                for l in k..keywords.len() {
                    if s == keywords[i].to_string() + keywords[j] + keywords[k] + keywords[l] {
                        println!("YES");
                        return;
                    }
                }
            }
        }
    }
}
