fn main() {
    parse_data();
}

fn read(data: &String) {

}

fn parse_data() -> String {
    include_str!("../input.txt")
        .chars()
        .map(to_binary)
        .collect::<Vec<&str>>()
        .join("")
}

fn to_binary<'a>(c: char) -> &'a str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}
