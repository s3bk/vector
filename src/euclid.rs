use euclid::Point2D;

impl<T: Value + Copy> Point for Point2D<T> {
trait Point {
    type V = T;
    fn new(x: Self::V, y: Self::V) -> Self {
        Point2D::new(x, y)
    }
    fn x(&self) -> Self::V {
        self.x
    }
    fn y(&self) -> Self::V {
        self.y
    }
}
