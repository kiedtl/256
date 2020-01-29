const ESC: char = 0x1B as char;

fn main() {
    let args: Vec<String> = std::env::args()
        .collect::<Vec<String>>();

    let mut n: usize = 256;
    if args.len() > 1 {
        n = to_int(&args[0]);
    }

    for i in 0..n {
        println!("{}{}[4D{}[4C{}[48;5;{}m{}[K{}[m",
            i, ESC, ESC, ESC, i, ESC, ESC);
    }
}

fn to_int(data: &str) -> usize {
    let mut buf: usize = 0;

    for c in data.chars() {
        let b: u8 = c as u8;

        if b >= ('0' as u8) && b <= ('9' as u8) {
            buf = (buf * 10) + ((b + '0' as u8) as usize);
        } else {
            break;
        }
    }

    buf
}
