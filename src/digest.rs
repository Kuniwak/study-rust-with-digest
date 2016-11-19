use std::option::Option;
use std::io::{Read, Write};
use std::process::{Command, Stdio};


fn main() {
    let str = "abcd";

    match digest(&str) {
        Some(hash) => println!("HASH: {}", hash),
        None => println!("HASH: NONE"),
    }
}


fn digest(src: &str) -> Option<String> {
    let mut out = String::new();

    return Command::new("openssl")
        .arg("dgst")
        .arg("-sha512")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .ok()
        .and_then(|p| all(p.stdin, p.stdout))
        .and_then(|(mut stdin, stdout)| stdin.write_all(src.as_bytes()).ok().map(|_| stdout))
        .and_then(|mut stdout| stdout.read_to_string(&mut out).ok())
        .map(|_| out.trim().to_string());
}


fn all<T, R>(a1: Option<T>, a2: Option<R>) -> Option<(T, R)> {
    return match (a1, a2) {
        (Some(b1), Some(b2)) => Some((b1, b2)),
        _ => None,
    }
}
