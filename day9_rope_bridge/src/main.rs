use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

struct Grid<'a> {
    tail_visited: HashSet<Point>,
    rope: Vec<Point>,
    move_defs: HashMap<&'a str, (i32, i32)>,
}

impl Grid<'_> {
    fn new(knots_count: usize) -> Self {
        let mut rope: Vec<Point> = Vec::with_capacity(knots_count);
        for _ in 0..knots_count {
            rope.push(Point { x: 0, y: 0 })
        }

        let mut move_defs: HashMap<&str, (i32, i32)> = HashMap::new();
        move_defs.insert("R", (1, 0));
        move_defs.insert("L", (-1, 0));
        move_defs.insert("U", (0, 1));
        move_defs.insert("D", (0, -1));

        Self {
            tail_visited: HashSet::new(),
            rope,
            move_defs,
        }
    }

    // translates move's direction to a (x, y) tuple to be added to a Point
    // e.g. "D" -> (0, -1))
    fn translate_move(&self, mov: &str) -> (i32, i32) {
        *self.move_defs.get(mov).unwrap()
    }

    // Basically calculates the hypotenuse according to the Pythagorean theorem
    fn should_tail_move(&self, first_index: usize, second_index: usize) -> bool {
        let first = self.rope.get(first_index).unwrap();
        let second = self.rope.get(second_index).unwrap();

        let sum_sides =
            ((first.x - second.x) as f64).powf(2.0) + ((first.y - second.y) as f64).powf(2.0);

        sum_sides.sqrt() > 2.0_f64.sqrt()
    }

    fn tail_visited(&mut self, p: Point) {
        self.tail_visited.insert(p);
    }

    fn get_point(&self, point_index: usize) -> Point {
        *self.rope.get(point_index).unwrap()
    }

    fn move_point_by(&mut self, point_index: usize, amount: (i32, i32)) {
        let point = self.rope.get_mut(point_index).unwrap();
        point.x += amount.0;
        point.y += amount.1;
    }

    fn calc_tail_move(&self, head_index: usize, tail_index: usize) -> (i32, i32) {
        let head = self.rope.get(head_index).unwrap();
        let tail = self.rope.get(tail_index).unwrap();

        let mut x = head.x - tail.x;
        let mut y = head.y - tail.y;

        if x != 0 {
            x /= x.abs();
        };
        if y != 0 {
            y /= y.abs();
        };

        (x, y)
    }
}

fn main() {
    let input = get_input("src/input.txt");

    let moves = input
        .lines()
        .map(|l| {
            let mov = l.split_whitespace().collect::<Vec<&str>>();
            (
                mov.first().unwrap().to_owned(),
                mov.last().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(&str, i32)>>();

    let mut grid = Grid::new(2);

    for (dir, count) in moves.iter() {
        let head_index = 0;
        let tail_index = 1;
        for _ in 0..*count {
            // moving head
            let move_amount = grid.translate_move(dir);
            grid.move_point_by(head_index, move_amount);

            if grid.should_tail_move(head_index, tail_index) {
                // if we get to this point, tail must move to catch up with the head

                let t_move = grid.calc_tail_move(head_index, tail_index);
                grid.move_point_by(tail_index, t_move);
            }

            let tail = grid.get_point(tail_index);
            grid.tail_visited(tail);
        }
    }

    println!("2 knot rope's tail visited: {:?}", grid.tail_visited.len());

    // PART 2
    let mut grid = Grid::new(10);

    for (dir, count) in moves.iter() {
        for _ in 0..*count {
            // moving head
            let head_move_amount = grid.translate_move(dir);
            grid.move_point_by(0, head_move_amount);

            // comparing and moving knots in groups of two
            for i in 0..9 {
                if grid.should_tail_move(i, i + 1) {
                    // if we get to this point, trailing knot must move to catch up with the leading one

                    let t_move = grid.calc_tail_move(i, i + 1);
                    grid.move_point_by(i + 1, t_move);
                }
            }

            let tail = grid.get_point(9);
            grid.tail_visited(tail);
        }
    }
    println!("10 knot rope's tail visited: {:?}", grid.tail_visited.len());
}

fn get_input(path: &str) -> String {
    let input = fs::read_to_string(path);
    match input {
        Ok(input) => input,
        Err(err) => panic!("err reading input: {}", err),
    }
}
