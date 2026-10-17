# Rust dalam 24 Jam — Peta Belajar buat Vibe Coding

*Fokus ke paham konsep dan bisa review kode, bukan hafal syntax.*

Tujuan: setelah 24 jam kamu tahu Rust dipakai buat apa, cara mikirnya, dan cukup ngerti konsep inti (ownership, borrowing, trait, error handling, concurrency) buat **baca, review, dan benerin** kode Rust yang ditulis AI.

## Cara pakai materi ini

- 24 jam ini jam efektif, bukan nonstop. Realistisnya 4–6 hari, 4–6 jam per hari.
- Compiler Rust itu cerewet. Buat vibe coding ini malah enak: kalau AI nulis kode yang salah, compiler yang nangkep. Tapi kamu harus bisa baca errornya.
- Ketik sendiri semua contoh (jangan copy-paste), ubah dikit sampai error, lalu baca pesannya. Separuh belajar Rust itu belajar baca error compiler.
- Tiap kali AI ngasih kode Rust, kamu harus bisa jawab tiga hal: siapa owner data ini, siapa yang minjam (mutable atau nggak), dan error-nya ditangani atau cuma di-`unwrap`.

## Peta 24 jam

| Blok | Jam | Topik | Output |
| --- | --- | --- | --- |
| 1 | 0–2 | Rust itu apa, dipakai di mana, setup | Hello world jalan, paham kapan pilih Rust |
| 2 | 2–5 | Syntax dasar | FizzBuzz dan konversi suhu |
| 3 | 5–9 | Ownership, borrowing, lifetime | Bisa jelasin kenapa compiler nolak sebuah kode |
| 4 | 9–12 | struct, enum, match, error handling | Model data pakai enum + Result |
| 5 | 12–15 | trait, generic, closure, iterator | Loop diganti iterator chain |
| 6 | 15–18 | Collection, module, Cargo, testing | Proyek multi-file + 3 test |
| 7 | 18–21 | Smart pointer, concurrency, async | Program multi-thread + tokio dasar |
| 8 | 21–24 | Proyek akhir + workflow vibe coding | CLI log analyzer |

## Blok 1 (jam 0–2): Rust itu apa dan dipakai di mana

### Rust dalam satu paragraf

Rust itu bahasa compiled, statically typed, tanpa garbage collector dan tanpa runtime berat. Performanya selevel C/C++, tapi keamanan memorinya dicek compiler lewat sistem **ownership**. Bug yang di C/C++ baru muncul pas runtime (use-after-free, dangling pointer, data race, buffer overflow) di Rust ditolak sebelum programnya sempat jalan. Harganya: kurva belajar curam di awal dan compile time lumayan lama.

### Dipakai di mana

- **Sistem dan OS**: Linux kernel (dukungan Rust masuk sejak versi 6.1), Android (kode baru di komponen low-level banyak pakai Rust), sebagian komponen Windows.
- **Infrastruktur dan cloud**: Firecracker (microVM di balik AWS Lambda), Pingora (proxy Cloudflare), sebagian service Discord.
- **Tooling CLI**: ripgrep, fd, bat, ruff dan uv (tooling Python), Deno.
- **Database dan data**: TiKV, SurrealDB, Qdrant, Polars.
- **Web dan desktop**: backend (axum, actix-web), desktop app lewat Tauri, WebAssembly buat jalan di browser.
- **Embedded dan IoT**: microcontroller tanpa OS (`no_std`).
- **Keamanan dan jaringan**: agent/sensor yang jalan lama, parser buat input yang nggak dipercaya. Di sini memory safety kepake banget.
- **Game**: engine Bevy.
- **Blockchain**: Solana dan beberapa chain lain.

### Kapan pilih Rust, kapan jangan

- **Pilih** kalau butuh performa plus keandalan, program jalan lama (service/agent), ngolah data dari luar yang nggak dipercaya, concurrency berat, atau binary kecil tanpa runtime.
- **Jangan** buat script sekali pakai, prototyping super cepat, data science eksploratif (Python lebih cocok), atau CRUD kecil di tim yang udah jago Node/Python.

### Mental model

Rust memindahkan bug dari runtime ke compile time. Kalau rasanya "dilawan compiler", itu normal: dia lagi ngasih tau bug yang di bahasa lain baru ketemu di production.

### Setup

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo new halo
cd halo
cargo run
```

Perintah yang bakal sering dipakai:

- `cargo check`: cek error cepat tanpa build penuh
- `cargo fmt`: rapihin format
- `cargo clippy`: linter, ngasih saran kode yang idiomatis
- `cargo build --release`: build optimasi penuh
- `cargo test`: jalanin test

Pasang juga **rust-analyzer** di editor.

**Latihan:** bikin proyek baru, ubah pesan hello world, jalanin `cargo check`, lalu sengaja salah ketik biar muncul error.

## Blok 2 (jam 2–5): Syntax dasar

### Variabel

```rust
let x = 5;             // immutable secara default
let mut y = 10;        // mutable
y += 1;
let x = x * 2;         // shadowing: bikin variabel baru bernama sama
const MAX: u32 = 100;  // konstanta, tipe wajib ditulis
```

Shadowing boleh ganti tipe, `mut` nggak boleh.

### Tipe dasar

- Integer: `i8`..`i128`, `u8`..`u128`, `isize`/`usize` (`usize` dipakai buat index). Default `i32`.
- Float: `f32`, `f64` (default).
- `bool`, `char` (4 byte, satu Unicode scalar).
- Tuple: `(i32, f64, char)`, akses pakai `t.0`.
- Array: `[i32; 3]`, panjang tetap.
- `()` atau unit type, artinya "nggak ada nilai".

### Fungsi dan expression

```rust
fn tambah(a: i32, b: i32) -> i32 {
    a + b   // tanpa titik koma = ini nilai yang di-return
}
```

Rust itu expression-based. `if`, `match`, dan blok `{}` semuanya menghasilkan nilai:

```rust
let label = if skor >= 70 { "lulus" } else { "remedial" };
```

### Control flow

```rust
for i in 0..5 { }          // 0 sampai 4
for i in 0..=5 { }         // 0 sampai 5
while n > 0 { n -= 1; }
let hasil = loop { break 42; };  // loop bisa return nilai
```

### Print dan macro

Tanda `!` artinya macro, bukan fungsi biasa.

```rust
println!("{} {:?} {:>5}", a, b, c);
// {} = Display (buat user), {:?} = Debug (buat developer), {:>5} = rata kanan lebar 5
```

**Latihan:** FizzBuzz, konversi suhu Celsius ke Fahrenheit, game tebak angka (baca input dari `std::io::stdin`).

## Blok 3 (jam 5–9): Ownership, borrowing, lifetime

Ini inti Rust. Alokasikan waktu paling banyak di sini.

### Kenapa ownership ada

Dua cara klasik ngelola memori: manual (C/C++, rawan bug) dan garbage collector (ada biaya runtime). Rust ambil cara ketiga: aturan yang dicek compiler, tanpa biaya runtime.

Singkatnya soal memori: **stack** cepat dan ukurannya harus diketahui pas compile. **Heap** fleksibel tapi butuh alokasi. `String` dan `Vec` nyimpen datanya di heap.

### Tiga aturan ownership

1. Setiap nilai punya satu **owner**.
2. Dalam satu waktu cuma ada satu owner.
3. Owner keluar dari scope, nilai di-**drop** (memori dibebaskan otomatis).

### Move

```rust
let a = String::from("halo");
let b = a;                  // ownership pindah ke b
// println!("{}", a);       // ERROR: a sudah di-move
```

Tipe kecil di stack (`i32`, `bool`, `char`, `f64`) implement **Copy**, jadi disalin otomatis. Tipe di heap di-**move**. Kalau mau salinan, tulis eksplisit `.clone()`.

> Kalau AI nulis `.clone()` di mana-mana, curiga. Sering itu cuma cara kabur dari borrow checker, dan clone di heap itu mahal.

Fungsi juga bisa ngambil ownership:

```rust
fn konsumsi(s: String) { }   // s pindah ke sini, di-drop di akhir fungsi
fn lihat(s: &String) { }     // cuma minjam
```

### Borrowing

- `&T`: shared reference, boleh banyak.
- `&mut T`: exclusive reference, cuma satu, dan nggak boleh ada `&T` barengan.

Ringkasnya: **banyak pembaca, atau satu penulis, tapi nggak keduanya sekaligus.**

```rust
let mut s = String::from("a");
let r1 = &s;
let r2 = &s;                 // oke, dua pembaca
// let w = &mut s;           // ERROR kalau r1 atau r2 masih dipakai setelahnya
println!("{} {}", r1, r2);
let w = &mut s;              // oke, r1 dan r2 sudah selesai dipakai
w.push('b');
```

Aturan ini persis kondisi terjadinya data race dan iterator invalidation. Rust ngelarangnya di level bahasa.

Reference ke variabel lokal yang sudah mati (dangling reference) juga ditolak compiler.

### Slice

`&str` dan `&[T]` adalah "view" ke sebagian data tanpa jadi owner-nya. Buat parameter fungsi, pilih `&str` daripada `&String`, dan `&[T]` daripada `&Vec<T>`. Lebih fleksibel.

### Lifetime

Lifetime itu hubungan umur antar reference, bukan durasi waktu. Hampir selalu di-infer compiler. Kamu baru perlu nulis eksplisit pas compiler nggak bisa tau reference hasil berasal dari input yang mana:

```rust
fn terpanjang<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

Bacanya: hasil nggak boleh hidup lebih lama dari yang terpendek di antara `x` dan `y`. Annotation ini nggak mengubah umur data, cuma ngasih tau compiler hubungannya.

Struct yang nyimpen reference butuh lifetime: `struct Parser<'a> { input: &'a str }`.

> Tips pemula: kalau lifetime mulai ribet, ubah desain. Pakai tipe yang punya data sendiri (`String`) dulu, optimasi belakangan.

### Drop dan RAII

File, lock, dan socket otomatis dilepas pas owner-nya keluar scope. Nggak ada `close()` manual yang bisa kelupaan.

### Error borrow checker yang paling sering muncul

| Kode | Artinya |
| --- | --- |
| E0382 | Pakai nilai yang sudah di-move |
| E0499 | Dua `&mut` ke data yang sama |
| E0502 | `&mut` dan `&` barengan |
| E0106 | Lifetime belum ditulis |
| E0597 | Reference hidup lebih lama dari datanya |

Baca bagian `help:` di pesan error, compiler sering udah ngasih solusinya.

**Latihan:** sengaja bikin kelima error di atas satu per satu, lalu benerin. Jelasin ke diri sendiri kenapa compiler nolak.

## Blok 4 (jam 9–12): struct, enum, pattern matching, error handling

### struct dan impl

```rust
struct Pengguna {
    nama: String,
    umur: u32,
}

impl Pengguna {
    fn baru(nama: &str, umur: u32) -> Self {
        Self { nama: nama.to_string(), umur }
    }
    fn sapa(&self) -> String {        // &self = minjam, read-only
        format!("Halo, {}", self.nama)
    }
    fn ulang_tahun(&mut self) {       // &mut self = boleh ngubah
        self.umur += 1;
    }
}
```

Tiga bentuk `self`, `&self`, `&mut self` langsung nyambung ke ownership dan borrowing.

### enum: fitur paling kuat di Rust

Setiap varian bisa bawa data yang beda-beda.

```rust
enum Event {
    Login { user: String },
    Logout(String),
    Error(u16, String),
    Tick,
}
```

`match` harus **exhaustive**, semua kemungkinan wajib ditangani:

```rust
fn deskripsi(e: &Event) -> String {
    match e {
        Event::Login { user } => format!("{} masuk", user),
        Event::Logout(u) => format!("{} keluar", u),
        Event::Error(kode, _) if *kode >= 500 => String::from("server error"),
        Event::Error(..) => String::from("error lain"),
        Event::Tick => String::from("tick"),
    }
}
```

Kalau kamu nambah varian baru, compiler nunjukin semua `match` yang perlu diupdate. Itu alasan refactor di Rust terasa aman. Pola pikirnya: desain tipe supaya **state yang ilegal nggak bisa dibuat**.

### Option dan Result

Rust nggak punya `null` dan nggak punya exception. Ketiadaan dan kegagalan jadi bagian dari tipe:

```rust
enum Option<T> { Some(T), None }
enum Result<T, E> { Ok(T), Err(E) }
```

```rust
fn bagi(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 { Err(String::from("pembagi nol")) } else { Ok(a / b) }
}

fn main() {
    match bagi(10.0, 0.0) {
        Ok(h) => println!("{}", h),
        Err(e) => println!("gagal: {}", e),
    }
}
```

### Operator `?`

```rust
use std::fs;
use std::io;

fn baca(path: &str) -> Result<String, io::Error> {
    let isi = fs::read_to_string(path)?;   // kalau Err, langsung di-return dari fungsi
    Ok(isi)
}
```

Alat bantu lain: `if let`, `let else`, `unwrap_or`, `map`, `and_then`, `ok_or`.

`unwrap()` dan `expect()` bikin program panic kalau `Err`/`None`. Boleh di prototipe dan test. Di kode beneran, itu red flag.

### Custom error

Aturan praktis: **library** pakai `thiserror`, **aplikasi/CLI** pakai `anyhow`.

### panic vs Result

- `panic!` = bug, ada invariant yang dilanggar.
- `Result` = kegagalan yang wajar terjadi (file nggak ada, input user salah, jaringan putus).

**Latihan:** modelkan baris log sebagai enum, tulis fungsi parse dari `&str` yang return `Result`, tangani baris yang formatnya rusak.

## Blok 5 (jam 12–15): trait, generic, closure, iterator

### Trait

Trait itu kumpulan perilaku, mirip interface.

```rust
trait Ringkas {
    fn ringkas(&self) -> String;
    fn judul(&self) -> String { String::from("tanpa judul") }  // implementasi default
}

struct Artikel { isi: String }

impl Ringkas for Artikel {
    fn ringkas(&self) -> String { self.isi.chars().take(20).collect() }
}
```

### Generic dan trait bound

```rust
fn cetak<T: Ringkas>(x: &T) { println!("{}", x.ringkas()); }
fn cetak2(x: &impl Ringkas) { println!("{}", x.ringkas()); }   // sama saja, lebih ringkas
fn cetak3<T>(x: &T) where T: Ringkas + std::fmt::Debug { }    // bound banyak
```

### Static dispatch vs dynamic dispatch

- **Generic / `impl Trait`**: compiler bikin satu versi fungsi per tipe (monomorphization). Cepat, binary agak membesar.
- **`dyn Trait`** (misal `Box<dyn Ringkas>`): lewat vtable pas runtime. Sedikit lebih lambat, tapi bisa nyimpen tipe campur dalam satu koleksi.

### Trait yang wajib kenal

- `#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]`: dibuatin otomatis.
- `Display` (print buat user), `Debug` (`{:?}`).
- `From` / `Into`: konversi antar tipe. `?` juga pakai ini buat konversi error.
- `Iterator`, `Drop`, `Deref`.
- `Send` / `Sync`: marker buat thread safety (Blok 7).
- `serde::Serialize` / `Deserialize`: baca/tulis JSON, TOML, dll.

### Closure

```rust
let tambah = |a, b| a + b;
let faktor = 3;
let kali = |x| x * faktor;   // nangkep `faktor` dari luar
```

Tiga jenis berdasarkan cara nangkepnya: `Fn` (minjam read-only), `FnMut` (minjam mutable), `FnOnce` (ambil ownership, cuma bisa dipanggil sekali). Keyword `move` memaksa closure ngambil ownership, dan wajib dipakai buat thread.

### Iterator

Iterator itu **lazy**: nggak ngerjain apa-apa sampai ada yang nge-consume (`collect`, `sum`, `for`).

```rust
let genap_kuadrat: Vec<u32> = (1..=10)
    .filter(|n| n % 2 == 0)
    .map(|n| n * n)
    .collect();
```

- `iter()` menghasilkan `&T`, `iter_mut()` menghasilkan `&mut T`, `into_iter()` menghasilkan `T` (konsumsi koleksinya).
- Adaptor yang sering dipakai: `map`, `filter`, `enumerate`, `zip`, `take`, `skip`, `flat_map`, `fold`, `sum`, `any`, `all`, `find`.
- `collect()` bisa ke `Vec`, `HashMap`, `String`, bahkan `Result<Vec<_>, _>`.

**Zero-cost abstraction**: iterator chain dikompilasi jadi kode yang secepat for loop manual. Jadi nulis gaya fungsional di Rust nggak bikin lambat.

**Latihan:** bikin trait `ParserLog` dengan dua implementasi (dua format log berbeda). Ganti for loop lama kamu jadi iterator chain.

## Blok 6 (jam 15–18): Collection, module, Cargo, testing

### Collection

`Vec`, `HashMap`, `HashSet`, `VecDeque`, `BTreeMap`.

```rust
let mut v = vec![1, 2, 3];
v.push(4);
let pertama = v.get(0);          // Option<&i32>, nggak panic kalau kosong

let mut hitung = std::collections::HashMap::new();
for kata in "a b a c a".split_whitespace() {
    *hitung.entry(kata).or_insert(0) += 1;
}
```

Hati-hati `v[i]`: kalau index di luar batas, program panic. Buat input dari luar, pakai `.get(i)`.

### String vs &str

- `String`: punya data sendiri, bisa tumbuh, di heap.
- `&str`: view ke teks. Literal `"halo"` bertipe `&'static str`.
- String itu UTF-8, jadi `s[0]` nggak boleh. Pakai `.chars()`, `.bytes()`, atau slicing yang bener.
- Parameter fungsi sebaiknya `&str`.

### Module dan visibility

- Semua private secara default, buka dengan `pub`.
- `mod parser;` di `main.rs` akan nyari `src/parser.rs`.
- `use` buat ngimpor nama biar nggak nulis path panjang.

### Cargo dan crate

- `Cargo.toml` nyimpen dependency, features, dan metadata. `Cargo.lock` ngunci versi.
- `cargo add serde` buat nambah dependency. Cari crate di crates.io, baca dokumentasinya di docs.rs.
- Workspace dipakai kalau satu repo punya beberapa crate (misal agent dan library bersama).

Crate yang wajib kenal:

| Crate | Buat apa |
| --- | --- |
| serde, serde\_json | Serialisasi JSON/TOML/YAML |
| clap | Parsing argumen CLI |
| tokio | Runtime async |
| anyhow, thiserror | Error handling |
| tracing | Logging terstruktur |
| reqwest | HTTP client |
| axum | Web framework |
| regex | Regular expression |
| rayon | Paralelisme data |

### Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bagi_nol_error() {
        assert!(bagi(1.0, 0.0).is_err());
    }
}
```

Jalanin dengan `cargo test`. Folder `tests/` buat integration test. Contoh di komentar dokumentasi (`///`) juga ikut dites.

**Latihan:** pecah proyek kamu jadi `main.rs`, `model.rs`, `parser.rs`. Tulis minimal tiga test buat parser.

## Blok 7 (jam 18–21): Smart pointer, concurrency, async, unsafe

### Smart pointer

| Tipe | Fungsi | Thread-safe |
| --- | --- | --- |
| `Box<T>` | Alokasi di heap, satu owner. Dipakai buat tipe rekursif dan `dyn Trait` | Ikut `T` |
| `Rc<T>` | Banyak owner lewat reference counting | Nggak |
| `Arc<T>` | Versi atomic dari `Rc` | Ya |
| `RefCell<T>` | Mutasi lewat `&T`, aturan borrow dicek pas runtime (panic kalau melanggar) | Nggak |
| `Mutex<T>`, `RwLock<T>` | Kunci yang ngasih akses mutable aman antar thread | Ya |

Pola umum: `Rc<RefCell<T>>` buat satu thread, `Arc<Mutex<T>>` buat banyak thread. Kalau AI nyodorin `Rc<RefCell<...>>` di mana-mana, biasanya desain datanya belum dipikirin.

### Concurrency

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let total = Arc::new(Mutex::new(0));
let mut handles = vec![];
for _ in 0..4 {
    let total = Arc::clone(&total);
    handles.push(thread::spawn(move || {
        let mut g = total.lock().unwrap();
        *g += 1;
    }));
}
for h in handles { h.join().unwrap(); }
println!("{}", *total.lock().unwrap());
```

**Send dan Sync** adalah kunci "fearless concurrency":

- `Send`: nilai boleh dipindah ke thread lain.
- `Sync`: `&T` boleh dibagi antar thread.
- `Rc` bukan `Send`, jadi kalau dipakai di thread, program nggak mau compile. Data race ditolak di compile time.

Alternatif berbagi state: **channel** (`std::sync::mpsc`, atau `tokio::sync::mpsc` di dunia async). Prinsipnya: lebih baik komunikasi lewat pesan daripada rebutan memori.

### Async

- `async fn` mengembalikan `Future`. Future itu lazy, baru jalan kalau di-`.await` dan dijalankan oleh runtime.
- Rust nggak punya runtime async bawaan. Standar de facto: **tokio**.

```rust
#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(ambil(1), ambil(2));   // jalan konkuren
    println!("{} {}", a, b);
}

async fn ambil(id: u32) -> u32 { id }
```

- **Pakai async** buat kerjaan I/O banyak dan konkuren: server, agent yang kirim data ke banyak tujuan, ribuan koneksi.
- **Pakai thread biasa atau rayon** buat kerjaan CPU-bound.
- Jangan nge-block di dalam async (misal `std::thread::sleep` atau baca file besar secara sync). Pakai `tokio::time::sleep` atau `spawn_blocking`.
- Jebakan klasik: nahan `std::sync::MutexGuard` melewati `.await`.

### unsafe

`unsafe` ngasih lima kemampuan tambahan: dereference raw pointer, panggil fungsi unsafe, akses `static mut`, implement unsafe trait, dan akses field union. Ini **bukan** mematikan borrow checker. Artinya kamu yang njamin invariant-nya, bukan compiler.

Dipakai buat FFI (manggil library C) dan struktur data low-level. Prinsipnya: kurung `unsafe` sekecil mungkin dan bungkus dengan API yang safe. Kalau AI nulis `unsafe` tanpa alasan jelas, tolak.

**Latihan:** hitung jumlah kata di beberapa file secara paralel pakai thread plus `Arc<Mutex>`, lalu ulangi pakai channel. Setelah itu bikin versi tokio yang manggil beberapa fungsi async (simulasi pakai `tokio::time::sleep`).

## Blok 8 (jam 21–24): Proyek akhir dan workflow vibe coding

### Proyek: CLI log analyzer

Spesifikasi:

1. Baca file log, tiap baris misalnya `2026-10-16T10:00:01 ERROR auth login gagal user=budi`.
2. Parse ke `struct LogEntry { waktu, level, sumber, pesan }`, dengan `level` berupa enum.
3. Baris yang rusak menghasilkan `Err` yang dihitung dan dilaporkan di akhir, bukan panic.
4. Statistik: jumlah per level (`HashMap`), 5 sumber teratas, filter lewat argumen CLI (`--level error`) pakai `clap`.
5. Minimal tiga test buat parser.
6. Bonus: output JSON pakai `serde_json`, mode `tail -f` pakai tokio, atau proses file besar paralel pakai `rayon`.

### Workflow vibe coding di Rust

1. **Mulai dari tipe.** Minta AI bikin `struct`, `enum`, dan signature fungsi dulu (tanpa isi). Review. Baru isi body-nya. Di Rust, desain tipe itu desain program.
2. **Kerjain kecil-kecil**, `cargo check` tiap selesai satu langkah.
3. **Baca error compiler dulu**, tebak penyebabnya, baru tanya AI. Kalau nanya, tempel error utuh plus kode terkait, dan minta penjelasan penyebab, bukan cuma fix.
4. **Jalanin `cargo clippy`** setelah AI selesai nulis. Banyak kode "jalan tapi jelek" ketahuan di sini.
5. **Minta test** bareng kodenya.

Template prompt:

```
Aku lagi belajar Rust. Tulis fungsi X dengan aturan: tanpa unwrap/expect, error pakai Result dan thiserror, parameter pakai &str atau &[T], jangan clone kecuali perlu dan jelasin alasannya. Sertakan unit test.
```

### Checklist review kode Rust dari AI

- Ada `unwrap()` atau `expect()` di jalur normal?
- `.clone()` berlebihan?
- `Rc<RefCell>` atau `Arc<Mutex>` tanpa alasan jelas?
- `unsafe` tanpa komentar yang nerangin kenapa aman?
- Lifetime ribet padahal bisa pakai tipe yang punya data sendiri?
- Ada blocking call di dalam fungsi async?
- `v[i]` atau slicing di input dari luar (bisa panic)?
- Dependency yang nggak dikenal? Cek dulu di crates.io: jumlah download, maintainer, umur crate.

## Konsep mendalam yang harus nempel

1. **Ownership**: tiap data punya satu owner, owner selesai berarti drop.
2. **Borrowing**: banyak `&T`, atau satu `&mut T`.
3. **Lifetime**: hubungan umur antar reference, bukan durasi waktu.
4. **enum + match**: modelkan state valid saja, state ilegal nggak bisa dibuat.
5. **Option / Result**: ketiadaan dan kegagalan jadi bagian dari tipe.
6. **Trait dan generic**: generic jadi monomorphization (cepat), `dyn` jadi vtable (fleksibel).
7. **Zero-cost abstraction**: abstraksi tingkat tinggi nggak nambah biaya runtime.
8. **Interior mutability**: ngubah lewat `&T`, pengecekan pindah ke runtime.
9. **Send / Sync**: keamanan thread dijaga type system.
10. **unsafe**: kontrak manual di dalam kotak kecil.

## Kesalahan umum pemula

- Berantem sama borrow checker pakai `clone` atau `Rc` terus-terusan. Solusi yang lebih sehat: ubah struktur data, pecah fungsi, atau kirim ID/index alih-alih reference.
- Nyimpen reference di dalam struct terlalu cepat. Mulai pakai `String` dulu.
- Semua parameter pakai `String` padahal `&str` cukup.
- `unwrap()` di mana-mana.
- Baca tutorial tanpa ngetik.

## Sumber belajar

- **The Rust Programming Language** ("the book"), gratis di doc.rust-lang.org/book. Target: bab 1–10 selama 24 jam ini.
- **Rust by Example**: contoh kode per topik.
- **Rustlings**: latihan kecil berbasis error, cocok buat ngelatih baca compiler.
- **Comprehensive Rust** (Google): kursus Rust yang rapi dan padat.
- **Crust of Rust** (Jon Gjengset, YouTube): buat ngerti konsep dalam kayak lifetime, iterator, dan smart pointer.
- **Tokio tutorial**: buat async.
- **docs.rs** dan **crates.io**: buat baca dokumentasi dan milih crate.
