#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(u16, u16);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line(Point, Point);

impl Line {
    pub fn get_points(&self) -> Vec<Point> {
        match self {
            Line(Point(x1, y1), Point(x2, y2)) if x1 == x2 && y1 <= y2 => {
                (*y1..=*y2).map(|y| Point(*x1, y)).collect()
            }
            Line(Point(x1, y1), Point(x2, y2)) if x1 == x2 && y2 <= y1 => {
                (*y2..=*y1).map(|y| Point(*x1, y)).collect()
            }
            Line(Point(x1, y1), Point(x2, y2)) if y1 == y2 && x1 <= x2 => {
                (*x1..=*x2).map(|x| Point(x, *y1)).collect()
            }
            Line(Point(x1, y1), Point(x2, y2)) if y1 == y2 && x2 <= x1 => {
                (*x2..=*x1).map(|x| Point(x, *y1)).collect()
            }
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
