use rsa::get_rand_prime;

fn main() {
    for _ in 0..10 {
        println!("{:#?}", get_rand_prime(1024));
    }
}
