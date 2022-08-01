#![allow(dead_code)]
/// Easy - Twitter
/// e-commerce record last N order ids in a log

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut order_list = OrderList::new(3);
        order_list.record(1);
        order_list.record(2);
        order_list.record(3);
        order_list.record(4);
        order_list.record(5);

        assert_eq!(order_list.get_last(0), Some(5));
        assert_eq!(order_list.get_last(0), Some(4));
        assert_eq!(order_list.get_last(0), Some(3));
        assert_eq!(order_list.get_last(0), None);
    }
}

struct OrderList {
    index: usize,
    log: Vec<Option<i32>>
}

impl OrderList {
    fn new(n: usize) -> OrderList {
        OrderList {
            index: 0,
            log: (0..n).map(|_| None).collect(),
        }
    }

    /// Adds an order to the list
    fn record(&mut self, order_id: i32) {
        unimplemented!();
    }

    /// Gets the ith last element from the list
    fn get_last(&self, i: usize) -> Option<i32> {
        unimplemented!();
    }
}
