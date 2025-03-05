use std::io::stdin;

fn main() {
    let mut s = String::new();

    let _ = stdin().read_line(&mut s);

    let (mut remaining, mut result) = solve(&s);

    let mut results = Vec::new();

    while result != (0, 0) {
        results.push(result);
        (remaining, result) = solve(&remaining);
    }

    println!("{}", results.len());
    for r in results {
        println!("{} {}", r.0, r.1);
    }
}

fn solve(s: &str) -> (String, (usize, usize)) {
    let normalized = s.split_whitespace().collect::<Vec<&str>>().join(" ");

    let result = normalized
        .split("kalan tar")
        .map(|sub| sub.split("kalantar").collect())
        .collect::<Vec<Vec<&str>>>();

    let kalan_tars = result.len() - 1;
    let kalantars = result
        .clone()
        .into_iter()
        .flatten()
        .collect::<Vec<&str>>()
        .len()
        - result.len();

    (
        result.into_iter().flatten().collect::<String>(),
        (kalantars, kalan_tars),
    )
}
