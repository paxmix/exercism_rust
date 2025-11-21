pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_index = match student {
        "Alice" => 0,
        "Bob" => 1,
        "Charlie" => 2,
        "David" => 3,
        "Eve" => 4,
        "Fred" => 5,
        "Ginny" => 6,
        "Harriet" => 7,
        "Ileana" => 8,
        "Joseph" => 9,
        "Kincaid" => 10,
        "Larry" => 11,
        _ => panic!("Illegal student"),
    };

    diagram
        .lines()
        .flat_map(|line| {
            line[student_index * 2..=student_index * 2 + 1]
                .chars()
                .map(|c| match c {
                    'G' => "grass",
                    'C' => "clover",
                    'R' => "radishes",
                    'V' => "violets",
                    _ => panic!("Illegal plant"),
                })
        })
        .collect()
}
