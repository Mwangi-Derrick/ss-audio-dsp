# ss-audio-dsp

**Signals & Systems, made audible.** A Rust project that maps the core theory of Signals and Systems onto real audio processing, one concept at a time.

![language](https://img.shields.io/badge/language-Rust-orange)
![status](https://img.shields.io/badge/status-in%20progress-yellow)
![course](https://img.shields.io/badge/course-Siganls&Systems-blue)

> Every time you play an MP3, join a video call, or put on noise-cancelling headphones, a chip is running the math from your Signals & Systems lectures. This repo implements that math from scratch, so the equations stop being symbols on a page and become sound you can hear and plots you can inspect.

---

## Table of contents

1. [Who this is for](#who-this-is-for)
2. [Theory to code map](#theory-to-code-map)
3. [Quick start](#quick-start)
4. [Usage](#usage)
5. [The four modules](#the-four-modules)
   - [1. Sampling and the Nyquist theorem](#1-sampling-and-the-nyquist-theorem)
   - [2. Fourier analysis (DFT / FFT)](#2-fourier-analysis-dft--fft)
   - [3. LTI systems and digital filters](#3-lti-systems-and-digital-filters)
   - [4. Digital logic and fixed-point audio](#4-digital-logic-and-fixed-point-audio)
6. [Repository structure](#repository-structure)
7. [Roadmap](#roadmap)
8. [Testing and validation](#testing-and-validation)
9. [Design decisions](#design-decisions)
10. [Further reading](#further-reading)
11. [Contributing](#contributing)
12. [License](#license)

---

## Who this is for

- **Electrical and Electronic Engineering students** taking Signals & Systems, Digital Signal Processing, Digital Electronics or Communications, who want to see the theory run.
- **Engineers** moving from analog or embedded work into DSP who want a small, readable reference implementation in a systems language.
- **Anyone** who has asked "where would I ever use a Fourier transform?"

You do not need prior Rust experience to follow the theory docs. You do need it to modify the code. The code favours clarity over cleverness, and each module is deliberately small.

---

## Theory to code map

This table is the heart of the project. Each row links a lecture topic to the code that demonstrates it and the observable result that proves it.

| Theory (EEE 2207 / related) | Key equation | Code | What you observe |
|---|---|---|---|
| Sampling, aliasing, Nyquist-Shannon | `fs > 2·fmax` | `src/sampler.rs` | A tone above `fs/2` folds down to a lower apparent frequency |
| Fourier series and transform (DFT in discrete time) | `X[k] = Σ x[n]·e^(-j2πkn/N)` | `src/dft.rs` | A noisy waveform resolves into discrete spectral peaks |
| LTI systems, difference equations | `y[n] = αx[n] + (1-α)y[n-1]` | `src/filters/iir.rs` | A digital RC low-pass attenuates high frequencies |
| Convolution, impulse response | `y[n] = Σ h[k]·x[n-k]` | `src/filters/fir.rs` | A moving-average or windowed-sinc filter shapes the spectrum |
| Binary arithmetic, shifts, fixed-point | `x << 1` doubles, `x >> 1` halves | `src/wav_io.rs`, `src/utils.rs` | Gain changes of about 6 dB per shift, plus overflow distortion |

---

## Quick start

### Prerequisites

- Rust toolchain (stable), installed via [rustup](https://rustup.rs)
- A terminal. Audio playback is optional and not required for the analysis tools.

### Build and run

```bash
git clone https://github.com/Mwangi-Derrick/ss-audio-dsp.git
cd ss-audio-dsp
cargo build --release
cargo run --release -- sample --analog-freq 2 --sample-rate 10
```

### Run the tests

```bash
cargo test
```

---

## Usage

The CLI is organised as subcommands, one per theory area. Only `sample` is implemented so far. Others are listed in the [roadmap](#roadmap).

```text
ss-audio-dsp <COMMAND>

Commands:
  sample    Simulate sampling of a sine wave and demonstrate aliasing
  analyze   (planned) Compute and display the spectrum of a WAV file
  filter    (planned) Apply an LTI filter to a WAV file
  gain      (planned) Apply a bit-shift gain to PCM samples
```

### Example: demonstrating aliasing

```bash
# Well sampled: 2 Hz tone at 10 Hz sampling (fs = 5 x f)
cargo run -- sample --analog-freq 2 --sample-rate 10

# Aliased: 8 Hz tone at 10 Hz sampling (fs < 2 x f)
cargo run -- sample --analog-freq 8 --sample-rate 10
```

In the second run, the samples are indistinguishable from those of a 2 Hz tone. The 8 Hz signal has "folded" about `fs/2 = 5 Hz`.

The apparent (aliased) frequency is:

```text
f_alias = | f - fs * round(f / fs) |
```

For `f = 8`, `fs = 10`: `round(0.8) = 1`, so `f_alias = |8 - 10| = 2 Hz`.

---

## The four modules

### 1. Sampling and the Nyquist theorem

**Theory.** A continuous-time signal `x(t)` is sampled every `Ts = 1/fs` seconds to produce `x[n] = x(nTs)`. The Nyquist-Shannon theorem states that a bandlimited signal with maximum frequency `fmax` can be perfectly reconstructed only if `fs > 2·fmax`. Below that, spectral copies overlap and frequencies are indistinguishable (aliasing).

**Why it matters.** Every ADC has an anti-aliasing filter in front of it for exactly this reason. Audio CDs sample at 44.1 kHz because human hearing tops out near 20 kHz.

**Implementation.** `sampler.rs` uses a timed loop to act as the sampling clock of a simulated ADC. At each tick, it evaluates `sin(2πft)` and prints the sample.

**Observation to try.** Sweep `--analog-freq` from 1 to 9 at `--sample-rate 10` and watch the apparent frequency rise to 5 Hz, then fall back down.

**Caveat.** The wall-clock ticker is a teaching device. `thread::sleep` has scheduling jitter, so it is not a real-time clock. For exact results, the planned offline mode computes `x[n] = sin(2π f n / fs)` with no timing at all.

Full notes: [`docs/01_sampling_nyquist.md`](docs/01_sampling_nyquist.md)

### 2. Fourier analysis (DFT / FFT)

**Theory.** The Fourier series decomposes a periodic signal into harmonically related sinusoids. The Fourier transform generalises this to aperiodic signals. In discrete time, the DFT of an `N`-sample block is:

```text
X[k] = sum_{n=0}^{N-1} x[n] * e^(-j*2*pi*k*n/N),    k = 0..N-1
```

Bin `k` corresponds to frequency `k * fs / N`. For real input, bins above `N/2` mirror those below.

**Implementation.** `dft.rs` is a direct O(N²) implementation, written to match the formula line for line. It is intentionally slow. Once it is verified, the `rustfft` crate provides an O(N log N) FFT, and the two are compared in tests.

**Observation to try.** Synthesise a 440 Hz + 880 Hz mixture and confirm two peaks with the expected amplitude ratio.

Full notes: [`docs/02_fourier_transform.md`](docs/02_fourier_transform.md)

### 3. LTI systems and digital filters

**Theory.** A system is linear if it obeys superposition and time-invariant if a delayed input gives an equally delayed output. Such systems are fully characterised by their impulse response `h[n]`, with output given by convolution `y[n] = x[n] * h[n]`. Continuous systems are described by differential equations. Discrete ones use difference equations.

**Implementation.**

- **IIR (recursive):** a first-order low-pass derived from an RC circuit.

  ```text
  y[n] = alpha * x[n] + (1 - alpha) * y[n-1]
  alpha = dt / (RC + dt),    fc = 1 / (2*pi*RC)
  ```

  Output depends on the previous output, so the system has memory (state).

- **FIR (non-recursive):** direct convolution with a finite impulse response, such as a moving average.

**Observation to try.** Feed white noise through the low-pass filter and compare the spectra before and after.

Full notes: [`docs/03_lti_systems_filters.md`](docs/03_lti_systems_filters.md)

### 4. Digital logic and fixed-point audio

**Theory.** PCM audio is stored as signed 16-bit integers (`i16`, two's complement, range -32768 to 32767). Shifts are the cheapest hardware multiplier: `<< 1` multiplies by 2, and `>> 1` divides by 2 with truncation (arithmetic shift for signed types).

Doubling amplitude is a gain of `20·log10(2) ≈ +6.02 dB`.

**The overflow lesson.** Shifting a large sample left can exceed the 16-bit range. In Rust, `<<` does not check value overflow, so the high bits are discarded and the result **wraps** (a large positive value becomes negative). This produces harsh distortion that sounds very different from the gentler analog-style clipping you get by **saturating** at the limits. The project implements both and compares them:

```rust
// Wrapping (what a naive shift does)
let wrapped = sample << 1;

// Saturating (what a well-designed system does)
let saturated = (sample as i32 * 2).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
```

Full notes: [`docs/04_bitwise_digital_logic.md`](docs/04_bitwise_digital_logic.md)

---

## Repository structure

```text
ss-audio-dsp/
├── Cargo.toml
├── README.md
├── .gitignore
├── docs/
│   ├── 01_sampling_nyquist.md
│   ├── 02_fourier_transform.md
│   ├── 03_lti_systems_filters.md
│   ├── 04_bitwise_digital_logic.md
│   └── img/                    # plots referenced by the docs
├── src/
│   ├── main.rs                 # CLI entry point (clap)
│   ├── lib.rs                  # library root, exposes modules to tests
│   ├── sampler.rs              # sampling / aliasing simulation
│   ├── wav_io.rs               # WAV parsing and writing
│   ├── dft.rs                  # DFT from the definition
│   ├── filters/
│   │   ├── mod.rs
│   │   ├── iir.rs              # recursive filters
│   │   └── fir.rs              # convolution filters
│   └── utils.rs                # ASCII plots, dB conversions, helpers
└── tests/
    └── integration_test.rs
```

---

## Roadmap

- [x] **Week 1:** Sampling simulator demonstrating Nyquist and aliasing
- [ ] **Week 2:** WAV file reader/writer (PCM into sample buffers)
- [ ] **Week 3:** Bit-shift gain with wrapping vs saturating comparison
- [ ] **Week 4:** DFT from scratch, then FFT via `rustfft`, with ASCII spectrum output
- [ ] **Week 5:** First-order IIR low-pass applied to a real audio file
- [ ] **Later:** FIR filters and windowing, frequency-response plotting, high-pass and band-pass filters, spectrogram (STFT), echo / comb filter as an LTI system with delay

---

## Testing and validation

DSP code is easy to get subtly wrong, so each module is checked against a known answer:

| Module | Test idea |
|---|---|
| Sampler | A tone of `f` Hz at `fs` Hz yields apparent frequency matching `f_alias` |
| DFT | A pure sine at bin-aligned frequency produces one peak (plus its mirror) at the correct bin |
| DFT vs FFT | Outputs agree within floating-point tolerance |
| IIR | Step response approaches 1 with time constant close to `RC`; DC passes, high frequencies are attenuated |
| Linearity | `filter(a·x1 + b·x2) == a·filter(x1) + b·filter(x2)` (property test) |
| Gain shifts | `<<` and saturating variants agree below overflow and differ above it |

---

## Design decisions

- **From scratch first, crates second.** The DFT and filters are hand-written so the code mirrors the equations. Libraries like `rustfft` and `hound` are used only after the from-scratch version is verified or where re-implementing adds no learning value.
- **Small, separable modules.** Each theory topic lives in one file and has a matching document in `docs/`.
- **`f64` for maths, `i16` for storage.** Processing is done in floating point and converted at the I/O boundary, as real systems often do.
- **Library plus binary.** Putting logic in `lib.rs` lets tests exercise it directly.

---

## Further reading

- A. V. Oppenheim, A. S. Willsky, *Signals and Systems* (the standard text)
- A. V. Oppenheim, R. W. Schafer, *Discrete-Time Signal Processing*
- S. W. Smith, *The Scientist and Engineer's Guide to Digital Signal Processing* (free online, very readable)
- J. O. Smith III, *Introduction to Digital Filters* (free online, audio-focused)
- Your EEE 2207 lecture notes. Cross-reference the `docs/` files against them and correct anything that differs.

---

## Contributing

Issues and pull requests are welcome, particularly corrections to the theory notes, additional test cases, and new demonstrations. Please keep the "from scratch, readable" spirit. Open an issue before large changes.

---

## License

Released under the MIT License. See `LICENSE`.