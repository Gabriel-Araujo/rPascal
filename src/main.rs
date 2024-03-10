mod token;
mod scanner;

fn main() {
    let tokens = scanner::Scanner::new("pr:=6.5;").init();

    match tokens {
        Ok(values) => {values.iter().for_each(|e| println!("{e}"))}
        Err(e) => {e.print()}
    }
}
