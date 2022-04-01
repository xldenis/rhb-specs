// SPEC LINES 6 + 8 + 6
use crate::prelude::*;

#[derive(Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    // Require that points are bounded by +- 10_000 in all dimensions for overflow reasons
    #[requires(-10000 <= @(self.x) && @(self.x) <= 10000)]
    #[requires(-10000 <= @(self.y) && @(self.y) <= 10000)]
    #[requires(-10000 <= @(p.0) && @(p.0) <= 10000)]
    #[requires(-10000 <= @(p.1) && @(p.1) <= 10000)]
    // The result is just the sum of the inputs
    #[ensures(@(result.x) === @(self.x) + @(p.0))]
    #[ensures(@(result.y) === @(self.y) + @(p.1))]
    fn mov(&self, p: &(isize, isize)) -> Self {
        Self { x: (self.x + p.0), y: (self.y + p.1) }
    }
}

struct Board {
    size: usize,
    field: Vec<Vec<usize>>,
}

impl Board {
    #[predicate]
    fn wf(self) -> bool {
        // A board is well formed if:
        //  1. it at most 1_000 x 1_000 (overflow)
        //  2. self.field holds a square vector of vectors of size `self.size`
        pearlite! {
            @(self.size) <= 1_000 &&
            (@(self.field)).len() === @self.size &&
            forall<i : Int> 0 <= i && i < @self.size ==> (@(@(self.field))[i]).len() === @self.size
        }
    }

    // Produces a well-formed board of size `size`
    #[requires(@size <= 1000)]
    #[ensures(result.size === size)]
    #[ensures(result.wf())]
    fn new(size: usize) -> Self {
        let mut rows: Vec<Vec<_>> = Vec::with_capacity(size);

        let mut i = 0;
        #[invariant(i_size, i <= size)]
        // All the created rows are of size `size`
        #[invariant(rows,
            forall<j : Int> 0 <= j && j < @i ==> (@((@rows)[j])).len() === @size)]
        #[invariant(row_len, (@rows).len() === @i )]
        while i < size {
            rows.push(vec::from_elem(0, size));
            i += 1;
        }

        Self { size, field: rows }
    }

    // given a well formed board, find if a point is available in the board: it must be inbounds and also unset
    #[requires(self.wf())]
    #[ensures(result ==> self.in_bounds(p))]
    fn available(&self, p: Point) -> bool {
        0 <= p.x
            && (p.x as usize) < self.size
            && 0 <= p.y
            && (p.y as usize) < self.size
            && self.field[p.x as usize][p.y as usize] == 0
    }

    // Check if a point is within the bounds of teh board.
    #[predicate]
    fn in_bounds(self, p: Point) -> bool {
        pearlite! {
            0 <= @(p.x) && @(p.x)< @(self.size) && 0 <= @(p.y) && @(p.y) < @(self.size)
        }
    }

    // calculate the number of possible moves
    #[requires(self.wf())]
    #[requires(self.in_bounds(p))]
    fn count_degree(&self, p: Point) -> usize {
        let mut count = 0;

        let mut i = 0;
        #[invariant(count, count <= i)]
        while i < moves().len() {
            let next = p.mov(&moves()[i]);
            if self.available(next) {
                count += 1;
            }
            i += 1;
        }
        count
    }
    // Given a valid point and board, set produces a well-formed board.
    #[requires(self.wf())]
    #[requires(self.in_bounds(p))]
    #[ensures((^self).wf())]
    #[ensures((^self).size === (*self).size)]
    fn set(&mut self, p: Point, v: usize) {
        self.field[p.x as usize][p.y as usize] = v
    }
}

#[trusted]
#[ensures((@result).len() === 8)]
#[ensures(forall<i : Int> 0 <= i && i < 8 ==> -2 <= @((@result)[i].0) && @((@result)[i].0) <= 2 && -2 <= @((@result)[i].1) &&@((@result)[i].1) <= 2)]
fn moves() -> Vec<(isize, isize)> {
    let mut v = Vec::new();
    v.push((2, 1));
    v.push((1, 2));
    v.push((-1, 2));
    v.push((-2, 1));
    v.push((-2, -1));
    v.push((-1, -2));
    v.push((1, -2));
    v.push((2, -1));

    v
}

fn min(v: &Vec<(usize, Point)>) -> Option<&(usize, Point)> {
    let mut i = 0;
    let mut min = None;
    while i < v.len() {
        match min {
            None => min = Some(&v[i]),
            Some(m) => {
                if v[i].0 < m.0 {
                    min = Some(&v[i])
                }
            }
        };
        i += 1;
    }
    min
}

// A simple lemma we need to convince why3 that 1_000^2 is within the bounds of a u64
#[logic]
#[requires(@a <= 1_000)]
#[ensures(@a * @a <= 1_000_000)]
fn dumb_nonlinear_arith(a: usize) {}

// require a board size and a valid position in the board.
#[requires(0 < @size && @size <= 1000)]
#[requires(x < size)]
#[requires(y < size)]
fn knights_tour(size: usize, x: usize, y: usize) -> Option<Board> {
    let mut board = Board::new(size);
    let mut p = Point { x: x as isize, y: y as isize };
    let mut step = 1;

    board.set(p, step);
    step += 1;

    // call the lemma to convince why3 our board size won't overflow
    proof_assert! {{ dumb_nonlinear_arith(size); true }}
    // Invariants just state the board keeps it size, stys valid and the point we're currently at is within bounds.
    #[invariant(b, board.size === size)]
    #[invariant(b, board.wf())]
    #[invariant(p, board.in_bounds(p))]
    // rather annoyingly z3 gets stuck proving size * size is inbounds, seemingly
    // due to a why3 bug / limitation in mlcfg
    while step <= (size * size) {
        // choose next square by Warnsdorf's rule
        let mut candidates = Vec::new();
        let mut i = 0;
        while i < moves().len() {
            proof_assert! { board.in_bounds(p) };
            let adj = p.mov(&moves()[i]);
            if board.available(adj) {
                let degree = board.count_degree(adj);
                candidates.push((degree, adj));
            }
        }
        match min(&candidates) {
            Some(&(_, adj)) => p = adj,
            None => return None,
        };
        board.set(p, step);
        step += 1;
    }
    Some(board)
}

