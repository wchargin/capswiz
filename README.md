# capswiz

Given a base64 string whose case information has been lost (e.g., it's
been converted to all lowercase), tries to find a recapitalization that
represents plausible English text.

## What?

For example, take the payload `Sphinx of black quartz, judge my vow.`.
Its base64 encoding is `U3BoaW54IG9mIGJsYWNrIHF1YXJ0eiwganVkZ2UgbXkgdm93Lg==`:

```
$ printf 'Sphinx of black quartz, judge my vow.' | base64
U3BoaW54IG9mIGJsYWNrIHF1YXJ0eiwganVkZ2UgbXkgdm93Lg==
$ printf U3BoaW54IG9mIGJsYWNrIHF1YXJ0eiwganVkZ2UgbXkgdm93Lg== | base64 -d; echo
Sphinx of black quartz, judge my vow.
```

But if we're given only the lowercase form of the base64, we get gibberish:

```
$ printf 'Sphinx of black quartz, judge my vow.' | base64 | tr A-Z a-z
u3boaw54ig9migjsywnrihf1yxj0eiwganvkz2ugbxkgdm93lg==
$ printf u3boaw54ig9migjsywnrihf1yxj0eiwganvkz2ugbxkgdm93lg== | base64 -d; echo
�v�kx�f�        �����z, j{��k�o vow�
$ printf u3boaw54ig9migjsywnrihf1yxj0eiwganvkz2ugbxkgdm93lg== | base64 -d | xxd
00000000: bb76 e86b 0e78 8a0f 668a 08ec cb09 eb8a  .v.k.x..f.......
00000010: 17f5 cb18 f47a 2c20 6a7b e4cf 6ba0 6f19  .....z, j{..k.o.
00000020: 2076 6f77 96                              vow.
```

Run that through `capswiz` to find the answer:

```
$ cargo build --release
$ ./target/release/capswiz u3boaw54ig9migjsywnrihf1yxj0eiwganvkz2ugbxkgdm93lg==
[2022-01-30T21:27:48.797446Z INFO  capswiz] guess: b"SpN\x03\x0ex\x8a\x0ff\x88h\xd2\xc9cQ w\xf5\xcb\x18\xf4\x10\x85\x86judge \x07\x19\x06\x0eow\x96" (base64: b"U3BOAw54ig9miGjSyWNRIHf1yxj0EIWGanVkZ2UgBxkGDm93lg==", score: -1730)
[2022-01-30T21:27:48.797502Z INFO  capswiz] guess: b"Sv\xcek\x0ex\x8a\x0fL\x88bl\xc9i\xeb"\x17\xf5c\x12t\x10\x8c\x06h\xd5dgk\xa0mr\x86\x0c\xcfw." (base64: b"U3bOaw54ig9MiGJsyWnrIhf1YxJ0EIwGaNVkZ2ugbXKGDM93Lg==", score: -1525)
[2022-01-30T21:27:48.797513Z INFO  capswiz] guess: b"Sv\xe8\x01nx"\x0ff bR\xcb\x03Q"\x11uc\x12tx\x85\xa0h\xdb\xe4\xcfe \x05y vow\x96" (base64: b"U3boAW54Ig9mIGJSywNRIhF1YxJ0eIWgaNvkz2UgBXkgdm93lg==", score: -1205)
[2022-01-30T21:27:48.797521Z INFO  capswiz] guess: b"Sph\x01nx\x8a\x0ff\x88blai\xd1\x8a\x11u\xc9x\xf4z,\x06juJ\xcfk\xa0\x05y \x0eow." (base64: b"U3BoAW54ig9miGJsYWnRihF1yXj0eiwGanVKz2ugBXkgDm93Lg==", score: -1195)
[2022-01-30T21:27:48.797561Z INFO  capswiz] guess: b"\xbbph\x03\x0ex"\x0ff h\xd2ai\xd1\x8a\x11u\xc9rt\x10\x8c judge \x05y\x06vow." (base64: b"u3BoAw54Ig9mIGjSYWnRihF1yXJ0EIwganVkZ2UgBXkGdm93Lg==", score: -845)
[2022-01-30T21:27:48.797739Z INFO  capswiz] guess: b"\xbbphinx"\x0ff bR\xc9i\xeb"\x17\xf5artz, \x02{\xe4ge\x06mr\x86vow." (base64: b"u3BoaW54Ig9mIGJSyWnrIhf1YXJ0eiwgAnvkZ2UGbXKGdm93Lg==", score: -790)
[2022-01-30T21:27:48.797895Z INFO  capswiz] guess: b"Sph\x01nx of\x88bR\xcb\x09\xd1"\x11uc\x18\xf4z, juJgk\x86my\x06vow." (base64: b"U3BoAW54IG9miGJSywnRIhF1Yxj0eiwganVKZ2uGbXkGdm93Lg==", score: -665)
[2022-01-30T21:27:48.798163Z INFO  capswiz] guess: b"SpNinx\x8a\x0fL bR\xc9cQ"\x11uartx\x8c juJge o\x19 t\xcfw\x96" (base64: b"U3BOaW54ig9MIGJSyWNRIhF1YXJ0eIwganVKZ2UgbxkgdM93lg==", score: -540)
[2022-01-30T21:27:48.799472Z INFO  capswiz] guess: b"Sphinx of h\xd2c\x03Q\x88qu\xcb\x18\xf4\x12,\x06judgk\x86my vow." (base64: b"U3BoaW54IG9mIGjSYwNRiHF1yxj0EiwGanVkZ2uGbXkgdm93Lg==", score: -255)
[2022-01-30T21:27:48.811905Z INFO  capswiz] guess: b"SpNinx of\x88blacQ quartx\x85\x86juJge\x06mr\xa0vow." (base64: b"U3BOaW54IG9miGJsYWNRIHF1YXJ0eIWGanVKZ2UGbXKgdm93Lg==", score: -200)
[2022-01-30T21:27:48.812565Z INFO  capswiz] guess: b"Sphk\x0ex of blai\xeb quartz,\x06juJ\xcfe o\x19 vow." (base64: b"U3Boaw54IG9mIGJsYWnrIHF1YXJ0eiwGanVKz2Ugbxkgdm93Lg==", score: -125)
[2022-01-30T21:27:48.817577Z INFO  capswiz] guess: b"Sphinx of h\xd2acQ\x88quc\x12tz%\xa0\x02udgk\x86mr\x86vow." (base64: b"U3BoaW54IG9mIGjSYWNRiHF1YxJ0eiWgAnVkZ2uGbXKGdm93Lg==", score: -30)
[2022-01-30T21:27:48.818279Z INFO  capswiz] guess: b"Sphinx of bRacQ"\x11uc\x18\xf4z,\x06juJgk\x86my vow." (base64: b"U3BoaW54IG9mIGJSYWNRIhF1Yxj0eiwGanVKZ2uGbXkgdm93Lg==", score: 175)
[2022-01-30T21:27:48.836599Z INFO  capswiz] guess: b"Sph\x01nx\x88oL black quartz, \x02uJge my vow." (base64: b"U3BoAW54iG9MIGJsYWNrIHF1YXJ0eiwgAnVKZ2UgbXkgdm93Lg==", score: 345)
[2022-01-30T21:27:48.841638Z INFO  capswiz] guess: b"Sphinx of black w\xf5artx\x8c \x02uJgk\xa0my vow." (base64: b"U3BoaW54IG9mIGJsYWNrIHf1YXJ0eIwgAnVKZ2ugbXkgdm93Lg==", score: 545)
[2022-01-30T21:27:48.848298Z INFO  capswiz] guess: b"SpN\x01nx oL black quartz, judge o\x19 vow." (base64: b"U3BOAW54IG9MIGJsYWNrIHF1YXJ0eiwganVkZ2Ugbxkgdm93Lg==", score: 600)
[2022-01-30T21:27:48.850972Z INFO  capswiz] guess: b"Sphinx of black w\xf5artz, \x02uJgk\xa0my vow." (base64: b"U3BoaW54IG9mIGJsYWNrIHf1YXJ0eiwgAnVKZ2ugbXkgdm93Lg==", score: 645)
[2022-01-30T21:27:48.851958Z INFO  capswiz] guess: b"Sphinx oL blacQ quartz, judge \x05y\x06vow." (base64: b"U3BoaW54IG9MIGJsYWNRIHF1YXJ0eiwganVkZ2UgBXkGdm93Lg==", score: 655)
[2022-01-30T21:27:48.860733Z INFO  capswiz] guess: b"Sphinx oL black quc\x12tz, judge \x05y\x06t\xcfw." (base64: b"U3BoaW54IG9MIGJsYWNrIHF1YxJ0eiwganVkZ2UgBXkGdM93Lg==", score: 685)
[2022-01-30T21:27:48.862670Z INFO  capswiz] guess: b"Sphinx of"\x02lc\x03k quartz, judge my vow." (base64: b"U3BoaW54IG9mIgJsYwNrIHF1YXJ0eiwganVkZ2UgbXkgdm93Lg==", score: 745)
[2022-01-30T21:27:48.863529Z INFO  capswiz] guess: b"Sphinx of black quc\x12tz, judge my vow." (base64: b"U3BoaW54IG9mIGJsYWNrIHF1YxJ0eiwganVkZ2UgbXkgdm93Lg==", score: 1210)
[2022-01-30T21:27:48.875317Z INFO  capswiz] guess: b"Sphinx of black quartz, judge my vow." (base64: b"U3BoaW54IG9mIGJsYWNrIHF1YXJ0eiwganVkZ2UgbXkgdm93Lg==", score: 1320)
```

And, just like that, the original base64 is recovered in 77 ms on my
laptop (a 2014 ThinkPad T440s, Intel i5-4300U).

## How?

Simple brute-force genetic algorithm. Not a lot of polish, but it's
easily fast enough, so not a lot of polish needed.

Each candidate decoding is scored:

  - Individual bytes are rewarded for being alphanumeric and penalized
    for being control characters or non-ASCII. (Yes, this is optimized
    for ASCII only. Sorry.)
  - Trigrams are rewarded in proportion to their frequency across all
    words in a dictionary.
  - Each letter that appears as part of a word is also highly rewarded
    (which weakly biases toward fewer, longer words, since adding a
    space drops the number of rewarded letters).

A search starts with the input base64 and keeps track of its
highest-scoring capitalization seen so far. At each iteration, the
search flips i.i.d. coins to decide whether to flip the case of each
letter. The probability of flipping anneals down over time, but is
bounded below by a positive constant. If the candidate decoding has a
new high score, it replaces the previous best.

To improve convergence, the process runs a small constant number of
search "heads" in parallel. At each iteration, each letter has a chance
to be copied from a different head. This way, heads can pull each other
out of local maxima. With only one head, the time to convergence tends
to look like a bimodal distribution mostly centered around ~100ms but
with a ~20% chance of failing to converge within 10s. Using 16 heads
dramatically increases the probability of convergence, at some cost to
expected runtime.

The implementation doesn't have any fancy tricks to incrementally update
the base64 decodings or scores, or anything like that. We just rely on
the core primitives being fast.

## Alternatives considered

You could try searching for your base64 string on the internet using a
case-insensitive search engine. If it's been seen before, you might get
the answer that way. However, depending on the latency of your search
engine and your internet connection, this search may well take longer
than the ~100ms required by this tool. :-)

## Why?

My friend was amazed that I could decode a string like this, and called
it "wizardry" (hence, `capswiz`). I really decoded it just by cheating:
the lowercase version of the base64 was basically the key in a K-V
database whose values included the properly cased version, so I just
looked it up. But I thought that it would be fun to rig up for real.

And, why not?
