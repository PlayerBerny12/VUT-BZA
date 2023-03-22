pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn sub(left: usize, right: usize) -> usize {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        // let m = ManagerRNG {priority: 3};
        let n = ManagerRNG::new(3);
        println!("TU: {:#?}", n);
        assert_eq!(result, 4);
    }
}


#[derive(Debug)]
struct ManagerRNG {priority: i32}

impl ManagerRNG {
    pub fn new(prio: i32) -> ManagerRNG {
        ManagerRNG {priority: prio}
    }
}


trait NextNumber {

}