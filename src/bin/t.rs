struct MyStruct {
    value: i32,
    pub serect: u32,
}

impl MyStruct {
    fn new(value: i32) -> Self {
        MyStruct { value, serect: 0 }
    }

    fn update_value(&mut self, new_value: i32) {
        self.value = new_value;
    }
}

struct MyStruct2 {}

impl MyStruct2 {
    fn new() -> Self {
        MyStruct2 {}
    }

    fn store(&self, mut ss: &mut MyStruct) {
        ss.serect = 10;
    }
}

fn main() {
    let mut my_object = MyStruct::new(5);
    let my_object2 = MyStruct2::new();
    my_object2.store(&mut my_object);
    my_object2.store(&mut my_object);
    println!("my_object.value: {}", my_object.serect);
}

fn modify_object(obj: &mut MyStruct) {
    obj.update_value(10);
}
