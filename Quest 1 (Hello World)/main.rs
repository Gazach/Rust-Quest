fn main() {
    println!("Hello dunia!");
}

#[test]
fn hello_unittest() {
    println!("Helo test!");
}

#[test]
fn letnames() {
    let name= "Gaza lazuardi";
    let live = "Bekasi";
    println!("Nama saya adalah {} dan saya tinggal di {}", name, live);
}

#[test]
fn variable_learn() {
    let constname = "Gaza lazuardi chandra"; // data tidak bisa di ubah
    println!("Hello {constname}");

    let mut name = "John thor"; // data bisa di ubah
    println!("Hello {name}");

    name = "John due"; // data terubah
    println!("Hello {name}");

    //note : variable tidak bisa di ubah jenis datanya, contoh jika varibale berdata "string" maka tidak bisa di ubah menjadi "integer" karena rust menggunakan static typing

}

#[test]
fn datatype_operation() {
    let a = 100;
    let b = 10;
    // kalkulasi
    let c = a + b;
    println!("{}", c);
    let d = a - b;
    println!("{}", d);
    let e = a * b;
    println!("{}", e);
    let f = a / b;
    println!("{}", f);
    let g = a % b;
    println!("{}", g);

    let mut a2 = 10;
    println!("Data lama {}", a2);

    a2 *= 10;

    println!("data baru {}", a2);
}

#[test]
fn booleandatatype() {
    let a = true;
    let b = false;
    println!("{} , {}", a , b);
    println!("\n");
    if a == false {
         println!("Nilai A = True!");
    } else {
        println!("Nilai A = Bukan True!");
    }
}

#[test]
fn operator_perbandingan() {
    let a = 10;
    let b = 5;
    if a > b {
        println!("Ya A lebih dari B");
    }
    // cara lebih mudah
    let mut result: bool = a > b;
    println!("{}", result);
    result = a < b;
    println!("{}", result);
    result = a >= b;
    println!("{}", result);
    result = a <= b;
    println!("{}", result);
    result = a == b;
    println!("{}", result);
}

#[test]
fn operator_boolian() {
    let nilai_semester = 60;
    let nilai_akhir = 75;

    let nilai_lulus = nilai_semester >= 75;
    let nilai_akhir_lulus =  nilai_akhir >= 75;

    let lulus = nilai_lulus && nilai_akhir_lulus;
    println!("Nilai = {}", lulus);

    if !lulus {
        println!("Anda tidak lulus!");
    } else {
        println!("Anda Lulus");
    }
}