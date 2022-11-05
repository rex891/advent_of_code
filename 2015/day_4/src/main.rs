fn main() {
    let code = "ckczppom";

    let mut i = 0;
    loop {
        let v = md5::compute(format!("{code}{i}").as_bytes());
        let x = format!("{:?}", v);
        if x.starts_with("000000") {
            println!("val: {x}, i: {i}");
            break;
        }
        i += 1;
    }
}
