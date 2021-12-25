use std::collections::HashMap;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    cache: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            cache: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match self.cache.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                
                self.cache.insert(arg, v);

                v
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let t = |a| a + 2;
    let mut c = Cacher::new(t);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v1, t(1));
    assert_eq!(v2, t(2));
}
