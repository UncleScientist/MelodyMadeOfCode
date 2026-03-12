fn main() {
    let bone = quest2::load_file("input/everybody_codes_e3_q02_p1.txt");
    println!("part 1 = {}", quest2::part1::run(bone[0]));

    let bone = quest2::load_file("input/everybody_codes_e3_q02_p2.txt");
    println!("part 2 = {}", quest2::part2::run(&bone[0]));

    let bones = quest2::load_file("input/everybody_codes_e3_q02_p3.txt");
    println!("part 3 = {}", quest2::part3::run(bones));
}
