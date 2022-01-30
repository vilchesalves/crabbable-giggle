#![allow(dead_code)]

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        let data = &self.data;
        println!("I'm dropping the pointer, it had data {data}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn main() {
        let _c = CustomSmartPointer {
            data: String::from("Point C"),
        };
        let _d = CustomSmartPointer {
            data: String::from("Point D"),
        };

        println!("Pointers created.");
        drop(_c);
        println!("end of scope");
    }
}
