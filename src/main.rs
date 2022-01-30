use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;

use clap::Parser;
use rand::{
    distributions::{Bernoulli, Distribution},
    seq::SliceRandom,
};

#[derive(Parser)]
struct Opts {
    haystack: String,
    #[clap(long, default_value = "/usr/share/dict/words")]
    words: PathBuf,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
struct Trigram([u8; 3]);

impl<'a> std::convert::TryFrom<&'a [u8]> for Trigram {
    type Error = <[u8; 3] as TryFrom<&'a [u8]>>::Error;
    fn try_from(x: &'a [u8]) -> Result<Self, <Self as TryFrom<&'a [u8]>>::Error> {
        Ok(Trigram(x.try_into()?))
    }
}

fn debug_bytestring(bs: &[u8]) -> String {
    let mut result = String::with_capacity(3 + bs.len() * 3);
    result.push_str("b\"");
    for &b in bs {
        if b == b' ' || b.is_ascii_graphic() {
            result.push(b as char);
        } else {
            result.push_str(&format!("\\x{:02x}", b));
        }
    }
    result.push('"');
    result
}

impl fmt::Debug for Trigram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Trigram({})", &debug_bytestring(&self.0))
    }
}

impl Trigram {
    fn trigrams(word: &[u8]) -> impl Iterator<Item = Trigram> + '_ {
        word.windows(3)
            .map(|t| Trigram::try_from(t).expect("trigrams: bad window size"))
    }
}

#[derive(Debug)]
struct Corpus<'a> {
    words: HashSet<&'a [u8]>,
    trigrams: Trigrams,
}

#[derive(Debug)]
struct Trigrams {
    freqs: HashMap<Trigram, usize>,
    total: usize,
}

impl<'a> std::iter::FromIterator<&'a [u8]> for Corpus<'a> {
    fn from_iter<T: IntoIterator<Item = &'a [u8]>>(iter: T) -> Self {
        let mut freqs: HashMap<Trigram, usize> = HashMap::new();
        let mut total = 0;
        let mut words = HashSet::new();
        for word in iter {
            words.insert(word);
            for trigram in Trigram::trigrams(word) {
                *freqs.entry(trigram).or_default() += 1;
                total += 1;
            }
        }
        Corpus {
            words,
            trigrams: Trigrams { freqs, total },
        }
    }
}

fn score(guess: &[u8], corpus: &Corpus<'_>) -> i64 {
    let mut guess = guess.to_vec();
    guess.make_ascii_lowercase();

    let mut score: i64 = 0;
    for b in &guess {
        score += if b.is_ascii_alphanumeric() {
            10
        } else if b.is_ascii_whitespace() {
            5
        } else if b.is_ascii_control() {
            -100
        } else {
            0
        };
    }

    for word in guess.split(|&b| b.is_ascii_whitespace()) {
        if corpus.words.contains(word) {
            score += 50 * (word.len() as i64);
        }
    }

    for trigram in Trigram::trigrams(&guess) {
        let freq = *corpus.trigrams.freqs.get(&trigram).unwrap_or(&0);
        score += (freq as i64 * 10) / (corpus.trigrams.total as i64);
    }

    score
}

#[derive(Clone)]
struct Search {
    haystack: Vec<u8>,
    guess: Vec<u8>,
    score: i64,
}

impl Search {
    fn copy_from(&mut self, other: &Self) {
        self.clear();
        self.haystack.extend(&other.haystack);
        self.guess.extend(&other.guess);
        self.score = other.score;
    }

    fn clear(&mut self) {
        self.haystack.clear();
        self.guess.clear();
        self.score = i64::MIN;
    }
}

fn main() -> io::Result<()> {
    let opts = Opts::parse();
    dbg!(&opts.haystack);

    let mut words_bytes = Vec::new();
    BufReader::new(File::open(&opts.words)?).read_to_end(&mut words_bytes)?;
    words_bytes.make_ascii_lowercase();
    let corpus: Corpus<'_> = words_bytes
        .split(|&x| x == b'\n')
        .filter(|x| !x.is_empty())
        .collect();

    let mut best = {
        let haystack = opts.haystack.clone().into_bytes();
        let mut guess: Vec<u8> = Vec::new();
        base64::decode_config_buf(&haystack, base64::STANDARD, &mut guess).unwrap();
        let score = score(&guess, &corpus);
        Search {
            haystack,
            guess,
            score,
        }
    };

    const N_HEADS: usize = 4;
    let mut heads = Vec::with_capacity(N_HEADS);
    for _ in 0..heads.capacity() {
        heads.push(best.clone());
    }

    let mut p_flip = 0.5;
    let p_inherit = 0.01;
    let should_inherit = Bernoulli::new(p_inherit).unwrap();
    let mut rng = rand::thread_rng();

    let mut scratch = Search {
        haystack: Vec::with_capacity(best.haystack.len()),
        guess: Vec::with_capacity(best.guess.len()),
        score: i64::MIN,
    };
    for head_index in (0..heads.len()).into_iter().cycle() {
        scratch.clear();
        scratch.haystack.extend(&heads[head_index].haystack);

        let should_flip = Bernoulli::new(p_flip).unwrap();
        for (i, b) in scratch.haystack.iter_mut().enumerate() {
            if should_inherit.sample(&mut rng) {
                let head = SliceRandom::choose(&heads[..], &mut rng).unwrap();
                *b = head.haystack[i];
            }
            if b.is_ascii_alphabetic() && should_flip.sample(&mut rng) {
                *b ^= 0x20;
            }
        }

        match base64::decode_config_buf(&scratch.haystack, base64::STANDARD, &mut scratch.guess) {
            Ok(()) => (),
            Err(base64::DecodeError::InvalidLastSymbol(_, _)) => println!(
                "{} has invalid last byte; skipping",
                debug_bytestring(&scratch.haystack)
            ),
            Err(e) => panic!("decoding {}: {:?}", debug_bytestring(&scratch.haystack), e),
        };
        p_flip = (p_flip * (0.999f64.powf(1.0 / N_HEADS as f64))).max(0.01);

        scratch.score = score(&scratch.guess, &corpus);
        println!(
            "{} -> {}: {} -> {}{} || best: {}",
            best.score,
            scratch.score,
            debug_bytestring(&best.guess),
            debug_bytestring(&scratch.guess),
            if scratch.score > best.score {
                " (improved!)"
            } else {
                ""
            },
            debug_bytestring(&scratch.haystack),
        );
        if scratch.score > best.score {
            best.copy_from(&scratch);
        }
        if scratch.score > heads[head_index].score {
            heads[head_index].copy_from(&scratch);
        }
    }
    unreachable!()
}
