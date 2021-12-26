#![allow(dead_code)]

#[derive(Debug)]
struct Doors {
    doors: [bool; 100],
}

impl Doors {
    fn new() -> Doors {
        Doors {
            doors: [false; 100],
        }
    }

    fn get_doors(&self) -> [bool; 100] {
        self.doors
    }

    fn toggle(&mut self, div: usize) -> () {
        for i in 0..self.doors.len() {
            if (i + 1) % div == 0 {
                self.doors[i] = !self.doors[i];
            }
        }
    }

    fn count_open(&self) -> (Vec<usize>, Vec<usize>) {
        let mut open = vec![];
        let mut closed = vec![];

        for i in 0..self.doors.len() {
            if self.doors[i] {
                open.push(i + 1);
            } else {
                closed.push(i + 1)
            }
        }

        (open, closed)
    }
}

#[test]
fn theres_100_doors() {
    let doors = Doors::new();
    let doors = doors.get_doors();

    assert_eq!(doors[0], false);
    assert_eq!(doors[99], false);
}

#[test]
fn toggle_doors() {
    let mut doors = Doors::new();
    doors.toggle(2);

    let doors = doors.get_doors();

    assert_eq!(doors[1], true);
    assert_eq!(doors[2], false);
}

#[test]
fn count_doors() {
    let mut doors = Doors::new();
    doors.doors[1] = true;

    let (open, closed) = doors.count_open();

    assert_eq!(open.len(), 1);
    assert_eq!(closed.len(), 99);
}

#[test]
fn walking_toggling() {
    let mut doors = Doors::new();

    for i in 1..=doors.doors.len() {
        doors.toggle(i);
    }

    println!("{:#?}", doors.count_open());
}
