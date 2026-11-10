# 03: LTI Systems and Digital Filters

> Course link: EEE 2207, system properties, convolution, difference/differential equations. Cross-links to your Electronics (RC circuit) coursework.

## 1. Learning objectives

- Test a system for linearity, time-invariance, causality and memory
- Use the impulse response and convolution to find outputs
- Derive a digital filter from an analog RC circuit
- Distinguish FIR from IIR filters and state their trade-offs

## 2. System properties

| Property | Test | Example |
|---|---|---|
| Linear | `T{a·x1 + b·x2} = a·T{x1} + b·T{x2}` | `y[n] = 3x[n]` is linear; `y[n] = x[n]²` is not |
| Time-invariant | `x[n-n0] -> y[n-n0]` | `y[n] = x[n] + x[n-1]` is TI; `y[n] = n·x[n]` is not |
| Causal | Output depends only on present and past inputs | Real-time filters must be causal |
| Memoryless | Output depends only on the present input | `y[n] = 2x[n]` has no memory; a filter with `y[n-1]` does |
| Stable (BIBO) | Bounded input gives bounded output | Check `sum |h[n]| < inf` |

A system that is both linear and time-invariant is **LTI**. LTI systems are fully described by their impulse response.

## 3. Theory

### 3.1 Impulse response and convolution

If `h[n]` is the output when the input is the unit impulse `delta[n]`, then for any input:

```text
y[n] = x[n] * h[n] = sum_k x[k] * h[n-k]
```

In continuous time: `y(t) = integral x(tau) * h(t - tau) dtau`.

### 3.2 Frequency response

Convolution in time becomes multiplication in frequency:

```text
Y(w) = H(w) * X(w)
```

`H(w)` is the **frequency response**. `|H|` is the gain at each frequency (what an equalizer plots), and `angle(H)` is the phase shift.

### 3.3 From RC circuit to digital filter

The analog RC low-pass has the differential equation:

```text
RC * dy/dt + y(t) = x(t)
```

Cutoff frequency and time constant:

```text
fc = 1 / (2*pi*RC),    tau = RC
```

Approximate the derivative with a backward difference over one sample period `dt = 1/fs`:

```text
RC * (y[n] - y[n-1]) / dt + y[n] = x[n]
```

Solving for `y[n]`:

```text
y[n] = alpha * x[n] + (1 - alpha) * y[n-1]
alpha = dt / (RC + dt)
```

This is a first-order **IIR** low-pass, often called an exponential moving average.

**Design recipe:**

1. Choose `fc` (e.g. 1 kHz) and `fs` (e.g. 44100 Hz).
2. `RC = 1 / (2*pi*fc)`, `dt = 1/fs`.
3. `alpha = dt / (RC + dt)`.

Small `alpha` gives heavy smoothing (low cutoff). `alpha = 1` passes everything.

**Why memory matters:** the term `y[n-1]` means the system stores state, just as the capacitor stores charge.

### 3.4 FIR vs IIR

| | FIR | IIR |
|---|---|---|
| Form | `y[n] = sum b_k x[n-k]` | Uses past outputs too |
| Impulse response | Finite length | Infinite (decaying) |
| Stability | Always stable | Must check poles |
| Linear phase | Possible (symmetric taps) | Generally not |
| Cost for sharp cutoff | High (many taps) | Low (few coefficients) |
| Analog analog | Moving average, windowed sinc | RC, Butterworth, Chebyshev |

### 3.5 Simple examples

- **Moving average (FIR, N taps):** `h[n] = 1/N` for `n = 0..N-1`. A crude low-pass.
- **Echo (FIR with delay):** `y[n] = x[n] + a·x[n-D]`. Produces a comb-filter spectrum.
- **First difference:** `y[n] = x[n] - x[n-1]`. A crude high-pass.

### 3.6 Z-domain preview

Taking the Z-transform of the IIR equation gives:

```text
H(z) = alpha / (1 - (1 - alpha) * z^-1)
```

A single pole at `z = 1 - alpha`. The pole lies inside the unit circle for `0 < alpha <= 1`, so the filter is stable. Poles closer to 1 mean a lower cutoff and slower response.

## 4. Implementation notes

- `filters/iir.rs`: struct holding `alpha` and `prev_output` (the state). A `process(&mut self, x: f64) -> f64` method keeps the state between samples.
- `filters/fir.rs`: struct holding the taps and a delay-line buffer. Compare against direct convolution.
- Process blocks of samples with an iterator so the same filter works for streaming and for files.

## 5. Experiments

| Experiment | Expected |
|---|---|
| Step input into IIR | Exponential rise to 1, reaching about 63% after `RC` seconds |
| Impulse input into IIR | Decaying exponential `alpha·(1-alpha)^n` |
| White noise before/after | Spectrum rolls off above `fc` |
| 100 Hz + 5 kHz mix, `fc = 500 Hz` | 5 kHz strongly attenuated; 100 Hz nearly unchanged |
| Linearity check | `filter(a·x1 + b·x2)` equals `a·filter(x1) + b·filter(x2)` |
| Time-invariance check | Delay input by `k` samples; output delays by `k` |

## 6. Questions to answer in your own words (tweak me)

1. Why can the backward-difference method distort the cutoff frequency at high `fc/fs`? What does the bilinear transform do better?
2. How does the filter behave at DC (`w = 0`)? Show `H(0) = 1`.
3. A high-pass can be built as `x[n] - lowpass(x[n])`. Why does that work for LTI systems?

## 7. Add later

- Frequency-response plot of the IIR filter
- Second-order (biquad) filters and the audio EQ cookbook
- Pole-zero diagrams