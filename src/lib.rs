pub mod ring;

#[cfg(test)]
mod tests {
    use crate::ring;

    #[test]
    fn create_new_ring() {
        let result = ring::RingBuffer::<i32>::new(2);
        assert_eq!(result.is_err(), false, "ring should be creared");

        let result = ring::RingBuffer::<i32>::new(0);
        assert_eq!(result.err(), Some("capacity must be non-negative value"));
    }

    #[test]
    fn offer_and_poll() {
        let mut buffer = ring::RingBuffer::<i32>::new(2).unwrap();

        let res = buffer.offer(10);
        assert!(res.is_ok(), "should be ok to insert");

        let res = buffer.offer(11);
        assert!(res.is_ok(), "should be ok to insert");

        let res = buffer.offer(13);
        assert_eq!(res.err(), Some(ring::Full), "expected error on Full");

        //poll from the buffer and try inserting again

        let res = buffer.poll();
        assert!(res.is_some(), "should return some value");

        let res = buffer.offer(13);

        assert!(
            res.is_ok(),
            "should be able to insert after removing one element"
        );
        //poll twice and 3rd should return None

        assert!(buffer.poll().is_some(), "should be able to poll");
        assert!(buffer.poll().is_some(), "should be able to poll");
        let res = buffer.poll();

        assert!(res.is_none(), "expected None");
    }
}
