use argon2::{self, Config};

fn main() {
    println!("argon2!");
    let password = b"password";
    let salt = b"somesalt";
    let config = Config {
        ad: &[],
        hash_length: 32,
        lanes: 1,
        mem_cost: 1024,
        secret: &[],
        thread_mode: argon2::ThreadMode::Sequential,
        time_cost: 1,
        variant: argon2::Variant::Argon2id,
        version: argon2::Version::Version13,
    };
    println!("{:?}", config);
    let encoded = argon2::hash_encoded(password, salt, &config).unwrap();
    println!("{}", encoded);
    let hash = argon2::hash_raw(password, salt, &config).unwrap();
    println!(
        "{}",
        hash.iter()
            .map(|x| format!("{:02x}", x))
            .collect::<String>()
    );
    let matches = argon2::verify_encoded(&encoded, password).unwrap();
    assert!(matches);
}

pub fn argon2_gen_hash(
    password: &[u8; 8],
    salt: &[u8; 8],
    hash_length: u32,
    lanes: u32,
    mem_cost: u32,
    time_cost: u32,
) -> String {
    argon2::hash_raw(
        password,
        salt,
        &Config {
            ad: &[],
            hash_length,
            lanes,
            mem_cost,
            secret: &[],
            thread_mode: argon2::ThreadMode::Sequential,
            time_cost,
            variant: argon2::Variant::Argon2id,
            version: argon2::Version::Version13,
        },
    )
    .unwrap()
    .iter()
    .map(|x| format!("{:02x}", x))
    .collect::<String>()
}

#[test]
fn argon2_gen_hash_test() {
    assert_eq!(
        argon2_gen_hash(b"password", b"somesalt", 32, 1, 1024, 1),
        "c8e9aedc956f6a7dff0a4d42940df628623f328ea1235005abac933c57093e23"
    );
}

#[test]
fn argon2_test() {
    let password = b"password";
    let salt = b"somesalt";
    let config = Config {
        ad: &[],
        hash_length: 32,
        lanes: 1,
        mem_cost: 1024,
        secret: &[],
        thread_mode: argon2::ThreadMode::Sequential,
        time_cost: 1,
        variant: argon2::Variant::Argon2id,
        version: argon2::Version::Version13,
    };
    let encoded = argon2::hash_encoded(password, salt, &config).unwrap();
    assert_eq!(
        encoded,
        "$argon2id$v=19$m=1024,t=1,p=1$c29tZXNhbHQ$yOmu3JVvan3/Ck1ClA32KGI/Mo6hI1AFq6yTPFcJPiM"
    );
    let hash = argon2::hash_raw(password, salt, &config).unwrap();
    println!(
        "{}",
        hash.iter()
            .map(|x| format!("{:02x}", x))
            .collect::<String>()
    );
    assert_eq!(
        hash.iter()
            .map(|x| format!("{:02x}", x))
            .collect::<String>(),
        "c8e9aedc956f6a7dff0a4d42940df628623f328ea1235005abac933c57093e23"
    );
    let matches = argon2::verify_encoded(&encoded, password).unwrap();
    assert!(matches);
}
