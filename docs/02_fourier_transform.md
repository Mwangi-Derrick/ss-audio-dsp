# 02: Fourier Series, Fourier Transform and the DFT

> Course link: EEE 2207, frequency-domain representation of signals.

## 1. Learning objectives

- Explain why the frequency domain is useful
- Relate Fourier series (periodic, continuous) to Fourier transform (aperiodic, continuous) and DFT (finite, discrete)
- Compute and interpret a DFT, including bin frequency and amplitude scaling
- Understand spectral leakage and windowing

## 2. The Fourier family

| Transform | Time signal | Frequency signal |
|---|---|---|
| Fourier Series (CTFS) | Continuous, periodic | Discrete, aperiodic |
| Fourier Transform (CTFT) | Continuous, aperiodic | Continuous, aperiodic |
| DTFT | Discrete, aperiodic | Continuous, periodic |
| **DFT** | **Discrete, finite (N samples)** | **Discrete, finite (N bins)** |

The DFT is what computers use. It treats the `N` samples as one period of a periodic signal.

## 3. Theory

### 3.1 Fourier series

A periodic `x(t)` with period `T0` and `w0 = 2*pi/T0`:

```text
x(t) = sum_{k=-inf}^{inf} c_k * e^(j*k*w0*t)
c_k  = (1/T0) * integral_{T0} x(t) * e^(-j*k*w0*t) dt
```

A square wave contains only odd harmonics with amplitudes falling as `1/k`. This is a good test signal.

### 3.2 Fourier transform

```text
X(w) = integral x(t) * e^(-j*w*t) dt
x(t) = (1/2pi) * integral X(w) * e^(j*w*t) dw
```

### 3.3 The DFT

```text
X[k] = sum_{n=0}^{N-1} x[n] * e^(-j*2*pi*k*n/N),    k = 0, ..., N-1
x[n] = (1/N) * sum_{k=0}^{N-1} X[k] * e^(+j*2*pi*k*n/N)
```

Using Euler's formula, each `X[k]` is a correlation of the signal with a cosine and a sine at frequency `k*fs/N`:

```text
X[k] = sum x[n]*cos(2*pi*k*n/N)  -  j * sum x[n]*sin(2*pi*k*n/N)
```

### 3.4 Reading the result

- **Bin frequency:** `f_k = k * fs / N`
- **Resolution:** `delta_f = fs / N`. Longer recordings give finer resolution.
- **Magnitude:** `|X[k]| = sqrt(Re^2 + Im^2)`
- **Amplitude of a real sinusoid:** `A = 2 * |X[k]| / N` for `0 < k < N/2`
- **Symmetry:** for real input, `X[N-k] = conj(X[k])`, so only bins `0..N/2` carry unique information.
- **Nyquist link:** bin `N/2` corresponds to `fs/2`. This is where aliasing from topic 01 shows up in the spectrum.

### 3.5 Spectral leakage

If a sinusoid does not complete a whole number of cycles in the `N`-sample window, its energy "leaks" into neighbouring bins. A window function (Hann, Hamming) tapers the edges and reduces leakage at the cost of a wider main lobe.

### 3.6 Complexity and the FFT

The direct DFT costs O(N²) operations. The Cooley-Tukey FFT costs O(N log N). For `N = 4096`, that is roughly 16.8 million vs about 49 thousand multiply-adds, a speed-up of about 340 times.

## 4. Implementation notes (`src/dft.rs`)

1. Implement `dft(x: &[f64]) -> Vec<Complex>` straight from the equation.
2. Add `magnitude_spectrum` and `bin_to_hz(k, fs, n)`.
3. Test with a bin-aligned sine (single clean peak).
4. Test with a non-aligned sine (observe leakage), then apply a Hann window.
5. Compare against `rustfft` output.

## 5. Experiments

| Signal | Expected spectrum |
|---|---|
| 440 Hz sine | Peak at bin `440*N/fs` |
| 440 Hz + 880 Hz, amplitudes 1 and 0.5 | Two peaks with 2:1 ratio |
| Square wave at 100 Hz | Peaks at 100, 300, 500 Hz... falling as `1/k` |
| White noise | Roughly flat spectrum |
| Impulse | Flat magnitude across all bins |

## 6. Questions to answer in your own words (tweak me)

1. Why does a time-limited signal never have a perfectly bandlimited spectrum?
2. What happens to the spectrum when you zero-pad the input? Does it add information?
3. Why is `X[0]` always real for real input, and what does it represent?

## 7. Add later

- `img/fft_spectrum.png`: spectrum of the two-tone test
- Parseval's theorem check: `sum |x[n]|^2 == (1/N) * sum |X[k]|^2`
- Short-Time Fourier Transform and spectrogram notes