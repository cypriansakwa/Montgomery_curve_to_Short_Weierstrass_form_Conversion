extern crate num_integer;
extern crate num_traits;

use num_traits::{Zero}; // Import `Zero` for `is_zero`
use std::error::Error; // Import the Error trait

fn modular_inverse(a: i64, p: i64) -> Option<i64> {
    let (mut t, mut new_t) = (0, 1);
    let (mut r, mut new_r) = (p, a);

    while new_r != 0 {
        let quotient = r / new_r;
        t = t - quotient * new_t;
        r = r - quotient * new_r;
        std::mem::swap(&mut t, &mut new_t);
        std::mem::swap(&mut r, &mut new_r);
    }

    if r > 1 {
        None
    } else if t < 0 {
        Some(t + p)
    } else {
        Some(t)
    }
}

fn montgomery_to_weierstrass(a: i64, b: i64, p: i64) -> Result<(i64, i64), Box<dyn Error>> {
    // Ensure B is not zero for the Montgomery curve
    if b.is_zero() {
        return Err("Parameter 'B' cannot be zero for a valid Montgomery curve.".into());
    }

    // Compute modular inverse of 3*B^2 and 27*B^3
    let b_squared = (b * b) % p;
    let b_cubed = (b * b_squared) % p;
    let inv_3_b2 = modular_inverse(3 * b_squared, p).ok_or("No inverse for 3*B^2 modulo p")?;
    let inv_27_b3 = modular_inverse(27 * b_cubed, p).ok_or("No inverse for 27*B^3 modulo p")?;

    // Calculate Weierstrass parameters 'a' and 'b'
    let weierstrass_a = ((3 - a.pow(2)) * inv_3_b2) % p;
    let weierstrass_b = ((2 * a.pow(3) - (9 % p) * a) * inv_27_b3) % p;

    // Adjust for any negative results in modulo p
    let weierstrass_a = if weierstrass_a < 0 { weierstrass_a + p } else { weierstrass_a };
    let weierstrass_b = if weierstrass_b < 0 { weierstrass_b + p } else { weierstrass_b };

    Ok((weierstrass_a, weierstrass_b))
}

fn main() {
    let a = 6; // Example A parameter of the Montgomery curve
    let b = 7; // Example B parameter of the Montgomery curve
    let p = 13; // Prime for field F_p

    match montgomery_to_weierstrass(a, b, p) {
        Ok((weierstrass_a, weierstrass_b)) => {
            println!("Weierstrass parameters: a = {}, b = {}", weierstrass_a, weierstrass_b);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
