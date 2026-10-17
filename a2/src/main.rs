use std::io;

fn main() {
    let mut input = String::new();

    println!("masukka suhu celcius:");
    io::stdin()
        .read_line(&mut input)
        .expect("gagal membaca input");

        let celsius: f64 = input
            .trim()
            .parse()
            .expect("Masukkan angka yang valid");

        println!("Hasil: {:.1}°F", celsius * 9.0 / 5.0 + 32.0);
}