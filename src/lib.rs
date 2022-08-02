use std::ops::{Add, Sub, AddAssign};

#[cfg(test)]
mod tests {
    use crate::FenwickTree;

    #[test]
    fn it_works() {
		let size = 100;
        let mut tree = FenwickTree::new(size);
		for i in 0..size {
			tree.add(i, i)
		}
		let mut sum = 0;
		for i in 0..size {
			sum += i;
			assert_eq!(tree.prefix_sum(i), sum);
		}
		for i in 0..size-1 {
			assert_eq!(tree.range_sum(i, i+1), i+1)
		}
    }

	#[test]
	fn check_add(){
		let mut tree =  FenwickTree::new(10);
		tree.add(3, 100)
	}
}

struct FenwickTree<A> {
    vals: Vec<A>,
    size: usize,
}

impl <A:Default + Clone> FenwickTree<A> {

	fn new(size: usize) -> FenwickTree<A> {
		let mut v = Vec::with_capacity(size);
		v.resize(size, Default::default());
		FenwickTree { vals: v, size }
	}
}

impl<A: Clone + AddAssign> FenwickTree<A> {
    // Least Significant Bit of i having a value of 1
    // #define LSB(i) ((i) & -(i))
    fn lsb(i: i32) -> usize {
        (i & (-i)) as usize
    }

    // Returns the sum of the first i elements (indices 0 to i)
    // Equivalent to range_sum(0, i)
    fn prefix_sum(&self, i: usize) -> A {
        let mut sum = self.vals[0].clone();
        let mut index = i;
        while index != 0 {
            sum += self.vals[index].clone();
            index -= Self::lsb(index as i32)
        }
        sum
    }

    // Add delta to element with index i (zero-based)
    fn add(&mut self, i: usize, delta: A) {
        if i == 0 {
            self.vals[0] += delta;
            return;
        }
        let mut i = i;
        while i < self.size {
            self.vals[i] += delta.clone();
            i += Self::lsb(i as i32)
        }
    }

	fn range_sum(&self, from: usize, to: usize) -> A where A:Sub<Output = A> {
		let (a,b) = if from < to {(from, to)} else {(to, from)};
		self.prefix_sum(b) - self.prefix_sum(a)
	} 
}
