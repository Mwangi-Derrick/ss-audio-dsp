# 01: Sampling and the Nyquist-Shannon Theorem

> Course link: EEE 2207(related), sampling and reconstruction of continuous-time signals.

## 1. Learning objectives

- Define sampling and relate `x(t)` to `x[n]`
- State the Nyquist-Shannon theorem and explain why it holds
- Predict the aliased frequency for any `f` and `fs`
- Explain why real systems use anti-aliasing filters

## 2. Theory

### 2.1 Sampling

Ideal uniform sampling of a continuous signal `x(t)` with period `Ts` gives

```text
x[n] = x(n * Ts),    fs = 1 / Ts
```

Mathematically, this is multiplication by an impulse train:

```text
x_s(t) = x(t) * sum_n delta(t - n*Ts)
```

### 2.2 What happens in frequency

Multiplying by an impulse train in time corresponds to convolving with an impulse train in frequency. The spectrum of `x(t)`, `X(f)`, is therefore **repeated at every multiple of `fs`**:

```text
X_s(f) = fs * sum_k X(f - k*fs)
```

If `X(f)` is bandlimited to `fmax` and `fs > 2*fmax`, the copies do not overlap and the original can be recovered with an ideal low-pass filter. If `fs <= 2*fmax`, copies overlap. That overlap is **aliasing**.

### 2.3 Aliased frequency

A sinusoid of frequency `f` sampled at `fs` appears at:

```text
f_alias = | f - fs * round(f / fs) |
```

This always lands in `[0, fs/2]`.

### 2.4 Worked example

`f = 8 Hz`, `fs = 10 Hz`:

- `round(8/10) = 1`
- `f_alias = |8 - 10| = 2 Hz`

Check by sampling at `t = n/10`:

```text
sin(2*pi*8*n/10) = sin(2*pi*n*(1 - 0.2)) = sin(-2*pi*0.2*n) = -sin(2*pi*2*n/10)
```

So the samples are those of a 2 Hz sine with inverted sign (phase flip), and the 8 Hz tone is indistinguishable from a 2 Hz one.

### 2.5 Practical points

- **Anti-aliasing filter:** an analog low-pass before the ADC removes content above `fs/2`.
- **Oversampling:** sampling well above Nyquist relaxes the analog filter requirements.
- **Real-world rates:** 8 kHz (telephony), 44.1 / 48 kHz (audio), 96 / 192 kHz (high-resolution audio).

## 3. Implementation notes (`src/sampler.rs`)

- The loop ticks every `Ts` seconds and evaluates `sin(2*pi*f*t)`.
- `thread::sleep` is imprecise (OS scheduling), so compute `t = n / fs` from the sample counter rather than from measured elapsed time. That keeps the maths exact even if the timing wobbles.
- Planned: an `--offline` flag that skips sleeping and emits `N` samples instantly.

## 4. Experiments

| Run | Command | Expected |
|---|---|---|
| Oversampled | `sample -a 2 -s 10` | Smooth, recognisable 2 Hz sine |
| Marginal | `sample -a 5 -s 10` | At exactly `fs/2`, samples depend on phase and the amplitude can appear reduced or zero |
| Aliased | `sample -a 8 -s 10` | Appears as 2 Hz |
| Wagon wheel | `sample -a 10 -s 10` | Appears as DC (0 Hz) |

## 5. Questions to answer in your own words (tweak me)

1. Why is `fs = 2*fmax` exactly not sufficient in general?
2. Sketch `X_s(f)` for `fs = 3*fmax` and `fs = 1.5*fmax`.
3. Why does a telephone line sample at 8 kHz but only carry about 300 to 3400 Hz?

## 6. Add later

- `img/nyquist_aliasing.png`: plot of 2 Hz and 8 Hz samples overlaid on `fs = 10 Hz`
- Link to lecture notes and lecturer's example problems