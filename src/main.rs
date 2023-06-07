use rand::Rng;
fn main() {
    let mut data = vec![];

    for _ in 0..10 {
        data.push(random());
    }

    print_all(data);
}

fn random() -> Option<i32> {
    let n = rand::thread_rng().gen_range(0..10);
    match n {
        0 => None,
        _ => Some(n),
    }
}

fn print_all(data: Vec<Option<i32>>) {
    for item in data {
        let res = print(item);
        match res {
            Ok(s) =>println!("---{}---", s),
            Err(k) => match k {
                ErrKind::Caution => println!("***Caution***"),
                ErrKind::Danger => {
                    println!("Danger!!");
                    panic!("Danger ERROR.");
                },
            }
        }
    }
}

fn print(item: Option<i32>) -> Result<String, ErrKind>{
    match item {
        None => {
            Err(ErrKind::Danger)
        }
        Some(n) => {
            println!("No, {}", n);
            if n == 1 {
                Err(ErrKind::Caution)
            } else {
                Ok("OK".to_string())
            }
        }
    }
}

enum ErrKind {
    Caution,
    Danger,
}
