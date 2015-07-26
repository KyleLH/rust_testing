enum OptionInt {
    Missing,
    ValidInt(i32)

}

fn test(a: i32) -> OptionInt {
    OptionInt::Missing
}

fn main() {
    let a = test(3);
    match a {
        OptionInt::Missing => {
            println!("Missing");
        }
        OptionInt::ValidInt(x) => {}
    }
}
