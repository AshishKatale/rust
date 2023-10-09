use std::env;
pub fn path_shorten() {
    let args: Vec<String> = env::args().collect();

    let len = if let Some(shorten_len) = args.iter().nth(1) {
        shorten_len.parse().expect(&format!(
            "Error: expected interger value, got: {}",
            shorten_len
        ))
    } else {
        2
    };

    let path: String = if let Some(p) = args.iter().nth(2) {
        p.to_string()
    } else {
        env::var("PWD").unwrap_or("/".to_string())
    };

    let short_path = path
        .split("/")
        .collect::<Vec<&str>>()
        .split_last()
        .and_then(|(dirname, path)| {
            let sp = path
                .iter()
                .map(|dir| dir.get(..len).unwrap_or(dir))
                .collect::<Vec<&str>>()
                .join("/");
            Some(sp + "/" + dirname)
        })
        .unwrap_or(path.to_string());

    println!("{}", short_path);
}

