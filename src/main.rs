extern crate rand;
mod lib;

#[cfg(test)]
mod tests {
    use crate::lib::*;
    use rand::prelude::*;
    #[test]
    fn add() {
        assert_eq!(bin_add(10, 11), 21);
        assert_eq!(bin_add(1313230, 1424211), 2737441);
        assert_eq!(bin_add(1, 14), 15);
        assert_eq!(bin_add(4410, 11), 4421);
        assert_eq!(bin_add(6, 51), 57);
        assert_eq!(bin_add(1051423, 14451561), 15502984);
    }

    #[test]
    fn add_random() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let (x, y) = (rng.gen::<u16>() as u32, rng.gen::<u16>() as u32);
            assert_eq!(bin_add(x, y), x + y);
        }
    }

    #[test]
    fn sub() {
        assert_eq!(bin_sub(10, 11), Err(-1));
        assert_eq!(bin_sub(1313230, 1424211), Err(-110981));
        assert_eq!(bin_sub(1, 14), Err(-13));
        assert_eq!(bin_sub(4410, 11), Ok(4399));
        assert_eq!(bin_sub(51, 6), Ok(45));
        assert_eq!(bin_sub(14451561, 1051423), Ok(13400138));
    }

    #[test]
    fn sub_random() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let (x, y) = (rng.gen::<u16>() as u32, rng.gen::<u16>() as u32);
            assert_eq!(bin_sub(x, y), {
                match x < y {
                    false => Ok(x - y),
                    _     => Err(x as i32 - y as i32),
                }
            });
        }
    }

    #[test]
    fn mul() {
        assert_eq!(bin_mul(10, 11), 110);
        assert_eq!(bin_mul(1313230, 14), 18385220);
        assert_eq!(bin_mul(1, 14), 14);
        assert_eq!(bin_mul(4410, 11), 48510);
        assert_eq!(bin_mul(6, 51), 306);
        assert_eq!(bin_mul(1051423, 1445), 1519306235);
    }


    #[test]
    fn mul_random() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let (x, y) = (rng.gen::<u16>() as u32, rng.gen::<u16>() as u32);
            assert_eq!(bin_mul( x, y), x * y);
        }
    }

    #[test]
    fn div() {
        assert_eq!(bin_div(10, 11), (0, 10));
        assert_eq!(bin_div(131, 14), (9, 5));
        assert_eq!(bin_div(1, 14), (0, 1));
        assert_eq!(bin_div(440, 11), (40, 0));
        assert_eq!(bin_div(6, 51), (0, 6));
        assert_eq!(bin_div(10514, 500), (21, 14));
    }

}
