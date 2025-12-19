# Advent of Code 2025 - Rust

Lösningar för Advent of Code 2025 i Rust.

## Struktur

```
aoc25-rust/
├── src/
│   ├── main.rs          # Main entry point
│   ├── utils.rs         # Återanvändbara hjälpfunktioner
│   └── days/
│       ├── mod.rs       # Module declarations
│       ├── day01.rs     # Dag 1 lösning
│       ├── day02.rs     # Dag 2 lösning
│       └── ...
├── inputs/
│   ├── day01.txt        # Riktig input för dag 1
│   ├── day01_example.txt # Exempel input för dag 1
│   └── ...
└── Cargo.toml
```

## Användning

### Kör en dag med riktig input
```bash
cargo run <dag>
```

Exempel:
```bash
cargo run 1
```

### Kör en dag med exempel input
```bash
cargo run <dag> --example
```

Exempel:
```bash
cargo run 1 --example
```

### Kör tester
```bash
# Kör alla tester
cargo test

# Kör tester för en specifik dag
cargo test day01
```

### Kör i release mode (snabbare)
```bash
cargo run --release 1
```

## Lägga till en ny dag

1. Skapa `src/days/dayXX.rs` med mallen:
```rust
pub fn solve(input: &str) {
    println!("=== Day XX ===");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    // Din lösning här
    0
}

fn part2(input: &str) -> i32 {
    // Din lösning här
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "exempel data";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
```

2. Lägg till modulen i `src/days/mod.rs`:
```rust
pub mod dayXX;
```

3. Lägg till case i `src/main.rs`:
```rust
XX => days::dayXX::solve(&input),
```

4. Skapa input-filer:
   - `inputs/dayXX.txt` - din riktiga input
   - `inputs/dayXX_example.txt` - exempel input

## Tips

- Börja alltid med att köra med `--example` för att testa mot känt resultat
- Skriv tester för varje del innan du kör mot riktig input
- Använd `utils.rs` för återanvändbar kod mellan dagar
- Kompilera i release mode för bättre prestanda på tunga beräkningar
