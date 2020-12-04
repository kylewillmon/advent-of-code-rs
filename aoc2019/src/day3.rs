#[derive(PartialEq,PartialOrd,Debug,Clone)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Point
    {
        Point { x, y }
    }

    fn origin() -> Point
    {
        Self::new(0, 0)
    }

    fn translate(&self, dir: char, len: u32) -> Point
    {
        match dir {
            'R' => Point { x: self.x + len, y: self.y },
            'L' => Point { x: self.x - len, y: self.y },
            'U' => Point { x: self.x, y: self.y + len },
            'D' => Point { x: self.x, y: self.y - len },
            _ => panic!("invalid direction"),
        }
    }
}

#[derive(PartialEq,Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(a: Point, b: Point) -> Self {
        if a < b {
            Line {
                start: a,
                end: b,
            }
        } else {
            Line {
                start: b,
                end: a,
            }
        }
    }
}

fn parse_motion(motion: &str) -> Result<(char, u32), String>
{
    let (dir, len) = motion.split_at(1);
    let dir = dir.chars().nth(0).ok_or("direrction not found")?;
    let len = len.parse::<u32>().map_err(|e| format!("{}", e))?;
    (dir, len)
}

fn parse_lines(input: String) -> Result<Vec<Line>, String>
{
    let mut lines = Vec::new();
    let mut cur = Point::origin();
    for motion in input.split(',') {
        let (dir, lenstr) = motion.split_at(1);
        let len = lenstr.parse::<u32>().map_err(|e| format!("{}", e))?;
        let next = cur.translate(dir.chars().nth(0).ok_or("direrction not found")?, len);

        lines.push(Line::new(cur, next.clone()));
        cur = next;
    }
    Ok(lines)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let input = "R3,U1,L2,D1".to_string();
        let lines = vec!(
            Line::new(Point::new(0, 0), Point::new(3, 0)),
            Line::new(Point::new(3, 0), Point::new(3, 1)),
            Line::new(Point::new(3, 1), Point::new(1, 1)),
            Line::new(Point::new(1, 1), Point::new(1, 0)),
        );

        assert_eq!(Ok(lines), parse_lines(input));
    }
}