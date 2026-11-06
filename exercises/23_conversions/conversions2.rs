// A trait `From` é usada para converter um valor em outro. Quando `From` é
// implementada, Rust também fornece automaticamente a conversão correspondente
// com `Into`.
// Documentação: https://doc.rust-lang.org/std/convert/trait.From.html
//
// Representar unidades de medida com tipos separados é uma prática comum,
// pois evita misturar acidentalmente valores de unidades diferentes.

struct Celsius(f64);

struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    // TODO: Converta Celsius para Fahrenheit. Não se preocupe com a precisão
    // de ponto flutuante. Fórmula: F = C * 1,8 + 32
}

impl From<Fahrenheit> for Celsius {
    // TODO: Converta Fahrenheit para Celsius.
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: [(f64, f64); 6] = [
        (-50.0, -58.0),
        (0.0, 32.0),
        (20.0, 68.0),
        (100.0, 212.0),
        (400.0, 752.0),
        (1000.0, 1832.0),
    ];

    #[test]
    fn celsius_to_fahrenheit() {
        for (celsius, fahrenheit) in CASES {
            let Fahrenheit(actual) = Celsius(celsius).into();
            assert_eq!(actual.round(), fahrenheit);
        }
    }

    #[test]
    fn fahrenheit_to_celsius() {
        for (celsius, fahrenheit) in CASES {
            let Celsius(actual) = Fahrenheit(fahrenheit).into();
            assert_eq!(actual.round(), celsius);
        }
    }
}
