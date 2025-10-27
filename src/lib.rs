//! Crate containing the point struct. Primarily used for
//! represnting a position, such as on a grid or in a 2D array.
use std::{fmt, ops};

/// A 2D co-ordinate.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point {
    /// X co-ordinate
    pub x: i32,
    /// Y co-ordinate
    pub y: i32,
}

impl Point {
    /// The point at (0, 0).
    pub const ORIGIN: Self = Self { x: 0, y: 0 };

    /// Return a new point instance with given x and y positions.
    #[inline(always)]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Returns a vector of points wrapped in options that would be orthogonally adjacent to the point.
    /// Takes a maximum x and y co-ordinate, which the returned points will not
    /// exceed or equal. They will also not be less than 0.
    pub fn get_adjacent(&self, max_x: i32, max_y: i32) -> Vec<Option<Self>> {
        let mut points: Vec<Option<Self>> = Vec::new();
        for p in self.get_all_adjacent() {
            if p.bounds_check(max_x, max_y) {
                points.push(Some(p))
            } else {
                points.push(None)
            }
        }
        points
    }

    /// Returns every point with a distance of 1 away from the point.
    /// Does not include points with non integer co_ordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    ///
    /// let mut p = Point::new(0, 0);
    ///
    /// let expected: Vec<Point> = vec![(1, 0), (0, -1), (-1, 0), (0, 1)].into_iter().map(Point::from).collect();
    /// assert_eq!(p.get_all_adjacent(), expected);
    /// ```
    pub fn get_all_adjacent(&self) -> Vec<Self> {
        let mut points = Vec::new();
        let mut offset = Point { x: 1, y: 0 };
        for _ in 0..4 {
            let new_point = *self + offset;
            points.push(new_point);
            offset.rotate_90_cw_ip();
        }
        points
    }

    /// The same as get_all_adjacent, but also returns diagonally adjacent
    /// points.
    pub fn get_all_adjacent_diagonal(&self) -> Vec<Self> {
        let mut points = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                if x != 0 || y != 0 {
                    points.push(Self::new(self.x + x, self.y + y));
                }
            }
        }
        points
    }

    /// Rotates the co-ordinate in place about the origin by 90 degrees clockwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    ///
    /// let mut p = Point::new(1, 0);
    ///
    /// p.rotate_90();
    /// assert_eq!(p, Point::new(0, -1));
    ///
    /// p.rotate_90();
    /// assert_eq!(p, Point::new(-1, 0));
    ///
    /// p.rotate_90();
    /// assert_eq!(p, Point::new(0, 1));
    /// ```
    #[inline]
	#[deprecated(since="0.5.0", note="Use rotate_90_cw_ip instead.")]
    pub const fn rotate_90(&mut self) {
        (self.x, self.y) = (self.y, -self.x);
    }
	
	/// Returns the co-ordinate rotated about the origin by 90 degrees clockwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    ///
    /// let mut p = Point::new(1, 0);
    ///
    /// assert_eq!(Point::new(0, -1), p.rotate_90_cw());
	/// ```
    #[inline]
    pub const fn rotate_90_cw(&self) -> Self {
        Point::new(self.y, -self.x)
    }
	
	/// Rotates the co-ordinate in place about the origin by 90 degrees clockwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    ///
    /// let mut p = Point::new(1, 0);
    ///
    /// p.rotate_90_cw_ip();
    /// assert_eq!(p, Point::new(0, -1));
    ///
    /// p.rotate_90_cw_ip();
    /// assert_eq!(p, Point::new(-1, 0));
    ///
    /// p.rotate_90_cw_ip();
    /// assert_eq!(p, Point::new(0, 1));
    /// ```
    #[inline]
    pub const fn rotate_90_cw_ip(&mut self) {
        (self.x, self.y) = (self.y, -self.x);
    }
	
	/// Returns the co-ordinate rotated about the origin by 90 degrees anti-clockwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    ///
    /// let mut p = Point::new(1, 0);
    ///
    /// assert_eq!(Point::new(0, 1), p.rotate_90_acw());
	/// ```
    #[inline]
    pub const fn rotate_90_acw(&self) -> Self {
        Point::new(-self.y, self.x)
    }
	
	/// Rotates the co-ordinate in place about the origin by 90 degrees anti-clockwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    ///
    /// let mut p = Point::new(1, 0);
    ///
    /// p.rotate_90_acw_ip();
    /// assert_eq!(p, Point::new(0, 1));
    ///
    /// p.rotate_90_acw_ip();
    /// assert_eq!(p, Point::new(-1, 0));
    ///
    /// p.rotate_90_acw_ip();
    /// assert_eq!(p, Point::new(0, -1));
	/// ```
    #[inline]
    pub const fn rotate_90_acw_ip(&mut self) {
        (self.x, self.y) = (-self.y, self.x);
    }
	
	/// Returns the co-ordinate rotated about the origin by 180 degrees.
	/// Equivalent to [Point::neg] if it was `const`.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    ///
    /// let mut p = Point::new(2, 1);
    /// 
    /// assert_eq!(p.rotate_180(), Point::new(-2, -1));
	/// ```
    #[inline]
    pub const fn rotate_180(&self) -> Self {
        Point::new(-self.x, -self.y)
    }
	
	/// Rotates the co-ordinate in place about the origin by 180 degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    ///
    /// let mut p = Point::new(2, 1);
	/// p.rotate_180_ip();
    /// 
    /// assert_eq!(p, Point::new(-2, -1));
	/// ```
    #[inline]
    pub const fn rotate_180_ip(&mut self) {
        *self = self.rotate_180();
    }
	
    /// Maps the four unit points to 0, 1, 2 and 3 for indexing a
    /// collection with 4 elements.
    ///
    /// > Note: Use of this method outside of the unit points
    /// > will return arbitrary and meaningless values.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    ///
    /// let p0 = Point::new(0, -1); // South
    /// let p1 = Point::new(-1, 0); // West
    /// let p2 = Point::new(0, 1); // North
    /// let p3 = Point::new(1, 0); // East
    ///
    /// assert_eq!(p0.dir(), 0);
    /// assert_eq!(p1.dir(), 1);
    /// assert_eq!(p2.dir(), 2);
    /// assert_eq!(p3.dir(), 3);
    /// ```
    pub const fn dir(&self) -> usize {
        (2 * self.x + self.y + 1).unsigned_abs() as usize
    }

    /// Maps the four unit points to 2, 3, 0, and 1 for indexing a
    /// collection with 4 elements. Hits the index two away from
    /// the result of dir.
    ///
    /// > Note: Use of this method outside of the unit points
    /// > will return arbitrary and meaningless values.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    ///
    /// let p0 = Point::new(0, -1); // South
    /// let p1 = Point::new(-1, 0); // West
    /// let p2 = Point::new(0, 1); // North
    /// let p3 = Point::new(1, 0); // East
    ///
    /// assert_eq!(p0.inv_dir(), 2);
    /// assert_eq!(p1.inv_dir(), 3);
    /// assert_eq!(p2.inv_dir(), 0);
    /// assert_eq!(p3.inv_dir(), 1);
    /// ```
    pub const fn inv_dir(&self) -> usize {
        let dir = self.dir();

        if dir < 2 { dir + 2 } else { dir - 2 }
    }

    /// Returns the Euclidean distance between self and other,
    /// using the pythagorean theorem.
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    ///
    /// let p1 = Point::new(3, 0);
    /// let p2 = Point::new(11, 6);
    ///
    /// // 8^2 + 6^2 = 100
    /// // sqrt(100) = 10
    /// assert_eq!(p1.dist(p2), 10.0f64)
    /// ```
    #[inline(always)]
    pub fn dist(self, other: Self) -> f64 {
        (self.dist_squared(other) as f64).sqrt()
    }

    /// Returns the squared Euclidean distance between self and other.
    /// This is more performant than dist as the square root
    /// step is skipped.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    ///
    /// let p1 = Point::new(0, 0);
    /// let p2 = Point::new(2, 1);
    ///
    /// // 2^2 + 1^2 = 5
    /// assert_eq!(p1.dist_squared(p2), 5);
    /// ```
    #[inline]
    pub fn dist_squared(self, other: Self) -> i32 {
        let disp = self - other;
        disp.x * disp.x + disp.y * disp.y
    }
	
	/// Returns the manhattan distance between self and other.
	///
	/// # Examples
	///
	/// ```
    /// use point::Point;
    ///
    /// let p1 = Point::new(0, 0);
    /// let p2 = Point::new(2, 1);
    ///
    /// // 2-0 = 2, 1-0 = 1, 1+2=3
    /// assert_eq!(p1.manhattan_dist(p2), 3);
    /// ```
    #[inline]
    pub fn manhattan_dist(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Helper for converting an index into a two dimensional co-ordinate
    /// given the width of the space. It is assumed that the index is within
    /// the height boundary, so it is not required as an argument.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    ///
    /// let width: usize = 3;
    /// let _height: usize = 3;
    ///
    /// // Index is known, but not co_ord.
    /// let index: usize = 4;
    /// let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    ///
    /// // Our grid looks like the following (! represents the desired position):
    /// // # # #
    /// // # ! #
    /// // # # #
    /// // From this, it is clear that our index is at (1, 1).
    ///
    /// let converted = Point::convert_up(index, width);
    ///
    /// assert_eq!(converted, Point::new(1, 1));
    /// assert_eq!(grid[converted.y as usize][converted.x as usize], 1);
    /// ```
    pub const fn convert_up(index: usize, width: usize) -> Point {
        let width = width as i32;
        let index = index as i32;
        Point {
            x: index % width,
            y: index / width,
        }
    }

    /// Performs the inverse of convert_up; turns a point into an index
    /// given a width.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    ///
    /// let width: usize = 3;
    /// let _height: usize = 3;
    ///
    /// // Co_ord is known, but not index.
    /// let co_ord = Point::new(1, 1);
    /// let values = vec![0, 0, 0, 0, 1, 0, 0, 0, 0];
    ///
    /// // Our vector looks like the following (! represents the desired position):
    /// // 0: #, 1: #, 2: #, 3: #, 4: !, 5: #, 6: #, 7: #, 8: #
    /// // From this, it is clear that our co_ord is at index 4.
    ///
    /// let converted = co_ord.convert_down(width);
    /// assert_eq!(converted, 4);
    /// assert_eq!(values[converted], 1);
    /// ```
    pub const fn convert_down(&self, width: usize) -> usize {
        self.y as usize * width + self.x as usize
    }

    /// Returns true if the point's x co-ordinate is less than max_x and
    /// greater than or equal to zero. Performs a similar check with the
    /// y co-ordinate. In all other cases, returns false.
    ///
    /// # Examples
    ///
    /// ```
    /// use point::Point;
    ///
    /// let p1 = Point::new(4, 5);
    /// let p2 = Point::new(-4, 5);
    ///
    /// assert!(p1.bounds_check(10, 10));
    /// assert!(!p1.bounds_check(4, 10));
    /// assert!(!p2.bounds_check(10, 10));
    /// ```
    pub const fn bounds_check(&self, max_x: i32, max_y: i32) -> bool {
        0 <= self.x && self.x < max_x && 0 <= self.y && self.y < max_y
    }
	
	/// Returns an iterator over all the points on the line between src and dest.
	/// Uses [Bresenham's Line Algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm) to do so.
	pub fn plot_line(src: Self, dest: Self) -> LineIter {
		LineIter::new(src, dest)
	}
}

/// An iterator over the points on a line between and including two points.
pub struct LineIter {
	dx: i32,
	sx: i32,
	dy: i32,
	sy: i32,
	err: i32,
	cur_x: i32,
	cur_y: i32,
	end_x: i32,
	end_y: i32,
}

impl LineIter {
	fn new(src: Point, dest: Point) -> Self {
		let cur_x = src.x;
		let end_x = dest.x;
		
		let cur_y = src.y;
		let end_y = dest.y;
		
		let dx = (end_x - cur_x).abs();
		let sx = if cur_x < end_x { 1 } else { -1 };
		
		let dy = -(end_y - cur_y).abs();
		let sy = if cur_y < end_y { 1 } else { -1 };
		
		let err = dx + dy;
		
		Self {
			dx,
			sx,
			dy,
			sy,
			err,
			cur_x,
			cur_y,
			end_x,
			end_y,
		}
	}
}

impl Iterator for LineIter {
	type Item = Point;
	
	fn next(&mut self) -> Option<Self::Item> {		
		let nx = Point::new(self.cur_x, self.cur_y);
		let e2 = 2 * self.err;
		
		if e2 >= self.dy {
            if self.cur_x == self.end_x { return None }
            self.err += self.dy;
            self.cur_x += self.sx;
		}
        if e2 <= self.dx {
            if self.cur_y == self.end_y { return None }
            self.err += self.dx;
            self.cur_y += self.sy;
        }
		
		Some(nx)
	}
}

impl ops::Add for Point {
    type Output = Self;

    /// Returns a new point containing the sum of the points' x and y values.
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for Point {
    type Output = Self;

    /// Returns a new point containing the difference of the points' x and y values.
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::Div<i32> for Point {
    type Output = Self;

    /// Returns a new point containg each co-ordinate divided by the given divisor.
    fn div(self, other: i32) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> From<(T, T)> for Point
where
    T: Into<i32>,
{
    fn from(val: (T, T)) -> Self {
        Self {
            x: val.0.into(),
            y: val.1.into(),
        }
    }
}

impl<U> From<Point> for (U, U)
where
    i32: Into<U>,
{
    fn from(val: Point) -> Self {
        (val.x.into(), val.y.into())
    }
}
