use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    if p == 1 {
        return 0;
    }

    let modulo = p as u128;
    let mut exp = a;
    let mut base = (g % p) as u128;
    let mut result = 1;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulo;
        }

        exp /= 2;
        base = (base * base) % modulo;
    }

    result as u64
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    public_key(p, b_pub, a)
}
