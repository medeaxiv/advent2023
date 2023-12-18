use nalgebra::Vector2;

pub type Position = Vector2<usize>;

impl std::ops::AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Up => {
                self.y = self.y.wrapping_sub(1);
            }
            Direction::Down => {
                self.y = self.y.wrapping_add(1);
            }
            Direction::Left => {
                self.x = self.x.wrapping_sub(1);
            }
            Direction::Right => {
                self.x = self.x.wrapping_add(1);
            }
        }
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Self;

    fn add(mut self, rhs: Direction) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::SubAssign<Direction> for Position {
    fn sub_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Up => {
                self.y = self.y.wrapping_add(1);
            }
            Direction::Down => {
                self.y = self.y.wrapping_sub(1);
            }
            Direction::Left => {
                self.x = self.x.wrapping_add(1);
            }
            Direction::Right => {
                self.x = self.x.wrapping_sub(1);
            }
        }
    }
}

impl std::ops::Sub<Direction> for Position {
    type Output = Self;

    fn sub(mut self, rhs: Direction) -> Self::Output {
        self -= rhs;
        self
    }
}

pub trait TileChar {
    fn to_char(&self) -> char;
}

pub struct Grid<T> {
    width: usize,
    height: usize,
    entries: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, entries: Vec<T>) -> Self {
        assert_eq!(width * height, entries.len());

        Self {
            width,
            height,
            entries,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn index(&self, position: Position) -> Option<usize> {
        if position.x < self.width && position.y < self.height {
            Some(position.y * self.width + position.x)
        } else {
            None
        }
    }

    pub fn position(&self, index: usize) -> Position {
        Position::new(index % self.width, index / self.width)
    }

    pub fn get(&self, position: Position) -> Option<&T> {
        self.index(position).and_then(|idx| self.entries.get(idx))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.entries.iter()
    }
}

impl<T> std::fmt::Debug for Grid<T>
where
    T: TileChar,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        write!(f, "┌")?;
        for _ in 0..self.width {
            write!(f, "─")?;
        }
        writeln!(f, "┐")?;
        write!(f, "│")?;

        for (idx, tile) in self.entries.iter().enumerate() {
            if idx > 0 && idx % self.width == 0 {
                writeln!(f, "│")?;
                write!(f, "│")?;
            }

            write!(f, "{}", tile.to_char())?;
        }

        writeln!(f, "|")?;
        write!(f, "└")?;
        for _ in 0..self.width {
            write!(f, "─")?;
        }
        writeln!(f, "┘")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const fn inverse(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub const fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }

    pub const fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

impl TileChar for Direction {
    fn to_char(&self) -> char {
        match self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
        }
    }
}
