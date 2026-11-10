# 04: Digital Logic and Fixed-Point Audio

> Course link: Digital Electronics (logic gates, binary arithmetic, number representation), applied to PCM audio.

## 1. Learning objectives

- Represent signed numbers in two's complement
- Explain shifts as multiplication and division by powers of two
- Convert gain ratios to decibels
- Distinguish wrap-around from saturation and explain why it matters in DSP hardware

## 2. Theory

### 2.1 PCM audio

Pulse-code modulation stores each sample as an integer. CD-quality audio uses 16-bit signed samples at 44.1 kHz.

- Range: `-32768` to `+32767` (`i16`)
- Quantisation step: `1 LSB`
- Theoretical SNR for an `N`-bit quantiser with a full-scale sine: `SNR ≈ 6.02·N + 1.76 dB`, so about 98 dB for 16 bits

### 2.2 Two's complement

For `N` bits, the most significant bit has weight `-2^(N-1)`. Example with 4 bits:

| Binary | Value |
|---|---|
| `0111` | +7 |
| `0001` | +1 |
| `0000` | 0 |
| `1111` | -1 |
| `1000` | -8 |

Addition works with the same hardware for signed and unsigned values, which is why two's complement is universal.

### 2.3 Shifts

- **Left shift** `x << 1`: multiplies by 2 (if no overflow)
- **Arithmetic right shift** `x >> 1` on a signed type: divides by 2, rounding toward negative infinity, and preserves the sign bit
- **Logical right shift** (unsigned types): fills with zeros

Shifts are single-cycle operations in hardware and need no multiplier.

### 2.4 Gain in decibels

```text
gain_dB = 20 * log10(A_out / A_in)
```

| Operation | Amplitude ratio | Gain |
|---|---|---|
| `<< 1` | 2 | +6.02 dB |
| `<< 2` | 4 | +12.04 dB |
| `>> 1` | 0.5 | -6.02 dB |
| `>> 2` | 0.25 | -12.04 dB |

Shifts give only coarse, power-of-two gains. Finer control needs multiplication by a fixed-point factor, e.g. `(x * g) >> 15` with `g` as a Q15 number.

### 2.5 Overflow: wrap vs saturate

Take `x = 20000` and apply `<< 1`. The mathematical result 40000 exceeds 32767.

- **Wrap-around:** keeps only the low 16 bits. 40000 becomes `40000 - 65536 = -25536`. A loud positive peak flips to a loud negative value, producing severe, broadband distortion (a "crackle").
- **Saturation (clipping):** clamps to 32767. The waveform is flattened rather than flipped. It still distorts, but far less harshly, and it is how most hardware DSPs behave.

Rust note: in debug builds, arithmetic overflow with `+`, `-`, `*` panics. The shift operators `<<` and `>>` panic only if the shift *amount* is too large, not if bits are lost. So `x << 1` on an `i16` silently wraps the value. Use `wrapping_*`, `saturating_*` or `checked_*` methods, or widen to `i32` and clamp, to make behaviour explicit.

```rust
let wrapped   = x.wrapping_shl(1);                         // explicit wrap
let saturated = ((x as i32) << 1).clamp(-32768, 32767) as i16;
```

### 2.6 Other bitwise operations in audio

- **AND with mask:** extract fields or reduce bit depth (e.g. `x & 0xFF00` keeps the top 8 bits, a crude bit-crusher)
- **XOR:** toggling bits; used in simple checksums and some distortion effects
- **Byte assembly:** WAV files are little-endian, so a sample is `(hi << 8) | lo`

## 3. Implementation notes (`wav_io.rs`, `utils.rs`)

1. Parse the WAV header (RIFF, `fmt `, `data` chunks). Confirm PCM, 16-bit.
2. Read samples into `Vec<i16>`. Start by assembling bytes manually with shifts, then compare with the `hound` crate.
3. Implement `gain_shift_wrapping`, `gain_shift_saturating`, and a `to_db(ratio)` helper.
4. Write the processed samples back out so you can listen to the difference.

## 4. Experiments

| Experiment | Expected |
|---|---|
| Quiet tone `>> 1` | 6 dB quieter, quantisation noise becomes relatively more audible |
| Quiet tone `<< 1` | 6 dB louder, no distortion |
| Loud tone `<< 1`, wrapping | Harsh crackle |
| Loud tone `<< 1`, saturating | Flattened peaks, milder distortion |
| Bit-crusher via mask | Gritty, lo-fi sound |
| Spectrum of clipped sine (link to topic 02) | Added harmonics at odd multiples |

## 5. Questions to answer in your own words (tweak me)

1. Why does a right shift of a negative number not give the same result as truncation toward zero?
2. How many dB of dynamic range does an 8-bit system have compared to 16-bit?
3. Why does hard clipping create harmonics, in terms of the Fourier series from topic 02?

## 6. Add later

- Q-format fixed-point (Q15, Q31) multiplication
- Dithering and noise shaping
- Comparison with 24-bit and 32-bit float audio