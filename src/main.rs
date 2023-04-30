fn main() {
    let arr: [i8; 6] = [0, 2, 4, 6, 8, 10];
    print!(
        "{},{},{},{},{},{}",
        arr[0], arr[1], arr[2], arr[3], arr[4], arr[5]
    );
    let persons: (&str, i8, &str, i8, &str, i8) = ("Alex", 21, "Abe", 22, "Anna", 23);
    print!(
        "{}:{}, {}:{}, {}:{}",
        persons.0, persons.1, persons.2, persons.3, persons.4, persons.5
    );
}
