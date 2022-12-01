pub fn input(day: &str) -> String {
    ureq::get(&format!("https://adventofcode.com/2022/day/{}/input", day))
        .set(
            "Cookie",
            &format!(
                "session={}",
                std::env::var("AOC_SESSION").expect("set AOC_SESSION env")
            ),
        )
        .call()
        .unwrap()
        .into_string()
        .unwrap()
}
