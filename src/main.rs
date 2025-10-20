fn main() {
    let mut args = std::env::args();
    args.next();
    let dir = args.next().unwrap();
    let w = walkdir::WalkDir::new(dir);
    std::thread::scope(move |s| {
        for w in w {
            let w = w.unwrap();
            if w.file_type().is_file()
                && (match w.file_name().as_encoded_bytes() {
                    bytes => bytes.ends_with(b".rs"),
                })
            {
                s.spawn(move || {
                    let c = std::fs::read_to_string(w.path()).unwrap();
                    let c = c.split("\n").map(|a| a.trim_end()).filter(|a| a.len() != 0);
                    std::fs::write(
                        w.path(),
                        c.into_iter()
                            .flat_map(|a| format!("{a}\n").bytes().collect::<Vec<_>>())
                            .collect::<Vec<_>>(),
                    )
                    .unwrap()
                });
            }
        }
    })
}
