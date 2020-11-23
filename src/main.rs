#![feature(test)]

extern crate test;

static N: usize = 1000;
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_use_iterator(b: &mut Bencher) {
        b.iter(|| {
            let a: Vec<usize> = (1..).take(N).collect();
        });
    }

    #[bench]
    fn bench_use_for(b: &mut Bencher) {
        b.iter(|| {
            let mut a: Vec<usize> = vec![0; N];
            for i in 0..N {
                a[i] = i + 1;
            }
        });
    }

    #[bench]
    fn bench_use_while(b: &mut Bencher) {
        b.iter(|| {
            let mut index_a: usize = 0;
            let mut cache: Vec<usize> = vec![0; N];
            while index_a < N {
                index_a += 1;
                cache[index_a - 1] = index_a;
            }
        });
    }
}
