# Materi Belajar Rust dalam 24 Jam untuk Vibe Coding

**Panduan teori, praktik, latihan, dan proyek akhir**

Materi ini dikembangkan dari roadmap *Rust dalam 24 Jam — Peta Belajar buat Vibe Coding*. Fokusnya bukan menghafal syntax. Fokusnya adalah memahami cara Rust mengelola data, membaca pesan compiler, dan menilai kode Rust yang dibuat dengan bantuan AI.

> **Catatan waktu:** 24 jam adalah waktu belajar efektif, bukan 24 jam tanpa henti. Gunakan 4–6 jam per hari selama beberapa hari. Jika satu konsep belum terasa jelas, lanjutkan setelah mengerjakan praktiknya; jangan memaksa mengejar jadwal.

## Hasil belajar

Setelah menyelesaikan materi, kamu diharapkan dapat:

1. Menjelaskan kapan Rust cocok dipakai dan bagaimana alur program Rust dikompilasi.
2. Membaca syntax dasar, fungsi, tipe data, percabangan, dan perulangan.
3. Menjelaskan ownership, move, borrowing, reference, slice, dan lifetime dengan contoh.
4. Memodelkan data menggunakan struct dan enum serta menangani hasil menggunakan Option dan Result.
5. Membaca trait, generic, closure, dan iterator dalam kode sehari-hari.
6. Menggunakan koleksi, module, Cargo, dependency, dan test.
7. Mengenali kegunaan smart pointer, thread, channel, async, dan unsafe.
8. Membuat serta meninjau CLI log analyzer sederhana.
9. Meminta bantuan AI dengan instruksi yang membuat kode lebih mudah diperiksa.

## Cara menggunakan panduan

Untuk setiap blok, ikuti urutan ini:

1. Baca penjelasan konsep sebelum melihat contoh.
2. Ketik kode sendiri, lalu jalankan.
3. Ubah satu hal pada kode dan amati hasilnya.
4. Jika muncul error, baca pesan compiler sampai ke bagian yang menjelaskan penyebab dan file/barisnya.
5. Sebelum meminta AI memperbaiki kode, tulis dugaanmu sendiri: apa yang dipinjam, siapa pemilik data, dan nilai apa yang dikembalikan.
6. Tutup sesi dengan menjelaskan ulang konsep menggunakan kata-katamu sendiri.

Rust menggunakan compiler yang ketat. Tujuannya bukan membuat pemrograman terasa sulit. Banyak kesalahan terkait tipe, penggunaan data setelah dipindahkan, dan akses data bersamaan dapat diketahui sebelum program dijalankan. Compiler tetap tidak bisa menjamin bahwa logika programmu benar; test dan review tetap diperlukan.

## Peta belajar 24 jam

| Blok | Waktu | Pokok bahasan | Hasil praktik |
|---|---:|---|---|
| 1 | 0–2 jam | Pengenalan Rust dan setup | Menjalankan program pertama dan memilih masalah yang cocok untuk Rust |
| 2 | 2–5 jam | Syntax dan tipe dasar | FizzBuzz, konversi suhu, dan program input sederhana |
| 3 | 5–9 jam | Ownership, borrowing, lifetime | Memperbaiki contoh yang ditolak borrow checker |
| 4 | 9–12 jam | Struct, enum, match, error handling | Memodelkan log dan mengembalikan Result |
| 5 | 12–15 jam | Trait, generic, closure, iterator | Mengolah data menggunakan iterator chain |
| 6 | 15–18 jam | Koleksi, module, Cargo, testing | Proyek beberapa file dengan test |
| 7 | 18–21 jam | Smart pointer, concurrency, async, unsafe | Memahami batas aman dan pola kerja konkuren |
| 8 | 21–24 jam | Proyek akhir dan workflow AI | Menyusun CLI log analyzer |

---

# Blok 1 — Mengenal Rust dan menyiapkan lingkungan

**Waktu:** 0–2 jam  
**Tujuan:** memahami kegunaan Rust, mengenali manfaat dan biaya belajarnya, lalu menjalankan program pertama.

## 1.1 Apa itu Rust?

Rust adalah bahasa yang dikompilasi dan memiliki tipe statis. Artinya, kode sumber diubah menjadi program yang dapat dijalankan, dan banyak informasi tipe diperiksa sebelum program berjalan.

Rust tidak memakai garbage collector untuk mengelola memori. Sebagai gantinya, compiler memeriksa aturan ownership dan borrowing. Dengan aturan ini, Rust dapat mencegah banyak kesalahan memori dalam kode aman (*safe Rust*), seperti memakai data setelah dilepas atau memiliki beberapa akses tulis yang saling bertabrakan.

Pemeriksaan ini punya biaya belajar: compiler bisa menolak kode yang secara sekilas terlihat masuk akal. Pesan tersebut sering kali membantu menunjukkan asumsi yang belum jelas tentang siapa yang memiliki data dan siapa yang meminjamnya.

Rust tidak otomatis mencegah semua bug. Kesalahan logika, hasil yang salah, validasi bisnis yang kurang, dan masalah yang dibuat di bagian unsafe tetap perlu ditangani dengan desain, test, dan review.

## 1.2 Gambaran proses build

Saat menjalankan proyek dengan Cargo, alurnya kira-kira seperti ini:

1. Cargo membaca konfigurasi proyek dan dependency.
2. Compiler memeriksa syntax, tipe, ownership, dan aturan lain.
3. Jika pemeriksaan berhasil, kode dikompilasi menjadi binary.
4. Program berjalan saat kamu menjalankannya.

Perintah yang sering dipakai:

| Perintah | Fungsi |
|---|---|
| cargo check | Memeriksa kode dengan cepat tanpa membuat binary final |
| cargo run | Mengompilasi bila perlu, lalu menjalankan program |
| cargo build | Mengompilasi proyek |
| cargo build --release | Membuat build yang dioptimalkan untuk penggunaan rilis |
| cargo fmt | Merapikan format kode |
| cargo clippy | Menampilkan linter dan saran perbaikan |
| cargo test | Menjalankan test |

**Jangan tertukar:** cargo check membantu menemukan error kompilasi dengan cepat, tetapi belum menggantikan test. Program yang bisa dikompilasi tetap dapat menghasilkan jawaban yang keliru.

## 1.3 Kapan Rust cocok?

Rust layak dipertimbangkan ketika performa, penggunaan memori, dan keandalan program penting, misalnya untuk:

- tool CLI yang menangani banyak file atau input;
- layanan jaringan dan backend;
- komponen sistem, agent, atau program yang berjalan lama;
- parser yang memproses data dari sumber luar;
- pemrosesan paralel;
- aplikasi embedded atau WebAssembly.

Bahasa lain mungkin lebih praktis untuk script kecil, eksplorasi data, atau prototipe yang perlu berubah sangat cepat. Pilihan bahasa sebaiknya mengikuti kebutuhan produk, kemampuan tim, ekosistem, dan biaya pemeliharaan.

## 1.4 Menyiapkan Rust

Cara yang umum adalah memasang toolchain menggunakan **rustup**, lalu memakai Cargo yang disertakan bersama toolchain tersebut. Ikuti petunjuk untuk sistem operasi di [halaman instalasi Rust](https://rust-lang.org/tools/install/).

Sesudah instalasi, buka terminal baru dan periksa:

~~~text
rustc --version
cargo --version
~~~

Jika perintah tidak dikenali, tutup dan buka kembali terminal, lalu pastikan direktori toolchain Rust masuk ke PATH. Di Windows, ikuti petunjuk installer resmi; beberapa lingkungan Windows juga membutuhkan komponen build tools C++.

Buat proyek pertama:

~~~text
cargo new halo
cd halo
cargo run
~~~

Cargo membuat struktur awal seperti ini:

~~~text
halo/
├── Cargo.toml
└── src/
    └── main.rs
~~~

File Cargo.toml berisi metadata dan dependency. File src/main.rs berisi titik masuk program executable.

Isi awal main.rs biasanya menyerupai:

~~~rust
fn main() {
    println!("Hello, world!");
}
~~~

fn main() adalah fungsi yang pertama kali dijalankan untuk program CLI. println! adalah macro untuk menulis teks ke terminal. Tanda seru menandakan pemanggilan macro.

## Praktik 1 — Kenalan dengan compiler

1. Buat proyek bernama halo.
2. Ganti pesan menjadi Saya mulai belajar Rust.
3. Jalankan cargo check, lalu cargo run.
4. Ubah sementara println! menjadi prontln!.
5. Jalankan cargo check dan baca pesan error.
6. Perbaiki typo, lalu jalankan cargo fmt.

### Refleksi singkat

- File mana yang menjadi titik masuk program?
- Perintah apa yang memeriksa kode dengan cepat?
- Apakah program yang lolos cargo check pasti memiliki logika yang benar? Mengapa?

---

# Blok 2 — Syntax dan tipe dasar

**Waktu:** 2–5 jam  
**Tujuan:** membaca kode dasar, mengubah data, membuat fungsi, dan mengendalikan alur program.

## 2.1 Variabel, mutability, dan shadowing

Variabel Rust tidak dapat diubah secara default. Gunakan mut jika nilai yang sama memang perlu diubah.

~~~rust
fn main() {
    let nama = "Rani";
    let mut jumlah = 2;

    jumlah += 1;

    let jumlah = jumlah * 2; // shadowing: deklarasi binding baru
    println!("{nama} memiliki {jumlah} buku");
}
~~~

Perbedaan penting:

- let mut jumlah = 2; jumlah += 1; mengubah nilai pada binding mutable.
- let jumlah = jumlah * 2; membuat binding baru yang menutupi binding sebelumnya. Ini disebut **shadowing**.
- Shadowing dapat menghasilkan tipe baru. Mutability sendiri tidak mengubah tipe variabel.

Konstanta memakai const, harus memiliki anotasi tipe, dan nilainya diketahui saat kompilasi:

~~~rust
const BATAS_MAKSIMUM: u32 = 100;
~~~

## 2.2 Tipe data

| Kelompok | Contoh | Catatan |
|---|---|---|
| Integer bertanda | i8, i16, i32, i64, i128, isize | Dapat menyimpan nilai negatif |
| Integer tanpa tanda | u8, u16, u32, u64, u128, usize | Tidak menyimpan nilai negatif |
| Pecahan | f32, f64 | f64 umum dipakai untuk perhitungan pecahan |
| Boolean | bool | Bernilai true atau false |
| Karakter | char | Satu nilai Unicode, ditulis dengan petik tunggal |
| Tuple | (i32, f64, char) | Menggabungkan beberapa nilai dengan tipe yang dapat berbeda |
| Array | [i32; 3] | Ukurannya tetap dan semua elemen bertipe sama |
| Unit | () | Menandakan tidak ada nilai yang berguna untuk dikembalikan |

usize biasa dipakai untuk panjang dan indeks karena ukurannya mengikuti arsitektur target. Untuk ukuran dinamis, gunakan koleksi seperti Vec<T> yang akan dipelajari kemudian.

Konversi tipe tidak selalu otomatis. Gunakan metode konversi yang terlihat jelas, misalnya u32::try_from(nilai) untuk konversi yang bisa gagal, atau nilai as u32 jika kamu sudah memahami batas dan perilakunya.

## 2.3 Fungsi dan expression

Fungsi memiliki parameter dan tipe hasil yang dapat ditulis secara eksplisit:

~~~rust
fn tambah(a: i32, b: i32) -> i32 {
    a + b
}
~~~

Baris terakhir tanpa titik koma adalah nilai yang dikembalikan. Jika ditulis a + b;, ekspresi itu menjadi statement dan fungsi tidak lagi mengembalikan nilai i32.

Rust disebut berbasis expression karena banyak bagian kode menghasilkan nilai. Percabangan if dapat mengembalikan nilai:

~~~rust
fn status(nilai: u32) -> &'static str {
    let hasil = if nilai >= 70 { "lulus" } else { "perlu latihan" };
    hasil
}
~~~

Kedua cabang if harus menghasilkan tipe yang sama.

## 2.4 Percabangan dan perulangan

~~~rust
let label = if skor >= 70 { "lulus" } else { "remedial" };

for angka in 0..5 {
    println!("{angka}"); // 0, 1, 2, 3, 4
}

for angka in 0..=5 {
    println!("{angka}"); // 0 sampai 5
}

while jumlah > 0 {
    jumlah -= 1;
}

let hasil = loop {
    break 42; // break dapat mengembalikan nilai dari loop
};
~~~

Rentang 0..5 tidak menyertakan angka terakhir, sedangkan 0..=5 menyertakannya.

## 2.5 Menampilkan nilai

~~~rust
println!("Nama: {}, data: {:?}", nama, data);
~~~

- {} memakai trait Display, biasanya untuk tampilan ramah pengguna.
- {:?} memakai trait Debug, biasanya untuk pemeriksaan saat development.
- {nama} adalah format capture untuk variabel bernama.

## Praktik 2A — FizzBuzz

Buat program yang memeriksa angka dari 1 sampai dengan 30:

- kelipatan 3 dan 5: cetak FizzBuzz;
- kelipatan 3 saja: cetak Fizz;
- kelipatan 5 saja: cetak Buzz;
- selain itu: cetak angkanya.

Contoh implementasi:

~~~rust
fn main() {
    for angka in 1..=30 {
        if angka % 15 == 0 {
            println!("FizzBuzz");
        } else if angka % 3 == 0 {
            println!("Fizz");
        } else if angka % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{angka}");
        }
    }
}
~~~

Kita memeriksa kelipatan 15 lebih dahulu karena angka tersebut juga kelipatan 3 dan 5.

## Praktik 2B — Konversi suhu

Rumus Celsius ke Fahrenheit adalah F = C × 9/5 + 32.

~~~rust
fn celsius_ke_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn main() {
    let hasil = celsius_ke_fahrenheit(25.0);
    println!("25°C = {hasil:.1}°F");
}
~~~

**Latihan pengembangan:** buat fungsi kebalikannya, Fahrenheit ke Celsius. Uji dengan 32°F dan 212°F.

## Praktik 2C — Input sederhana

~~~rust
use std::io;

fn main() {
    let mut input = String::new();

    println!("Masukkan suhu Celsius:");
    io::stdin()
        .read_line(&mut input)
        .expect("gagal membaca input");

    let celsius: f64 = input
        .trim()
        .parse()
        .expect("masukkan angka yang valid");

    println!("Hasil: {:.1}°F", celsius * 9.0 / 5.0 + 32.0);
}
~~~

read_line menambahkan teks ke String, sehingga variabelnya harus mutable. trim() membuang newline dan spasi di ujung. Contoh ini memakai expect supaya langkah awal mudah diikuti; pada aplikasi sungguhan, input salah lebih baik ditangani sebagai error yang ditampilkan dengan rapi.

### Latihan mandiri

1. Ubah program FizzBuzz agar berhenti pada angka yang dimasukkan pengguna.
2. Tambahkan fungsi yang menerima nilai dan mengembalikan true jika nilai berada di antara 0 dan 100.
3. Ubah program konversi suhu agar input yang bukan angka tidak membuat program panic. Simpan latihan ini untuk dibahas lagi pada Blok 4 tentang Result.

---

# Blok 3 — Ownership, borrowing, slice, dan lifetime

**Waktu:** 5–9 jam  
**Tujuan:** memahami aturan utama Rust dan bisa menjelaskan alasan compiler menolak kode tertentu.

Blok ini penting. Luangkan waktu tambahan jika perlu. Hindari mengatasi semua error dengan .clone(); pahami dulu kenapa data harus dipindahkan, dipinjam, atau dibuat salinannya.

## 3.1 Stack dan heap secara sederhana

Untuk membangun mental model awal:

- **Stack** menyimpan nilai dengan ukuran yang diketahui pada saat kompilasi dan biasanya dikelola dengan cepat.
- **Heap** menyimpan data berukuran dinamis. Nilai seperti String dan Vec<T> memiliki metadata kecil pada stack, sementara isi datanya biasanya berada di heap.

Ini gambaran belajar, bukan uraian lengkap tata letak memori untuk semua tipe. Yang lebih penting adalah memahami siapa pemilik data heap dan kapan data itu dibersihkan.

## 3.2 Ownership

Aturan dasarnya:

1. Setiap nilai memiliki satu owner.
2. Pada satu waktu, nilai mempunyai satu owner utama.
3. Ketika owner keluar dari scope, nilai dibersihkan secara otomatis.

~~~rust
fn main() {
    let teks_a = String::from("halo");
    let teks_b = teks_a; // ownership berpindah ke teks_b

    println!("{teks_b}");
    // println!("{teks_a}"); // error: teks_a sudah tidak memiliki String itu
}
~~~

Assignment let teks_b = teks_a untuk String biasanya **memindahkan** ownership. Setelah move, teks_a tidak dapat digunakan lagi.

Beberapa tipe sederhana seperti i32, bool, dan char menerapkan trait Copy. Nilainya disalin saat assignment:

~~~rust
let a = 8;
let b = a;
println!("{a} dan {b}"); // keduanya masih bisa dipakai
~~~

.clone() membuat salinan eksplisit. Pada String atau Vec, ini bisa berarti alokasi dan penyalinan data di heap. Clone tidak selalu salah, tetapi sebaiknya ada alasan.

## 3.3 Ownership ketika memanggil fungsi

~~~rust
fn konsumsi(teks: String) {
    println!("{teks}");
} // teks dibersihkan di akhir fungsi

fn hitung_panjang(teks: &String) -> usize {
    teks.len()
}

fn main() {
    let data = String::from("Rust");

    let panjang = hitung_panjang(&data);
    println!("{data} panjangnya {panjang}");

    konsumsi(data);
    // println!("{data}"); // data sudah dipindahkan ke konsumsi
}
~~~

Baca signature sebagai petunjuk:

- teks: String: fungsi menerima ownership.
- teks: &String: fungsi meminjam untuk dibaca.
- teks: &mut String: fungsi meminjam dengan izin untuk mengubah.

## 3.4 Borrowing

Aturan referensi yang perlu diingat:

- Boleh ada banyak referensi pembaca &T.
- Atau ada satu referensi penulis &mut T.
- Referensi penulis tidak boleh aktif bersamaan dengan referensi pembaca yang masih dipakai.

~~~rust
fn main() {
    let mut teks = String::from("Rust");

    let pembaca_1 = &teks;
    let pembaca_2 = &teks;
    println!("{pembaca_1} {pembaca_2}");

    let penulis = &mut teks; // aman setelah referensi pembaca terakhir dipakai
    penulis.push_str(" aman");

    println!("{penulis}");
}
~~~

Compiler menilai kapan sebuah reference benar-benar dipakai. Karena itu reference bisa selesai sebelum akhir scope tertulis jika tidak digunakan lagi.

Aturan ini mencegah akses tulis yang saling bertabrakan dalam kode aman. Compiler juga menolak banyak pola yang berpotensi menghasilkan data race.

## 3.5 Slice

Slice adalah pandangan ke rentang data tanpa mengambil ownership. Untuk string, tipe umumnya &str; untuk array atau vector, gunakan &[T].

~~~rust
fn hitung_kata(teks: &str) -> usize {
    teks.split_whitespace().count()
}

fn jumlahkan(nilai: &[i32]) -> i32 {
    nilai.iter().sum()
}
~~~

Parameter &str lebih fleksibel daripada &String: ia dapat menerima string literal dan potongan dari String. Begitu juga, &[T] lebih fleksibel daripada &Vec<T>.

String Rust disimpan sebagai UTF-8. Mengakses karakter dengan indeks byte seperti teks[0] tidak diperbolehkan karena satu karakter Unicode dapat memakai beberapa byte. Gunakan .chars(), .bytes(), atau metode string yang sesuai.

## 3.6 Lifetime

Lifetime adalah hubungan masa berlaku antara reference. Lifetime bukan timer dan bukan cara memperpanjang umur data. Compiler biasanya dapat menyimpulkannya sendiri.

Contoh signature yang menyatakan bahwa hasil adalah salah satu input:

~~~rust
fn lebih_panjang<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}
~~~

'a menyatakan bahwa reference hasil tidak boleh dipakai melebihi masa berlaku reference input yang terkait. Annotation ini menjelaskan hubungan kepada compiler; ia tidak membuat data hidup lebih lama.

Struct yang menyimpan reference biasanya perlu lifetime pada definisinya:

~~~rust
struct Pembaca<'a> {
    isi: &'a str,
}
~~~

Untuk pemula, mulailah dengan struct yang memiliki String sendiri jika itu membuat desain lebih jelas. Borrowing di dalam struct berguna, tetapi menambah aturan lifetime yang perlu dipahami.

## 3.7 Drop dan RAII

Ketika owner keluar dari scope, Rust menjalankan mekanisme pembersihan tipe tersebut. Ini disebut pola RAII: masa hidup resource mengikuti masa hidup objek pemiliknya. Pola ini umum digunakan untuk file, lock, dan resource lain, sehingga jalur keluar dari fungsi tidak mudah lupa membebaskan resource.

## 3.8 Praktik laboratorium borrow checker

Coba aktifkan contoh error satu per satu, lalu perbaiki.

**A. Pakai nilai setelah move**

~~~rust
let a = String::from("data");
let b = a;
println!("{a}"); // mengapa error?
~~~

Perbaikan yang mungkin:

- Pakai b setelah move.
- Pinjam &a jika fungsi hanya perlu membaca.
- Clone jika memang membutuhkan dua String yang terpisah.

**B. Dua reference mutable**

~~~rust
let mut angka = 1;
let a = &mut angka;
let b = &mut angka;
*a += 1;
*b += 1;
~~~

Perbaikan: gunakan satu reference mutable pada satu waktu, atau batasi scope peminjaman pertama.

**C. Pembaca dan penulis yang tumpang tindih**

~~~rust
let mut nama = String::from("Ayu");
let pembaca = &nama;
let penulis = &mut nama;
println!("{pembaca} {penulis}");
~~~

Perbaikan: selesaikan penggunaan pembaca sebelum membuat penulis.

### Error compiler yang perlu dikenali

| Kode error | Makna umum |
|---|---|
| E0382 | Nilai dipakai setelah ownership berpindah |
| E0499 | Lebih dari satu mutable borrow aktif |
| E0502 | Mutable borrow bertabrakan dengan immutable borrow |
| E0106 | Lifetime perlu dijelaskan pada konteks tertentu |
| E0597 | Reference tidak dapat hidup selama penggunaan yang diminta |

Nomor dan teks detail dapat berubah mengikuti kode. Baca konteks error dan bagian help; sering kali compiler menjelaskan lokasi peminjaman yang bertabrakan.

### Latihan akhir Blok 3

1. Buat fungsi yang menerima &str dan mengembalikan jumlah karakter.
2. Buat fungsi yang menerima &mut Vec<i32> dan menambahkan angka 10.
3. Berikan contoh kapan parameter String diperlukan dan kapan &str lebih sesuai.
4. Jelaskan dengan kata-katamu sendiri mengapa compiler menolak dua contoh error di atas.
5. Jika solusi AI menambahkan tiga .clone(), cari tahu apakah fungsi dapat memakai reference atau mengubah urutan kerja.

---

# Blok 4 — Struct, enum, pattern matching, dan error handling

**Waktu:** 9–12 jam  
**Tujuan:** membentuk model data yang jelas dan menangani kegagalan tanpa panic sebagai alur normal.

## 4.1 Struct dan method

Struct mengelompokkan beberapa field yang menggambarkan satu entitas.

~~~rust
#[derive(Debug)]
struct Pengguna {
    nama: String,
    umur: u32,
}

impl Pengguna {
    fn baru(nama: &str, umur: u32) -> Self {
        Self {
            nama: nama.to_string(),
            umur,
        }
    }

    fn sapa(&self) -> String {
        format!("Halo, {}", self.nama)
    }

    fn ulang_tahun(&mut self) {
        self.umur += 1;
    }
}
~~~

- Self berarti tipe yang sedang diimplementasikan.
- &self meminjam objek untuk dibaca.
- &mut self meminjam objek untuk diubah.
- self tanpa & dapat mengambil ownership objek.

#[derive(Debug)] meminta compiler membuat implementasi debug sederhana agar nilai dapat ditampilkan dengan {:?}.

## 4.2 Enum dan pattern matching

Enum menyatakan bahwa nilai hanya dapat berada dalam salah satu varian yang didefinisikan.

~~~rust
#[derive(Debug)]
enum Event {
    Login { pengguna: String },
    Logout(String),
    Error { kode: u16, pesan: String },
    Tick,
}
~~~

Setiap varian boleh membawa data yang berbeda. Pola ini membantu menyatakan kondisi yang mungkin terjadi secara eksplisit.

match harus menangani semua kemungkinan:

~~~rust
fn deskripsi(event: &Event) -> String {
    match event {
        Event::Login { pengguna } => format!("{pengguna} masuk"),
        Event::Logout(pengguna) => format!("{pengguna} keluar"),
        Event::Error { kode, .. } if *kode >= 500 => String::from("kesalahan server"),
        Event::Error { .. } => String::from("kesalahan lain"),
        Event::Tick => String::from("pemeriksaan berkala"),
    }
}
~~~

.. berarti field lain tidak perlu disebutkan. Guard if memberi syarat tambahan pada sebuah pola.

Jika varian enum baru ditambahkan, compiler dapat menunjukkan match mana yang perlu diperbarui. Ini alasan enum membantu membuat perubahan model data lebih aman.

## 4.3 Option: nilai mungkin tidak ada

Rust menggunakan Option<T> untuk menyatakan nilai yang mungkin ada atau tidak ada:

~~~rust
enum Option<T> {
    Some(T),
    None,
}
~~~

Misalnya, pencarian pada koleksi dapat mengembalikan None jika tidak menemukan hasil. Jangan menganggap semua pencarian pasti berhasil.

~~~rust
let angka = vec![10, 20, 30];

match angka.get(8) {
    Some(nilai) => println!("Nilai: {nilai}"),
    None => println!("Indeks tidak tersedia"),
}
~~~

## 4.4 Result: operasi mungkin gagal

Result<T, E> menyatakan operasi berhasil dengan nilai T, atau gagal dengan error E.

~~~rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
~~~

Contoh pembagian yang memeriksa input:

~~~rust
fn bagi(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("pembagi tidak boleh nol"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match bagi(10.0, 2.0) {
        Ok(hasil) => println!("Hasil: {hasil}"),
        Err(pesan) => println!("Tidak dapat menghitung: {pesan}"),
    }
}
~~~

Option menjawab “ada nilai atau tidak?”. Result menjawab “berhasil atau gagal, dan apa penyebab kegagalannya?”.

## 4.5 Operator ?

? memudahkan meneruskan error ke pemanggil. Fungsi yang memakainya harus mengembalikan tipe yang dapat mewakili kegagalan tersebut.

~~~rust
use std::{fs, io};

fn baca_file(path: &str) -> Result<String, io::Error> {
    let isi = fs::read_to_string(path)?;
    Ok(isi)
}
~~~

Jika read_to_string menghasilkan Err, ? segera mengembalikan error dari baca_file. Jika berhasil, nilai di dalam Ok diteruskan ke variabel isi.

## 4.6 Kapan panic boleh digunakan?

- Gunakan Result untuk kegagalan yang wajar: file tidak ditemukan, input salah, koneksi terputus.
- Gunakan Option untuk hasil yang mungkin memang tidak ada.
- panic! menandakan kondisi yang membuat program tidak dapat melanjutkan dengan benar, atau digunakan pada test/prototipe ketika kegagalan harus langsung menghentikan proses.
- unwrap() dan expect() akan panic ketika nilai yang diharapkan tidak tersedia. Hindari keduanya pada jalur input pengguna atau operasi yang dapat gagal.

Untuk aplikasi, crate anyhow membantu meneruskan dan memberi konteks pada error. Untuk library yang ingin menyediakan jenis error terstruktur, thiserror sering digunakan. Keduanya adalah pilihan ekosistem, bukan fitur bawaan bahasa.

## Praktik 4 — Parse baris log

Format baris yang akan digunakan:

~~~text
2026-10-16T10:00:01 ERROR auth login gagal user=budi
~~~

Rancangan data:

~~~rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Level {
    Info,
    Warn,
    Error,
}

#[derive(Debug)]
struct LogEntry {
    waktu: String,
    level: Level,
    sumber: String,
    pesan: String,
}
~~~

Tugasmu:

1. Pastikan baris punya waktu, level, sumber, dan pesan.
2. Terima level INFO, WARN, atau ERROR.
3. Jika level tidak dikenal atau field kurang, kembalikan Err.
4. Jangan panic hanya karena ada satu baris yang rusak.

Kerangka fungsi:

~~~rust
fn parse_log(baris: &str) -> Result<LogEntry, String> {
    // Ambil field dengan urutan yang disepakati.
    // Validasi level.
    // Kembalikan Ok(LogEntry) atau Err(pesan).
    todo!()
}
~~~

todo!() sengaja menyebabkan panic jika fungsi dijalankan. Ini penanda sementara agar compiler tetap mengenali bahwa isi fungsi belum dibuat. Jangan tinggalkan todo!() dalam program final.

### Latihan akhir Blok 4

- Ubah Result<f64, String> pada fungsi pembagian menjadi error yang menjelaskan input mana yang salah.
- Buat enum untuk status proses: Menunggu, Berjalan, Selesai, dan Gagal(String).
- Buat match yang menampilkan pesan berbeda untuk setiap status.
- Tambahkan satu kasus test untuk None atau Err.

---

# Blok 5 — Trait, generic, closure, dan iterator

**Waktu:** 12–15 jam  
**Tujuan:** memahami abstraksi yang sering muncul dalam library Rust dan membaca transformasi data berantai.

## 5.1 Trait

Trait mendefinisikan perilaku yang dapat disediakan oleh beberapa tipe.

~~~rust
trait Ringkasan {
    fn ringkas(&self) -> String;

    fn judul(&self) -> String {
        String::from("Tanpa judul")
    }
}

struct Artikel {
    isi: String,
}

impl Ringkasan for Artikel {
    fn ringkas(&self) -> String {
        self.isi.chars().take(20).collect()
    }
}
~~~

Artikel menyatakan bahwa ia mengimplementasikan trait Ringkasan. Method bawaan pada trait dapat digunakan atau diganti oleh implementasi.

Trait mirip interface dalam beberapa bahasa, tetapi trait juga menjadi bagian dari sistem generic, batasan tipe, dan dispatch Rust.

## 5.2 Generic dan trait bound

Generic memungkinkan fungsi menerima berbagai tipe, selama tipe tersebut memenuhi batasan yang dibutuhkan.

~~~rust
fn cetak_ringkasan<T: Ringkasan>(item: &T) {
    println!("{}", item.ringkas());
}

fn cetak_ringkasan_ringkas(item: &impl Ringkasan) {
    println!("{}", item.ringkas());
}

fn debug_dan_ringkas<T>(item: &T)
where
    T: Ringkasan + std::fmt::Debug,
{
    println!("{item:?}: {}", item.ringkas());
}
~~~

T: Ringkasan berarti tipe T harus menyediakan perilaku pada trait tersebut. Gunakan where jika batasan generic mulai panjang agar signature lebih mudah dibaca.

## 5.3 Static dispatch dan dynamic dispatch

- **Generic atau impl Trait:** compiler mengetahui tipe konkret dan membuat versi kode untuk tipe tersebut. Pendekatan ini disebut static dispatch.
- **dyn Trait:** program dapat bekerja dengan nilai dari tipe konkret yang berbeda melalui trait object. Pemanggilan dilakukan secara dinamis melalui mekanisme seperti vtable.

Contoh kapan dyn berguna adalah koleksi yang berisi beberapa tipe berbeda tetapi semuanya menerapkan trait yang sama. Ada trade-off: fleksibilitas runtime dan alokasi/indirection dibanding tipe konkret. Untuk kode awal, gunakan generic kecuali memang perlu menyimpan tipe beragam bersama-sama.

## 5.4 Trait yang sering ditemui

| Trait | Kegunaan umum |
|---|---|
| Debug | Menampilkan nilai untuk pemeriksaan developer |
| Display | Mengubah nilai menjadi tampilan yang ramah pengguna |
| Clone | Membuat salinan eksplisit |
| PartialEq / Eq | Membandingkan nilai |
| Hash | Membuat hash untuk koleksi seperti HashMap |
| Default | Menyediakan nilai awal |
| From / Into | Mengonversi satu tipe ke tipe lain |
| Iterator | Menyediakan urutan nilai yang dapat diolah |
| Send / Sync | Menandai tipe yang dapat dipindah atau dibagikan antarthread |
| Serialize / Deserialize | Umumnya disediakan oleh crate serde untuk data seperti JSON |

Attribute #[derive(...)] dapat membuat implementasi otomatis untuk beberapa trait, jika semua field mendukungnya.

## 5.5 Closure

Closure adalah fungsi kecil yang dapat menangkap nilai dari scope di sekitarnya.

~~~rust
let tambah = |a, b| a + b;

let faktor = 3;
let kali_faktor = |nilai| nilai * faktor;

println!("{}", tambah(2, 4));
println!("{}", kali_faktor(5));
~~~

Trait pemanggilan closure yang sering muncul:

- Fn: closure membaca atau meminjam nilai.
- FnMut: closure dapat mengubah state yang dipinjam secara mutable.
- FnOnce: closure mengambil ownership dari nilai yang ditangkap; pemanggilan bisa menghabiskan closure.

Kata kunci move memindahkan nilai yang ditangkap ke dalam closure. Ini sering muncul saat closure dikirim ke thread, karena thread baru harus memiliki data yang dipakainya.

## 5.6 Iterator

Iterator menghasilkan nilai satu per satu. Adapter seperti map dan filter bersifat lazy: data belum diproses sampai iterator dikonsumsi, misalnya dengan collect, sum, for, atau count.

~~~rust
let kuadrat_genap: Vec<u32> = (1..=10)
    .filter(|angka| angka % 2 == 0)
    .map(|angka| angka * angka)
    .collect();
~~~

Urutannya:

1. (1..=10) menghasilkan rentang.
2. filter hanya meneruskan angka genap.
3. map mengubah setiap angka menjadi kuadrat.
4. collect menjalankan proses dan mengumpulkan hasil menjadi Vec<u32>.

Perbedaan metode pada koleksi:

- iter() meminjam tiap elemen sebagai &T.
- iter_mut() meminjam tiap elemen sebagai &mut T.
- into_iter() mengonsumsi koleksi dan memberikan tiap elemen sebagai T.

Adapter dan konsumer yang umum: map, filter, enumerate, zip, take, skip, flat_map, fold, sum, any, all, find, collect.

Contoh mengubah teks menjadi panjang kata:

~~~rust
let panjang: Vec<usize> = "rust aman dan cepat"
    .split_whitespace()
    .map(str::len)
    .collect();
~~~

Contoh menangani beberapa hasil Result:

~~~rust
let angka: Result<Vec<i32>, _> = ["10", "20", "30"]
    .iter()
    .map(|teks| teks.parse::<i32>())
    .collect();
~~~

Jika satu parse gagal, hasil keseluruhan dapat menjadi Err. Ini berguna untuk merangkai operasi yang mungkin gagal.

Iterator Rust sering dikompilasi dengan optimisasi yang baik. Tetap pilih bentuk yang paling jelas bagi pembaca. Klaim bahwa abstraksi tidak menambah biaya harus dipahami sebagai tujuan desain dan optimisasi, bukan alasan untuk mengabaikan pengukuran saat performa menjadi masalah.

## Praktik 5 — Mengganti loop dengan iterator

Versi loop:

~~~rust
let angka = vec![1, 2, 3, 4, 5, 6];
let mut hasil = Vec::new();

for n in angka {
    if n % 2 == 0 {
        hasil.push(n * n);
    }
}
~~~

Tugas:

1. Tulis ulang menggunakan filter, map, dan collect.
2. Tambahkan .take(2) agar hanya dua hasil pertama yang dipakai.
3. Buat trait SumberLog dengan method nama_sumber().
4. Implementasikan trait untuk dua struct berbeda.
5. Jelaskan kapan generic lebih mudah dipakai daripada dyn Trait.

### Cek pemahaman

- Apa bedanya iter() dan into_iter()?
- Pada baris chain mana pekerjaan benar-benar mulai dijalankan?
- Closure mana yang diperlukan bila nilai yang ditangkap akan diubah?

---

# Blok 6 — Koleksi, String, module, Cargo, dan testing

**Waktu:** 15–18 jam  
**Tujuan:** mengorganisasi program kecil dan memeriksa perilaku dengan test.

## 6.1 Koleksi

| Koleksi | Kegunaan |
|---|---|
| Vec<T> | Daftar berurutan yang ukurannya dapat bertambah |
| HashMap<K, V> | Pemetaan key ke value, pencarian umum yang cepat |
| HashSet<T> | Menyimpan nilai unik |
| VecDeque<T> | Antrean dua arah |
| BTreeMap<K, V> | Pemetaan berurutan berdasarkan key |

Contoh Vec:

~~~rust
let mut angka = vec![1, 2, 3];
angka.push(4);

match angka.get(8) {
    Some(nilai) => println!("{nilai}"),
    None => println!("indeks tidak ada"),
}
~~~

Akses angka[8] akan panic ketika indeks tidak ada. Jika indeks berasal dari input atau perhitungan, .get(index) lebih aman karena mengembalikan Option.

Contoh menghitung kemunculan kata:

~~~rust
use std::collections::HashMap;

let mut hitung = HashMap::new();

for kata in "rust aman rust cepat".split_whitespace() {
    *hitung.entry(kata).or_insert(0) += 1;
}
~~~

entry membantu memasukkan nilai awal jika key belum ada, lalu memperbarui nilai yang sudah ada.

## 6.2 String dan &str

- String memiliki data teks sendiri dan dapat berubah ukuran.
- &str meminjam bagian teks yang sudah ada.
- String literal seperti "halo" adalah &'static str.
- Teks Rust memakai UTF-8. Byte ke-0 tidak selalu sama dengan karakter ke-0.

Untuk fungsi yang hanya membaca teks, pilih &str sebagai parameter umum:

~~~rust
fn sapa(nama: &str) -> String {
    format!("Halo, {nama}")
}
~~~

Pemanggil dapat menyerahkan &String yang akan dikoersi menjadi &str, atau langsung memberikan string literal.

## 6.3 Module dan visibility

Module memecah program menjadi bagian yang tanggung jawabnya jelas. Item bersifat private secara default. Tambahkan pub jika item perlu dipakai dari luar module.

Contoh struktur:

~~~text
src/
├── main.rs
├── model.rs
└── parser.rs
~~~

Di main.rs:

~~~rust
mod model;
mod parser;

use model::LogEntry;
use parser::parse_line;
~~~

Di model.rs, tipe yang digunakan dari module lain perlu diekspos:

~~~rust
pub struct LogEntry {
    pub waktu: String,
}
~~~

Visibility membuat batas API eksplisit. Jangan menjadikan semua field pub tanpa alasan; buka bagian yang memang perlu diakses module lain.

## 6.4 Cargo dan dependency

Cargo.toml menyimpan metadata dan dependency yang dipakai proyek. Cargo.lock mencatat versi dependency yang terpilih untuk aplikasi agar build lebih konsisten.

Perintah berikut tersedia pada toolchain Cargo modern:

~~~text
cargo add serde
cargo add clap --features derive
~~~

Jika subcommand cargo add tidak tersedia, tambahkan dependency pada bagian [dependencies] di Cargo.toml, lalu jalankan cargo check.

Saat memilih crate:

1. Baca dokumentasinya.
2. Periksa kompatibilitas dan versi.
3. Periksa aktivitas dan pemeliharaan crate.
4. Pahami fitur yang diaktifkan.
5. Hindari memasang dependency hanya untuk satu baris kode jika standard library sudah cukup.

Beberapa crate yang sering ditemui:

| Crate | Kegunaan |
|---|---|
| serde dan serde_json | Serialisasi dan deserialisasi data, termasuk JSON |
| clap | Membaca argumen command-line |
| tokio | Runtime dan utilitas async |
| anyhow | Menangani error aplikasi dengan konteks |
| thiserror | Membuat jenis error terstruktur |
| tracing | Logging terstruktur untuk aplikasi |
| reqwest | HTTP client |
| axum | Web framework |
| regex | Pencarian berbasis regular expression |
| rayon | Pemrosesan data paralel |

Nama crate dan API dapat berubah. Untuk kerja nyata, periksa dokumentasi versi yang dipakai proyek.

## 6.5 Testing

Test memeriksa bahwa perilaku program sesuai dengan yang diharapkan. Rust menyediakan test bawaan lewat attribute #[test].

~~~rust
fn bagi(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pembagian_normal() {
        assert_eq!(bagi(10, 2), Some(5));
    }

    #[test]
    fn pembagi_nol_tidak_menghasilkan_nilai() {
        assert_eq!(bagi(10, 0), None);
    }
}
~~~

Jalankan dengan cargo test.

- Unit test biasanya ditempatkan dekat kode yang diuji dalam module tests.
- Integration test berada di folder tests/ dan memakai API publik crate.
- Contoh pada dokumentasi juga dapat dijalankan sebagai doctest.

Test sebaiknya memeriksa perilaku yang terlihat, termasuk batas dan kegagalan. Compiler tidak mengetahui hasil bisnis yang benar.

## Praktik 6 — Pecah program menjadi module

Ambil fungsi parse log dari Blok 4, lalu pisahkan:

- model.rs: enum level dan struct entri log;
- parser.rs: fungsi parse dan test parser;
- main.rs: titik masuk program.

Buat setidaknya tiga test:

1. Baris valid menghasilkan field yang sesuai.
2. Level yang tidak dikenal menghasilkan error.
3. Baris tanpa pesan menghasilkan error.

Jalankan cargo fmt, cargo check, dan cargo test.

---

# Blok 7 — Smart pointer, concurrency, async, dan unsafe

**Waktu:** 18–21 jam  
**Tujuan:** mengenali kapan data dibagi, kapan pekerjaan konkuren berguna, dan kapan kode memerlukan kehati-hatian ekstra.

## 7.1 Smart pointer

Smart pointer adalah tipe yang menambahkan perilaku pengelolaan atau akses terhadap data.

| Tipe | Kegunaan | Catatan |
|---|---|---|
| Box<T> | Memiliki satu nilai di heap | Berguna untuk tipe rekursif atau trait object |
| Rc<T> | Beberapa owner dalam satu thread | Reference count non-atomic; tidak untuk berbagi antarthread |
| Arc<T> | Beberapa owner yang dapat dibagi antarthread | Reference count atomic; tidak otomatis membuat isi dapat diubah dengan aman |
| RefCell<T> | Memeriksa aturan borrow saat runtime | Pelanggaran dapat menyebabkan panic |
| Mutex<T> | Mengizinkan satu akses mutable pada satu waktu | Thread lain menunggu lock |
| RwLock<T> | Banyak pembaca atau satu penulis | Cocok saat pola aksesnya sesuai |

Pola yang sering ditemui:

- Rc<RefCell<T>>: beberapa owner dan mutasi yang diperiksa saat runtime, dalam satu thread.
- Arc<Mutex<T>>: berbagi ownership antarthread dengan akses yang dilindungi lock.

Tipe gabungan bukan jawaban default. Jika sebuah struct dapat dimiliki oleh satu pihak dan dibagikan melalui pesan atau reference yang lebih sederhana, desain itu mungkin lebih mudah dibaca.

## 7.2 Thread dan berbagi data

Contoh berikut membuat beberapa thread yang memperbarui counter bersama:

~~~rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let total = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..4 {
        let total_thread = Arc::clone(&total);

        let handle = thread::spawn(move || {
            let mut nilai = total_thread.lock().expect("mutex poisoned");
            *nilai += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("thread panicked");
    }

    println!("Total: {}", *total.lock().expect("mutex poisoned"));
}
~~~

- Arc membuat beberapa thread dapat memiliki handle ke data yang sama.
- Mutex memastikan hanya satu thread mengubah nilai pada satu waktu.
- move memindahkan handle thread ke closure.
- join menunggu thread selesai.

Contoh ini menggunakan expect agar konsep thread lebih ringkas. Pada aplikasi nyata, tentukan bagaimana kegagalan lock dan panic thread harus dilaporkan.

### Send dan Sync

- Send: sebuah nilai boleh dipindahkan ke thread lain.
- Sync: reference ke sebuah nilai boleh dibagikan ke thread lain.

Banyak tipe standar otomatis memenuhi aturan ini jika isi di dalamnya memenuhi syarat. Rc<T> tidak untuk transfer antarthread karena penghitung referensinya tidak dirancang untuk akses lintas thread.

### Channel

Channel memungkinkan thread mengirim pesan ke thread lain, alih-alih berbagi state mutable secara langsung. Ini dapat membuat alur kepemilikan lebih sederhana: satu bagian program memiliki data, bagian lain mengirim instruksi atau hasil.

Latihan: buat beberapa thread yang menghitung jumlah kata pada teks berbeda, lalu kirim tiap hasil melalui std::sync::mpsc::channel. Bandingkan desain ini dengan counter Arc<Mutex<usize>>.

## 7.3 Async dan Future

Fungsi async menghasilkan sebuah Future. Membuat Future belum tentu langsung menjalankan pekerjaan di dalamnya; Future perlu dijalankan dan dipoll oleh runtime, lalu .await digunakan untuk menunggu hasilnya dalam alur async.

Rust tidak menentukan satu runtime async bawaan. Tokio adalah salah satu runtime yang banyak digunakan. Async berguna terutama untuk mengelola banyak operasi I/O yang menunggu, seperti koneksi jaringan atau akses asynchronous.

Contoh pola Tokio:

~~~rust
async fn ambil_data(id: u32) -> u32 {
    id * 10
}

#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(ambil_data(1), ambil_data(2));
    println!("{a} dan {b}");
}
~~~

Contoh dependency:

~~~toml
[dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread", "time"] }
~~~

tokio::join! menunggu beberapa Future dalam satu task. Ini memberi kesempatan operasi menunggu berjalan konkuren; bukan berarti pekerjaan CPU otomatis dibagi ke beberapa core.

Panduan pemilihan:

- **I/O-bound:** banyak waktu menunggu file, socket, atau layanan lain; async dapat membantu mengelola banyak tugas.
- **CPU-bound:** pekerjaan intensif seperti kompresi atau analisis besar; pertimbangkan thread, worker pool, atau Rayon.
- Jangan memanggil operasi blocking yang lama pada executor async tanpa memahami dampaknya. Gunakan API asynchronous atau jalur spawn_blocking ketika sesuai.
- Hindari menahan lock synchronous terlalu lama. Berhati-hatilah jika guard lock masih hidup saat kode melakukan .await.

Async membuat alur yang menunggu lebih skalabel dalam situasi tertentu, tetapi juga menambah kompleksitas. Gunakan ketika kebutuhan memang ada, bukan karena semua kode Rust harus async.

## 7.4 unsafe

unsafe memberi akses ke beberapa operasi yang tidak dapat dibuktikan aman oleh compiler, seperti dereference raw pointer, pemanggilan fungsi unsafe, implementasi unsafe trait, akses pada union, dan beberapa kasus global mutable.

unsafe bukan tombol untuk mematikan semua aturan borrow checker. Programmer tetap harus menjaga kontrak keamanan yang tidak dapat diperiksa compiler.

Prinsip review:

1. Cari alasan konkret mengapa unsafe diperlukan.
2. Batasi blok unsafe sekecil mungkin.
3. Dokumentasikan invariant yang harus dipenuhi.
4. Bungkus implementasi low-level dengan API aman jika memungkinkan.
5. Jangan menerima kode unsafe dari AI hanya karena kode tersebut berhasil dikompilasi.

### Praktik akhir Blok 7

1. Jalankan contoh thread counter.
2. Ubah jumlah thread dan jelaskan hasilnya.
3. Implementasikan penghitungan beberapa file dengan channel.
4. Buat fungsi async yang menunggu tokio::time::sleep untuk simulasi I/O.
5. Jelaskan mengapa async tidak otomatis mempercepat perhitungan CPU.
6. Cari satu crate yang menggunakan unsafe; baca dokumentasi keamanan API-nya, jangan menyalin unsafe ke proyek sendiri.

---

# Blok 8 — Proyek akhir: CLI log analyzer

**Waktu:** 21–24 jam  
**Tujuan:** menggabungkan model data, parsing, error handling, koleksi, module, dependency, dan testing ke dalam proyek kecil.

## 8.1 Spesifikasi program

Buat program CLI yang membaca file log dengan satu format per baris:

~~~text
2026-10-16T10:00:01 ERROR auth login gagal user=budi
2026-10-16T10:00:02 INFO api permintaan diterima
2026-10-16T10:00:03 WARN storage ruang hampir penuh
~~~

Program harus:

1. Menerima lokasi file dari argumen CLI.
2. Parse waktu, level, sumber, dan pesan.
3. Mendukung level INFO, WARN, dan ERROR.
4. Menghitung jumlah baris valid menurut level.
5. Menampilkan lima sumber paling sering.
6. Mengabaikan baris rusak dengan mencatat nomor dan penyebabnya.
7. Mendukung filter opsional, misalnya --level error.
8. Memiliki minimal tiga test parser.
9. Tidak panic hanya karena file tidak ada atau ada baris yang formatnya salah.

## 8.2 Buat proyek

~~~text
cargo new log-analyzer
cd log-analyzer
cargo add anyhow
cargo add clap --features derive
~~~

Jika cargo add tidak tersedia, edit Cargo.toml menjadi:

~~~toml
[package]
name = "log-analyzer"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1"
clap = { version = "4", features = ["derive"] }
~~~

Buat struktur berikut:

~~~text
src/
├── main.rs
├── model.rs
└── parser.rs
~~~

## 8.3 Buat model data

Isi src/model.rs:

~~~rust
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Level {
    Info,
    Warn,
    Error,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
        };
        write!(f, "{label}")
    }
}

#[derive(Debug)]
pub struct ParseLevelError(pub String);

impl fmt::Display for ParseLevelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "level tidak dikenal: {}", self.0)
    }
}

impl std::error::Error for ParseLevelError {}

impl FromStr for Level {
    type Err = ParseLevelError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_uppercase().as_str() {
            "INFO" => Ok(Level::Info),
            "WARN" => Ok(Level::Warn),
            "ERROR" => Ok(Level::Error),
            _ => Err(ParseLevelError(value.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: Level,
    pub source: String,
    pub message: String,
}
~~~

Hash memungkinkan Level dipakai sebagai key di HashMap. FromStr membuat parse() dapat mengubah teks seperti "ERROR" menjadi enum Level.

## 8.4 Buat parser

Isi src/parser.rs:

~~~rust
use crate::model::{Level, LogEntry};

fn take_word(input: &str) -> Option<(&str, &str)> {
    let input = input.trim_start();

    if input.is_empty() {
        return None;
    }

    let end = input.find(char::is_whitespace).unwrap_or(input.len());
    let word = &input[..end];
    let rest = &input[end..];

    Some((word, rest))
}

pub fn parse_line(line: &str) -> Result<LogEntry, String> {
    let (timestamp, rest) =
        take_word(line).ok_or_else(|| String::from("waktu tidak ditemukan"))?;

    let (raw_level, rest) =
        take_word(rest).ok_or_else(|| String::from("level tidak ditemukan"))?;

    let (source, rest) =
        take_word(rest).ok_or_else(|| String::from("sumber tidak ditemukan"))?;

    let message = rest.trim_start();
    if message.is_empty() {
        return Err(String::from("pesan tidak ditemukan"));
    }

    let level = raw_level
        .parse::<Level>()
        .map_err(|error| error.to_string())?;

    Ok(LogEntry {
        timestamp: timestamp.to_string(),
        level,
        source: source.to_string(),
        message: message.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Level;

    #[test]
    fn parse_baris_valid() {
        let entry = parse_line(
            "2026-10-16T10:00:01 ERROR auth login gagal user=budi",
        )
        .expect("baris harus valid");

        assert_eq!(entry.level, Level::Error);
        assert_eq!(entry.source, "auth");
        assert_eq!(entry.message, "login gagal user=budi");
    }

    #[test]
    fn menolak_level_tidak_dikenal() {
        let hasil = parse_line("2026-10-16T10:00:01 DEBUG auth tes");
        assert!(hasil.is_err());
    }

    #[test]
    fn menolak_pesan_kosong() {
        let hasil = parse_line("2026-10-16T10:00:01 INFO auth");
        assert!(hasil.is_err());
    }
}
~~~

take_word mengambil satu token dan sisa teksnya. Parser tidak mengasumsikan hanya ada satu spasi antarfield. Setelah tiga field pertama, sisa teks dianggap sebagai pesan supaya pesan boleh berisi spasi.

Parser mengembalikan Result karena format baris dari file bisa salah. Ia tidak perlu menghentikan seluruh program hanya karena satu baris tidak valid.

## 8.5 Buat CLI dan statistik

Isi src/main.rs:

~~~rust
mod model;
mod parser;

use anyhow::{Context, Result};
use clap::Parser;
use model::Level;
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
};

#[derive(Debug, Parser)]
#[command(name = "log-analyzer", about = "Menganalisis file log sederhana")]
struct Args {
    /// Lokasi file log yang akan dianalisis
    file: PathBuf,

    /// Filter level: info, warn, atau error
    #[arg(long, value_name = "LEVEL")]
    level: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let filter = args
        .level
        .as_deref()
        .map(str::parse::<Level>)
        .transpose()?;

    let contents = fs::read_to_string(&args.file)
        .with_context(|| format!("gagal membaca file {}", args.file.display()))?;

    let mut counts: HashMap<Level, usize> = HashMap::new();
    let mut sources: HashMap<String, usize> = HashMap::new();
    let mut malformed = 0;
    let mut included = 0;

    for (index, line) in contents.lines().enumerate() {
        match parser::parse_line(line) {
            Ok(entry) => {
                let should_include = filter.map_or(true, |level| level == entry.level);

                if should_include {
                    *counts.entry(entry.level).or_insert(0) += 1;
                    *sources.entry(entry.source).or_insert(0) += 1;
                    included += 1;
                }
            }
            Err(error) => {
                malformed += 1;
                eprintln!("Baris {} diabaikan: {}", index + 1, error);
            }
        }
    }

    println!("Baris valid yang dihitung: {included}");

    for level in [Level::Error, Level::Warn, Level::Info] {
        println!("{level}: {}", counts.get(&level).copied().unwrap_or(0));
    }

    let mut top_sources: Vec<(String, usize)> = sources.into_iter().collect();
    top_sources.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    println!("Lima sumber teratas:");
    for (source, count) in top_sources.iter().take(5) {
        println!("- {source}: {count}");
    }

    println!("Baris rusak: {malformed}");
    Ok(())
}
~~~

anyhow::Context menambahkan keterangan berguna jika pembacaan file gagal. Perhatikan bahwa unwrap_or(0) di sini bukan unwrap() yang panic: ia memberi nilai default jika key statistik belum ada.

filter.map_or(true, ...) berarti: jika filter tidak diberikan, masukkan semua level; jika filter diberikan, masukkan baris yang level-nya sama. Statistik level dan sumber dihitung hanya untuk baris yang lolos filter. Baris rusak tetap dilaporkan.

## 8.6 Jalankan dan uji

Buat sample.log:

~~~text
2026-10-16T10:00:01 ERROR auth login gagal user=budi
2026-10-16T10:00:02 INFO api permintaan diterima
2026-10-16T10:00:03 WARN storage ruang hampir penuh
2026-10-16T10:00:04 ERROR auth token tidak valid
baris ini tidak lengkap
~~~

Jalankan:

~~~text
cargo fmt
cargo check
cargo test
cargo run -- sample.log
cargo run -- sample.log --level error
~~~

Pada perintah pertama, parser seharusnya mengabaikan baris yang tidak lengkap dan menampilkan nomor barisnya. Pada perintah kedua, statistik hanya menghitung error, sementara baris rusak tetap dicatat.

## 8.7 Peningkatan bertahap

Tambahkan satu fitur pada satu waktu:

1. **Validasi waktu:** tentukan apakah timestamp harus diperiksa formatnya atau cukup disimpan sebagai teks.
2. **JSON:** gunakan serde dan serde_json agar hasil dapat dipakai oleh tool lain.
3. **Statistik rentang waktu:** kelompokkan jumlah log per menit atau per jam.
4. **File besar:** ubah pembacaan penuh menjadi pembacaan per baris menggunakan BufRead.
5. **Multi-file:** terima beberapa file dan gabungkan statistik.
6. **Benchmark:** ukur sebelum memutuskan apakah perlu Rayon atau optimisasi lain.
7. **Async:** jadikan proyek async hanya jika ada kebutuhan I/O konkuren yang nyata, seperti menerima stream log dari beberapa koneksi.

## 8.8 Apa yang harus bisa kamu jelaskan saat proyek selesai

- Mengapa Level dibuat sebagai enum?
- Mengapa parse_line mengembalikan Result?
- Mengapa timestamp dan source dimiliki oleh LogEntry sebagai String?
- Mengapa HashMap sesuai untuk hitungan per level dan sumber?
- Apa bedanya baris rusak dengan file yang gagal dibuka?
- Di mana proyek menggunakan .clone()? Jika tidak ada, apakah itu disengaja?
- Bagaimana cara menambah level baru tanpa lupa memperbarui tampilan dan statistik?

---

# Workflow belajar dan vibe coding dengan Rust

## Minta AI mengerjakan bagian kecil

Mulai dari desain tipe dan signature, bukan langsung meminta seluruh aplikasi:

1. Minta struct dan enum.
2. Periksa apakah semua status yang diperlukan sudah dimodelkan.
3. Minta signature fungsi dan jenis error.
4. Periksa ownership dan jenis parameter.
5. Baru minta isi fungsi.
6. Tambahkan test dan jalankan sendiri.

## Prompt yang dapat digunakan

~~~text
Saya sedang belajar Rust. Bantu implementasikan fungsi [nama fungsi] untuk [tujuan].

Batasan:
- Jelaskan ownership setiap nilai yang penting.
- Gunakan &str atau &[T] jika fungsi hanya perlu membaca.
- Gunakan Result untuk kegagalan yang wajar.
- Hindari unwrap dan expect pada jalur input pengguna.
- Jangan menambahkan clone tanpa menjelaskan alasan biaya atau ownership-nya.
- Sertakan test untuk kasus normal dan kasus gagal.
- Tampilkan langkah perubahan kecil, bukan seluruh proyek sekaligus.
~~~

Ketika ada error compiler, kirimkan pesan lengkap bersama potongan kode terkait. Minta AI menjelaskan penyebab dan alternatif desain, bukan hanya memberikan kode pengganti.

## Checklist review kode Rust dari AI

Sebelum menerima perubahan, periksa:

- Apakah tiap String, Vec, atau nilai heap punya owner yang jelas?
- Apakah fungsi hanya membaca data tetapi meminta String atau Vec dengan ownership?
- Apakah ada .clone() yang tidak dibutuhkan?
- Apakah unwrap() atau expect() dipakai pada operasi yang wajar gagal?
- Apakah operasi indexing atau slicing dapat menerima input di luar batas?
- Apakah match menangani semua varian?
- Apakah error memberi konteks yang cukup bagi pengguna?
- Apakah test mencakup nilai kosong, input rusak, dan batas?
- Apakah dependency baru memang diperlukan dan dokumentasinya dipercaya?
- Apakah ada unsafe, Rc<RefCell<...>>, atau Arc<Mutex<...>> tanpa alasan desain yang jelas?
- Apakah operasi blocking dilakukan di dalam kode async?
- Apakah perubahan sudah melewati cargo fmt, cargo check, cargo clippy, dan cargo test?

---

# Rangkuman konsep yang perlu melekat

1. **Ownership:** setiap nilai memiliki pemilik; nilai dibersihkan saat owner keluar dari scope.
2. **Move:** ownership dapat berpindah; setelah berpindah, binding lama tidak selalu dapat dipakai.
3. **Borrowing:** banyak pembaca atau satu penulis aktif pada satu waktu.
4. **Lifetime:** menjelaskan hubungan masa berlaku reference; bukan memperpanjang umur data.
5. **Struct:** menyimpan kumpulan field yang menggambarkan satu entitas.
6. **Enum dan match:** menyatakan pilihan state yang mungkin dan menangani setiap pilihan.
7. **Option:** hasil mungkin ada atau mungkin tidak.
8. **Result:** operasi bisa berhasil atau gagal dengan alasan yang dapat ditangani.
9. **Trait:** mendefinisikan perilaku yang dapat dimiliki banyak tipe.
10. **Generic:** menulis kode yang dapat bekerja dengan berbagai tipe yang memenuhi syarat.
11. **Iterator:** merangkai transformasi data; adapter umumnya lazy sampai dikonsumsi.
12. **Koleksi:** pilih bentuk koleksi berdasarkan cara data dicari, disusun, dan diubah.
13. **Send dan Sync:** marker yang membantu memastikan data aman dipindahkan atau dibagikan antarthread.
14. **Interior mutability:** memungkinkan mutasi melalui tipe tertentu dengan pemeriksaan yang dapat terjadi saat runtime.
15. **Async:** mengelola pekerjaan yang menunggu; perlu runtime dan tidak otomatis mempercepat pekerjaan CPU.
16. **Unsafe:** bagian kecil tempat programmer memegang kontrak keamanan yang tidak dapat dibuktikan compiler.

# Kesalahan umum saat mulai belajar

- Mengatasi semua error borrow checker dengan .clone(), tanpa memahami siapa yang seharusnya memiliki data.
- Menyimpan reference dalam struct terlalu awal sebelum memahami lifetime.
- Memakai String untuk semua parameter padahal fungsi cukup menerima &str.
- Memakai unwrap() pada file, input, jaringan, atau operasi lain yang dapat gagal.
- Menganggap program yang lolos compiler berarti logikanya sudah benar.
- Menambahkan Arc<Mutex<...>> tanpa benar-benar perlu berbagi state mutable.
- Menggunakan async pada pekerjaan CPU-bound tanpa alasan.
- Menganggap unsafe sebagai jalan pintas untuk memperbaiki desain ownership.
- Menonton atau membaca contoh tanpa mengetik dan mengubahnya sendiri.
- Mengubah terlalu banyak hal sekaligus, sehingga sulit mengetahui sumber error.

# Sumber belajar lanjutan

- [The Rust Programming Language](https://doc.rust-lang.org/stable/book/) — buku utama dengan penjelasan bertahap.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — konsep yang disertai contoh kode.
- [Rustlings](https://rustlings.rust-lang.org/) — latihan kecil untuk membiasakan diri dengan compiler.
- [Comprehensive Rust](https://google.github.io/comprehensive-rust/) — materi kursus Rust yang tersusun.
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial) — panduan async menggunakan Tokio.
- [docs.rs](https://docs.rs/) — dokumentasi crate.
- [crates.io](https://crates.io/) — registri crate Rust.

