use nalgebra::Vector2;

pub type Position = Vector2<i64>;

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

impl std::ops::AddAssign<Movement> for Position {
    fn add_assign(&mut self, rhs: Movement) {
        match rhs.direction {
            Direction::Up => {
                self.y = self.y.wrapping_sub(rhs.distance as i64);
            }
            Direction::Down => {
                self.y = self.y.wrapping_add(rhs.distance as i64);
            }
            Direction::Left => {
                self.x = self.x.wrapping_sub(rhs.distance as i64);
            }
            Direction::Right => {
                self.x = self.x.wrapping_add(rhs.distance as i64);
            }
        }
    }
}

impl std::ops::Add<Movement> for Position {
    type Output = Self;

    fn add(mut self, rhs: Movement) -> Self::Output {
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

impl std::ops::SubAssign<Movement> for Position {
    fn sub_assign(&mut self, rhs: Movement) {
        match rhs.direction {
            Direction::Up => {
                self.y = self.y.wrapping_add(rhs.distance as i64);
            }
            Direction::Down => {
                self.y = self.y.wrapping_sub(rhs.distance as i64);
            }
            Direction::Left => {
                self.x = self.x.wrapping_add(rhs.distance as i64);
            }
            Direction::Right => {
                self.x = self.x.wrapping_sub(rhs.distance as i64);
            }
        }
    }
}

impl std::ops::Sub<Movement> for Position {
    type Output = Self;

    fn sub(mut self, rhs: Movement) -> Self::Output {
        self -= rhs;
        self
    }
}

pub trait TileChar {
    fn to_char(&self) -> char;
}

impl TileChar for char {
    fn to_char(&self) -> char {
        *self
    }
}

pub struct Grid<T> {
    width: i64,
    height: i64,
    entries: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, entries: Vec<T>) -> Self {
        assert_eq!(width * height, entries.len());

        Self {
            width: width as i64,
            height: height as i64,
            entries,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn width(&self) -> i64 {
        self.width
    }

    pub fn height(&self) -> i64 {
        self.height
    }

    pub fn contains(&self, position: &Position) -> bool {
        (0..self.width).contains(&position.x) && (0..self.height).contains(&position.y)
    }

    pub fn index(&self, position: &Position) -> Option<usize> {
        if self.contains(position) {
            Some((position.y * self.width + position.x) as usize)
        } else {
            None
        }
    }

    pub fn position(&self, index: usize) -> Position {
        Position::new(index as i64 % self.width, index as i64 / self.width)
    }

    pub fn get(&self, position: &Position) -> Option<&T> {
        self.index(position).and_then(|idx| self.entries.get(idx))
    }

    pub fn put(&mut self, position: &Position, tile: T) {
        if let Some(index) = self.index(position) {
            self.entries[index] = tile;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.entries.iter()
    }
}

impl<T> Grid<T>
where
    T: TileChar,
{
    pub fn to_char_grid(&self) -> Grid<char> {
        Grid {
            width: self.width,
            height: self.height,
            entries: self.entries.iter().map(|t| t.to_char()).collect(),
        }
    }
}

impl<T> std::ops::Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl<T> Clone for Grid<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            entries: self.entries.clone(),
        }
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
            if idx > 0 && idx % self.width as usize == 0 {
                writeln!(f, "│")?;
                write!(f, "│")?;
            }

            write!(f, "{}", tile.to_char())?;
        }

        writeln!(f, "│")?;
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
    pub const ALL: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];

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

    pub const fn orientation(&self) -> Orientation {
        match self {
            Self::Up => Orientation::Vertical,
            Self::Down => Orientation::Vertical,
            Self::Left => Orientation::Horizontal,
            Self::Right => Orientation::Horizontal,
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

impl std::ops::Mul<usize> for Direction {
    type Output = Movement;

    fn mul(self, rhs: usize) -> Self::Output {
        Self::Output {
            direction: self,
            distance: rhs,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Movement {
    pub direction: Direction,
    pub distance: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Orientation {
    Horizontal,
    Vertical,
}
