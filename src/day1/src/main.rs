use day1::cal_from_str;

fn main() {
    let fdata = include_str!("data/inputs.txt");

    let mut tot = 0;
    for l in fdata.lines() {
        tot += cal_from_str(l);
    }

    println!("Total: {tot}");
}
