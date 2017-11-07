#[cfg(test)]

use jaro::jaro;
use test::Bencher;

#[bench]
fn bench(b: &mut Bencher) {
    b.iter(|| jaro("Environment", "Envronment"));
}