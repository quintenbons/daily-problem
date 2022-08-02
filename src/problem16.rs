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

        assert_eq!(order_list.get_last(0), &Some(2));
        assert_eq!(order_list.get_last(1), &Some(1));
        assert_eq!(order_list.get_last(2), &None);
        assert_eq!(order_list.get_last(3), &None);

        order_list.record(4);
        order_list.record(5);

        assert_eq!(order_list.get_last(0), &Some(5));
        assert_eq!(order_list.get_last(1), &Some(4));
        assert_eq!(order_list.get_last(2), &Some(2));
        assert_eq!(order_list.get_last(3), &None);
    }
}

struct OrderList {
    index: usize,
    log: Vec<Option<i32>>,
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
        self.log[self.index] = Some(order_id);
        self.index = (self.index + 1) % self.log.len();
    }

    /// Gets the ith last element from the list
    fn get_last<'a>(&'a self, i: usize) -> &'a Option<i32> {
        if i < self.log.len() {
            let key = (self.index + self.log.len() - i - 1) % self.log.len();
            &self.log[key]
        } else {
            &None
        }
    }
}
