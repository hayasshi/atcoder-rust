fn main() {
    let n: usize = {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf.trim().parse().unwrap()
    };

    let mut wl: Vec<Work> = (0..n)
        .map(|_| {
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            let mut iter = buf.split_whitespace();
            Work {
                work_time: iter.next().unwrap().parse().unwrap(),
                end_time: iter.next().unwrap().parse().unwrap()
            }
        })
        .collect();

    wl.sort_by(|lw, rw| lw.end_time.cmp(&rw.end_time));
    let mut sum = 0;
    for work in &mut wl {
        sum = sum + work.work_time;
        if sum > work.end_time {
            print!("No");
            std::process::exit(0);
        };
    }
    print!("Yes");
}

struct Work {
    work_time: usize,
    end_time: usize,
}
