use bech32::{ToBase32, Variant};
use clap::Parser;
use secp256k1::rand::thread_rng;
use secp256k1::Secp256k1;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::SystemTime;

const NOSTR_BECH32_PUBLIC_KEY_PREFIX: &str = "npub";
const NOSTR_BECH32_SECRET_KEY_PREFIX: &str = "nsec";
// Bech32 encoding valid character set
// `CHARSET` is private in `bech32` crate, so define it here
// ref: https://github.com/rust-bitcoin/rust-bech32
const CHARSET: [char; 32] = [
    'q', 'p', 'z', 'r', 'y', '9', 'x', '8', //  +0
    'g', 'f', '2', 't', 'v', 'd', 'w', '0', //  +8
    's', '3', 'j', 'n', '5', '4', 'k', 'h', // +16
    'c', 'e', '6', 'm', 'u', 'a', '7', 'l', // +24
];

#[derive(Parser)]
struct Args {
    /// The address prefix to match
    #[arg(short, long)]
    prefix: String,

    /// The address suffix to match (optional)
    #[arg(short, long)]
    suffix: Option<String>,

    /// Cpu cores to use (default: cpu_cores/2)
    #[arg(short, long)]
    cores: Option<usize>,
}

fn worker(prefix: String, suffix: String, counter: Arc<AtomicU64>, exit_flag: Arc<AtomicBool>) {
    let mut local_count = 0;
    let secp = Secp256k1::new();
    let timer = SystemTime::now();
    loop {
        if exit_flag.load(Ordering::SeqCst) {
            break;
        }
        let (secret_key, public_key) = secp.generate_keypair(&mut thread_rng());
        let nostr_pub = bech32::encode(
            NOSTR_BECH32_PUBLIC_KEY_PREFIX,
            public_key.x_only_public_key().0.serialize().to_base32(),
            Variant::Bech32,
        )
        .unwrap();
        if nostr_pub.starts_with(&prefix) {
            if suffix.is_empty() || nostr_pub.ends_with(&suffix) {
                let data = secret_key.secret_bytes().to_base32();
                let nostr_sec =
                    bech32::encode(NOSTR_BECH32_SECRET_KEY_PREFIX, data, Variant::Bech32).unwrap();
                println!("\n[!] Result:");
                println!("secret_key:  {} (hex)", secret_key.display_secret());
                println!("secret_key:  {}", nostr_sec);
                println!("public_key:  {}", nostr_pub);
                exit_flag.store(true, Ordering::SeqCst);
                break;
            }
        }

        local_count += 1;
        let elapsed = timer.elapsed().unwrap().as_secs();
        if elapsed % 5 == 0 {
            counter.fetch_add(local_count, Ordering::SeqCst);
            local_count = 0;
        }
    }
}

fn main() {
    let args = Args::parse();

    let ref prefix = args.prefix;
    for c in prefix.chars() {
        if !CHARSET.contains(&c) {
            eprintln!("prefix {} contains invalid bech32 character: {}", prefix, c);
            return;
        }
    }

    let ref suffix = args.suffix.unwrap_or_default();
    for c in suffix.chars() {
        if !CHARSET.contains(&c) {
            eprintln!("suffix {} contains invalid bech32 character: {}", suffix, c);
            return;
        }
    }

    let suffix_len = suffix.len();
    let difficulty = 32.0_f64.powi((prefix.len() + suffix_len) as i32);
    let prefix = NOSTR_BECH32_PUBLIC_KEY_PREFIX.to_string() + "1" + prefix;
    if suffix_len == 0 {
        print!(
            "[#] Start searching with prefix {} (difficulty est.: {})",
            prefix, difficulty
        );
    } else {
        print!(
            "[#] Start searching with prefix {} and suffix {} (difficulty est.: {})",
            prefix, suffix, difficulty
        );
    }
    std::io::stdout().flush().unwrap();

    let cores = num_cpus::get();
    let thread_num = {
        let input = args.cores.unwrap_or(0);
        if input == 0 {
            cores / 2
        } else if input > cores {
            cores
        } else {
            input
        }
    };

    let counter = Arc::new(AtomicU64::new(0));
    let exit_flag = Arc::new(AtomicBool::new(false));

    let mut threads = Vec::new();
    for _ in 0..thread_num {
        let counter = counter.clone();
        let exit_flag = exit_flag.clone();
        let prefix = prefix.clone();
        let suffix = suffix.clone();
        let thread = std::thread::spawn(move || worker(prefix, suffix, counter, exit_flag));
        threads.push(thread);
    }

    let report_thread = std::thread::spawn(move || {
        let time = SystemTime::now();
        let mut last_c = 0;
        let mut last_t = 0;
        loop {
            if exit_flag.load(Ordering::SeqCst) {
                break;
            }
            let total_count = counter.load(Ordering::SeqCst);
            let elapsed = time.elapsed().unwrap().as_secs_f64();
            let elapsed_secs = elapsed as u64;
            if elapsed_secs % 30 == 0 {
                if last_t == elapsed_secs {
                    continue;
                }
                if elapsed_secs == 30 {
                    print!("\n");
                }
                let speed = (total_count - last_c) as f64 / 30.0;
                print!(
                    "\r[+] Total {} keys in {:.1} mins ({:.1} keys/s)",
                    total_count,
                    elapsed / 60.0,
                    speed
                );
                std::io::stdout().flush().unwrap();
                last_c = total_count;
                last_t = elapsed_secs;
            }
        }
    });
    threads.push(report_thread);
    for thread in threads {
        thread.join().unwrap();
    }
}
