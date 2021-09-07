#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for NovaError {}

pub fn run() -> Result<(), Box<dyn Error>> {
    let condition = true;

    if condition {
        return Err(Box::new(MyError("Oops".into())));
    }

    Ok(())
}