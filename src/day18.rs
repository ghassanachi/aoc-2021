use aoc_runner_derive::{aoc, aoc_generator};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct NodeContent {
    parent: Weak<RefCell<Self>>,
    left: Option<SnailNumber>,
    right: Option<SnailNumber>,
    data: Option<u64>,
}

impl NodeContent {
    fn new(data: Option<u64>) -> Self {
        NodeContent {
            parent: Weak::new(),
            left: None,
            right: None,
            data,
        }
    }
}

pub type SnailNumber = Rc<RefCell<NodeContent>>;

trait Node
where
    Self: Sized,
{
    fn new_node(data: Option<u64>) -> Self;
    fn from_str(input: &str) -> Self;
    fn insert_left(&mut self, data: Option<u64>);
    fn insert_right(&mut self, data: Option<u64>);
    fn left(&self) -> Option<Self>;
    fn right(&self) -> Option<Self>;
    fn parent(&self) -> Option<Self>;
    fn siblings(&self, other: &Self) -> bool;
    fn set_data(&mut self, data: u64);
    fn preorder(&self) -> Vec<(Weak<RefCell<NodeContent>>, usize)>;
    fn reduce(&mut self) -> bool;
    fn is_leaf(&self) -> bool;
    fn add(self, other: Self) -> Self;
    fn to_string(&self) -> String;
    fn magnitude(&self) -> u64;
}

impl Node for SnailNumber {
    fn new_node(data: Option<u64>) -> Self {
        Rc::new(RefCell::new(NodeContent::new(data)))
    }

    fn from_str(input: &str) -> Self {
        let root = Self::new_node(None);
        let mut cursor = root.clone();
        let mut char_iter = input.chars();
        while let Some(c) = char_iter.next() {
            match c {
                '[' => {
                    cursor.insert_left(None);
                    cursor = cursor.left().unwrap();
                }
                ']' => {
                    cursor = cursor.parent().unwrap();
                }
                ',' => {
                    cursor = cursor.parent().unwrap();
                    cursor.insert_right(None);
                    cursor = cursor.right().unwrap();
                }
                val => {
                    let val: u64 = val.to_string().parse().expect("invalid input");
                    cursor.set_data(val)
                }
            }
        }
        root
    }

    fn insert_left(&mut self, data: Option<u64>) {
        let child = Self::new_node(data);
        child.borrow_mut().parent = Rc::downgrade(self);
        self.borrow_mut().left = Some(child);
    }

    fn insert_right(&mut self, data: Option<u64>) {
        let child = Self::new_node(data);
        child.borrow_mut().parent = Rc::downgrade(self);
        self.borrow_mut().right = Some(child);
    }

    fn left(&self) -> Option<Self> {
        self.borrow().left.as_ref().map(|n| n.clone())
    }

    fn right(&self) -> Option<Self> {
        self.borrow().right.as_ref().map(|n| n.clone())
    }

    fn parent(&self) -> Option<Self> {
        self.borrow().parent.upgrade()
    }

    fn siblings(&self, other: &Self) -> bool {
        if let (Some(s), Some(o)) = (self.parent(), other.parent()) {
            return Rc::ptr_eq(&s, &o);
        }
        false
    }

    fn set_data(&mut self, data: u64) {
        self.borrow_mut().data = Some(data);
    }

    fn preorder(&self) -> Vec<(Weak<RefCell<NodeContent>>, usize)> {
        fn helper(
            node: &Option<SnailNumber>,
            depth: usize,
            result: &mut Vec<(Weak<RefCell<NodeContent>>, usize)>,
        ) {
            if let Some(n) = node {
                helper(&n.borrow().left, depth + 1, result);
                if n.borrow().data.is_some() {
                    result.push((Rc::downgrade(n), depth));
                }
                helper(&n.borrow().right, depth + 1, result);
            }
        }
        let mut out = Vec::new();
        helper(&Some(self.clone()), 0, &mut out);
        out
    }

    fn reduce(&mut self) -> bool {
        let mut nodes = self.preorder();
        // Handle explode
        for i in 0..nodes.len() - 1 {
            let (node, depth) = (nodes[i].0.upgrade().unwrap(), nodes[i].1);
            if depth <= 4 {
                continue;
            }
            let right_pair = nodes[i + 1].0.upgrade().unwrap();
            if !node.siblings(&right_pair) {
                continue;
            }
            let parent = node.parent().unwrap();
            drop(node);
            drop(right_pair);
            parent.borrow_mut().data = Some(0);
            let left = parent.borrow_mut().left.take();
            let right = parent.borrow_mut().right.take();
            let (left, right) = (left.unwrap(), right.unwrap());
            if i > 0 {
                let next_left = nodes[i - 1].0.upgrade().unwrap();
                let left_val = left.borrow().data.unwrap();
                let next_left_val = next_left.borrow().data.unwrap();
                next_left.borrow_mut().data = Some(left_val + next_left_val);
            }
            if i + 2 < nodes.len() {
                let next_right = nodes[i + 2].0.upgrade().unwrap();
                let right_val = right.borrow().data.unwrap();
                let next_right_val = next_right.borrow().data.unwrap();
                next_right.borrow_mut().data = Some(next_right_val + right_val);
            }
            return true;
        }

        // Hanle Split
        for (node, _) in &mut nodes {
            let mut node = node.upgrade().unwrap();
            assert!(node.borrow().data.is_some());
            let val = node.borrow().data.unwrap();
            if val <= 9 {
                continue;
            }
            node.borrow_mut().data = None;
            node.insert_left(Some(val / 2));
            node.insert_right(Some((val / 2) + (val % 2)));
            return true;
        }
        false
    }

    fn is_leaf(&self) -> bool {
        self.borrow().data.is_some()
    }

    fn add(self, other: Self) -> Self {
        let new_root = Self::new_node(None);
        self.borrow_mut().parent = Rc::downgrade(&new_root);
        other.borrow_mut().parent = Rc::downgrade(&new_root);
        new_root.borrow_mut().left = Some(self);
        new_root.borrow_mut().right = Some(other);
        new_root
    }

    fn to_string(&self) -> String {
        fn helper(node: &Option<SnailNumber>, out: &mut String) {
            if let Some(n) = node {
                if n.borrow().left.is_some() {
                    out.push('[');
                    helper(&n.borrow().left, out);
                    out.push(',');
                }
                if n.is_leaf() {
                    out.push_str(&n.borrow().data.unwrap().to_string())
                }
                if n.borrow().left.is_some() {
                    helper(&n.borrow().right, out);
                    out.push(']');
                }
            }
        }
        let mut out = String::new();
        helper(&Some(self.clone()), &mut out);
        out
    }

    fn magnitude(&self) -> u64 {
        if self.is_leaf() {
            return self.borrow().data.unwrap();
        }
        3 * Self::magnitude(self.borrow().left.as_ref().unwrap())
            + 2 * Self::magnitude(self.borrow().right.as_ref().unwrap())
    }
}

#[aoc_generator(day18, part1)]
pub fn input_generator(input: &str) -> Vec<SnailNumber> {
    input.lines().fold(Vec::new(), |mut acc, el| {
        acc.push(SnailNumber::from_str(el));
        acc
    })
}

#[aoc(day18, part1, day18_1)]
pub fn part1(input: &[SnailNumber]) -> u64 {
    let mut start_node = input[0].clone();
    for i in 1..input.len() {
        let other = input[i].clone();
        start_node = start_node.add(other);
        while start_node.reduce() {}
    }
    start_node.magnitude()
}

#[aoc(day18, part2, day18_2)]
pub fn part2(input: &str) -> u64 {
    let mut max = 0;
    let numbers: Vec<_> = input.lines().collect();
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }
            let i_num = SnailNumber::from_str(numbers[i]);
            let j_num = SnailNumber::from_str(numbers[j]);
            let mut node = i_num.add(j_num);
            while node.reduce() {}
            max = max.max(node.magnitude());
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn test_part_1() {
        let input = input_generator(SAMPLE);
        assert_eq!(part1(&input), 4140)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&SAMPLE), 3993)
    }
}
