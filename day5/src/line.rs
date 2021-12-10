#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(u16, u16);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line(Point, Point);

impl Line {
    pub fn get_points(&self) -> Vec<Point> {
        match self {
            // Vertical lines
            Line(Point(x1, y1), Point(x2, y2)) if x1 == x2 && y1 <= y2 => {
                (*y1..=*y2).map(|y| Point(*x1, y)).collect()
            }
            Line(Point(x1, y1), Point(x2, y2)) if x1 == x2 && y2 <= y1 => {
                (*y2..=*y1).map(|y| Point(*x1, y)).collect()
            }
            // Horizontal lines
            Line(Point(x1, y1), Point(x2, y2)) if y1 == y2 && x1 <= x2 => {
                (*x1..=*x2).map(|x| Point(x, *y1)).collect()
            }
            Line(Point(x1, y1), Point(x2, y2)) if y1 == y2 && x2 <= x1 => {
                (*x2..=*x1).map(|x| Point(x, *y1)).collect()
            }
            // Diagonal lines
            Line(Point(x1, y1), Point(x2, y2)) if y1 < y2 && x1 < x2 => (*x1..=*x2)
                .zip(*y1..=*y2)
                .map(|(x, y)| Point(x, y))
                .collect(),

            Line(Point(x1, y1), Point(x2, y2)) if y1 < y2 && x1 > x2 => (*x2..=*x1)
                .rev()
                .zip(*y1..=*y2)
                .map(|(x, y)| Point(x, y))
                .collect(),

            Line(Point(x1, y1), Point(x2, y2)) if y1 > y2 && x1 > x2 => (*x2..=*x1)
                .zip(*y2..=*y1)
                .map(|(x, y)| Point(x, y))
                .collect(),

            Line(Point(x1, y1), Point(x2, y2)) if y1 > y2 && x1 < x2 => (*x1..=*x2)
                .zip((*y2..=*y1).rev())
                .map(|(x, y)| Point(x, y))
                .collect(),
            _ => vec![],
        }
    }
}

impl From<String> for Line {
    fn from(line: String) -> Self {
        let splitted: Vec<&str> = line.split(" -> ").collect();
        let p1: Vec<&str> = splitted[0].split(',').collect();
        let p2: Vec<&str> = splitted[1].split(',').collect();
        let x1: u16 = p1[0].parse().unwrap();
        let y1: u16 = p1[1].parse().unwrap();
        let x2: u16 = p2[0].parse().unwrap();
        let y2: u16 = p2[1].parse().unwrap();
        Line(Point(x1, y1), Point(x2, y2))
    }
}

#[cfg(test)]
mod tests {
    use super::{Line, Point};

    #[test]
    fn gets_diagonal_line_points() {
        let line = Line(Point(5, 5), Point(8, 2));
        let points = line.get_points();

        assert_eq!(points, [Point(5, 5), Point(6, 4), Point(7, 3), Point(8, 2)])
    }

    #[test]
    fn gets_diagonal_line_points2() {
        let line = Line(Point(8, 2), Point(5, 5));
        let points = line.get_points();

        assert_eq!(points, [Point(8, 2), Point(7, 3), Point(6, 4), Point(5, 5)])
    }
}
