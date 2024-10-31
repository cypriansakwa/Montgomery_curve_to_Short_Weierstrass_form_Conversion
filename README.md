## Overview 
This Rust project provides functionality to convert a Montgomery curve into its equivalent Short Weierstrass form. It utilizes the mathematical properties of elliptic curves and modular arithmetic to achieve this conversion.
## Installation

To get started, ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine. You can then clone the repository and build the project:

  >```
  > git clone https://github.com/cypriansakwa/Montgomery_curve_to_Short_Weierstrass_form_Conversion.git
   >cd Montgomery_curve_to_Short_Weierstrass_form_Conversion
   >cargo build

## Usage
Once the project is built, you can run the program using:
>```
>cargo run
You can modify the parameters `A`, `B`, and `p` in the main function to test different Montgomery curves.
## Example
To convert a Montgomery curve defined by parameters $A=6$, $B=7$, and a prime $p=11$, the program will output the corresponding Short Weierstrass form parameters.
## Algorithm
The conversion from Montgomery form to Short Weierstrass form is based on the following formulas:
  - Weierstrass Parameters:
    - $a = \frac{3 - A^2}{3 \cdot B^2}$
    - $b = \frac{2 \cdot A^3 - (9 \mod p) \cdot A}{(27 \mod p) \cdot B^3}$.
  - Modular Inverse: The program calculates the modular inverse of required terms to ensure correct arithmetic in the finite field defined by the prime $p$.
## Important Notes
- The input parameters must be chosen such that $B\not=0$ to ensure the validity of the Montgomery curve.
- Ensure that the parameters are compatible with the modular arithmetic required for elliptic curves.
