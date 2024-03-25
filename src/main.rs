struct MyStruct;

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Bye!");
    }
}

fn move_me(a: MyStruct) -> MyStruct {
    println!("Entered function");
    a
}

fn main() {
    println!("Started");
    let a = MyStruct;
    let b = move_me(a);
    println!("Exited Function");
}