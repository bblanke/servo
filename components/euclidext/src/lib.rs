use euclid::default::{Point2D, Rect, Size2D};

/*pub trait RectToi32 {
    fn to_i32(&self) -> Rect<i32>;
    fn ceil(&self) -> Rect<f64>;
}

impl RectToi32 for Rect<f64> {
    fn to_i32(&self) -> Rect<i32> {
        Rect::new(
            Point2D::new(
                self.origin.x.to_i32().unwrap(),
                self.origin.y.to_i32().unwrap(),
            ),
            Size2D::new(
                self.size.width.to_i32().unwrap(),
                self.size.height.to_i32().unwrap(),
            ),
        )
    }

    fn ceil(&self) -> Rect<f64> {
        Rect::new(
            Point2D::new(self.origin.x.ceil(), self.origin.y.ceil()),
            Size2D::new(self.size.width.ceil(), self.size.height.ceil()),
        )
    }
}*/

pub trait Size2DExt {
    fn to_u64(&self) -> Size2D<u64>;
}

impl Size2DExt for Size2D<f64> {
    fn to_u64(&self) -> Size2D<u64> {
        self.cast()
    }
}

impl Size2DExt for Size2D<u32> {
    fn to_u64(&self) -> Size2D<u64> {
        self.cast()
    }
}

pub trait RectExt {
    fn to_u64(&self) -> Rect<u64>;
}

impl RectExt for Rect<f64> {
    fn to_u64(&self) -> Rect<u64> {
        self.cast()
    }
}

impl RectExt for Rect<u64> {
    fn to_u64(&self) -> Rect<u64> {
        self.cast()
    }
}