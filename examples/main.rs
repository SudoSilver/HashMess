use HashMess::HashMess;

fn main() {
    let mut example: HashMess = HashMess::new();

    example.insert("num".to_string(), 5);
    example.insert("hello".to_string(), Box::new(|| println!("hello world")) as Box<dyn Fn()>);

    let num: i32 = *example.get::<i32>("num");
    println!("{}", num);

    let func : &Box<dyn Fn()> = example.get::<Box<dyn Fn()>>("hello");
    func();
}
