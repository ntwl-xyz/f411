# kttl — Analog Control Prototype Wiring (LM35 sensor)

Fully-analog temperature controller for a kettle, styled after a Soviet
radiometer panel. No MCU: a comparator loop reaches/holds a rotary-selected
target, a moving-coil meter shows live temperature on a **linear 0–100 °C
dial**, and a solenoid-struck keyswitch clicks (Geiger-style) on each heater
cycle.

Sensor is an **LM35** (10 mV/°C, linear) — chosen so the dial is truly linear
and the setpoint ladder is evenly spaced.

> **Status:** prototype / paper design. A few values depend on measuring the
> actual meter movement and solenoid — see *Open Unknowns*.

---

## 1. Panel controls

| Ref | Control            | Type                 | Function                                  | Domain      |
|-----|--------------------|----------------------|-------------------------------------------|-------------|
| SW1 | Rotary selector    | 6-position, 1 wafer  | Target temp tap (60/70/80/85/90/100 °C)   | low-voltage |
| SW2 | Rocker (on/off)    | SPST                 | **Heater enable only** (not master power) | low-voltage |
| SW3 | Hold toggle        | SPDT                 | After target: HOLD (cycle) vs ONE-SHOT    | low-voltage |
| M1  | Moving-coil meter  | µA/mA movement (М205)| Live temperature, linear 0–100 °C         | always-on   |

Mains is live whenever plugged in. Electronics + meter sit on an **always-on
rail**; the heater is gated downstream and switched only by the relay/SSR
(mains-last). The rocker just **stops heating** — it is not a master cut-off.

---

## 2. Power tree

```
   MAINS (live when plugged in)
     ├─────────► PSU (mains→5V, always on) ──► +5V rail ─┬─► LM35, comparator, latch,
     │                                                   │   meter driver, clicker logic
     │                                                   └─► VREF_PRECISE (LM4040-2.5)
     │                                                       → setpoint ladder
     │
     └─► HEATER PATH (mains):  F1 fuse ─► TC1 thermal cutoff ─► K1/SSR1 ─► ELEMENT
                                                                  ▲
                                  enable = SW2(rocker) AND logic-heat
```

**Note:** the ladder reference is now a **precision reference (not ratiometric)**
because the LM35 output is an absolute voltage (10 mV/°C). The 5 V rail no longer
needs to be accurate, but VREF_PRECISE does.

---

## 3. Schematic (block + signal level)

```
        +5V (always on)                         VREF_PRECISE = 2.500V (LM4040-2.5)
            │                                              │
     ┌──────┴──────┐                              ┌────────┴────────┐
     │  LM35        │  OUT = 10mV/°C               │ SETPOINT LADDER │
     │  Vs  OUT  GND│  (0°C=0V, 100°C=1.00V)       │ R1..R7 (taps=°C)│
     └───┬───┬───┬──┘                              └────────┬────────┘
         │   │   └─GND                                      │ taps: 0.60/0.70/0.80/
         │   │                                              │       0.85/0.90/1.00 V
         │   ├─────────────┬──────────────┐         ┌───────┴───────┐
         │   │             │              │         │ SW1 rotary 6-pos│
      [Rpd 18k             │              │         └───────┬───────┘
       to GND]        (to meter      (to comparator −)      │ common = selected Vref
         │            driver U3)          │          R_in 10k │
        GND                               │            │      │
                                          ▼            ▼      │
                                    ┌──────────────────────┐  │
                                    │  U1 COMPARATOR LM393 │  │ Rf 2.4M
                                    │  (−) LM35    (+) tap  │◄─┴──── hysteresis
                                    │              OUT      ├──┬──── (to +) ~±1°C
                                    └──────────────────────┘  │
                                       OUT HIGH = cold         │ on→off edge = "reached"
                                       (pull-up 10k to +5V)    │
                                ┌──────────────────────────────┴──┐
                                │                                  │
                                ▼                                  ▼
                        ┌──────────────┐                  ┌────────────────┐
                        │ U2 CD4013    │                  │ CLICKER one-shot│
                        │ D-FF latch   │                  │ → Q1 MOSFET     │
                        │ D=Hi         │                  │ → L1 solenoid   │
                        │ CLK=reached  │                  │   (+ D1 flyback)│
                        │ CLR=POR + SW1│                  │ strikes keyswitch│
                        │        Q' ───┼──┐               └────────────────┘
                        └──────────────┘  │
                                          ▼
                  SW3 HOLD ──────► ┌───────────────┐
                  (HOLD: force Hi; │ ENABLE gate   │
                   1-SHOT: use Q') │ (AND)         │
                                   └──────┬────────┘
                                          │ heater-want
                                  SW2 rocker (series)
                                          │
                                          ▼
                                   ┌──────────────┐
                                   │ Q2 ─► K1/SSR1│  switches MAINS to ELEMENT
                                   └──────┬───────┘  (via F1 + TC1, mains-last)
                                          ▼
                                       ELEMENT

   METER BRANCH:
        LM35 OUT ─► U3 (op-amp + transistor current sink, Re) ─► M1
                    I_meter = V_LM35 / Re      (no offset: 0°C = 0V)
                    linear 0–100 °C dial
```

---

## 4. Component values

### 4.1 Sensor — LM35
- **U_LM35:** LM35 (or LM35DZ), Vs = +5 V, OUT = 10 mV/°C.
  - 0 °C → 0.000 V, 60 °C → 0.600 V, 100 °C → 1.000 V.
- **Rpd:** ~18 kΩ from OUT to GND (lets the output pull down toward 0 V; basic
  LM35 only *sources* current). True 0 °C reading needs a negative rail; with a
  GND pull-down it reads reliably from ~+2 °C up — fine for a kettle.
- Seal the sensor against water/steam (it's a 3-pin active part, not a bead).

### 4.2 Precision reference + setpoint ladder
- **VREF_PRECISE:** LM4040-2.5 (2.500 V shunt reference) + bias resistor from
  +5 V (e.g. ~2.7 kΩ). Absolute accuracy now matters (not ratiometric).
- Ladder GND → 2.500 V; SW1 taps land on the LM35 setpoint voltages:

| Ref | Segment              | Calc.   | Use (E96/std) |
|-----|----------------------|---------|---------------|
| R1  | GND → 60 °C (0.60 V) | 6.00 kΩ | 6.04 kΩ       |
| R2  | 60 → 70 °C           | 1.00 kΩ | 1.00 kΩ       |
| R3  | 70 → 80 °C           | 1.00 kΩ | 1.00 kΩ       |
| R4  | 80 → 85 °C           | 0.50 kΩ | 499 Ω         |
| R5  | 85 → 90 °C           | 0.50 kΩ | 499 Ω         |
| R6  | 90 → 100 °C          | 1.00 kΩ | 1.00 kΩ       |
| R7  | 100 °C (1.00 V) → ref| 15.0 kΩ | 15.0 kΩ       |
| RT  | master cal trim      | —       | 500 Ω 10-turn (series w/ R7) |

Ladder current ≈ 100 µA. Setpoint voltages = temp × 10 mV/°C — clean and even.

### 4.3 Comparator + hysteresis
- **U1:** LM393. LM35 OUT → (−); SW1 tap → (+) via **R_in = 10 kΩ**.
  OUT HIGH = cold = heater wanted. Pull-up 10 kΩ on OUT to +5 V.
- **Rf:** 2.4 MΩ from OUT to (+) → maintain band ≈ ±1 °C (R_in/Rf · Vout_swing).
  Band varies slightly with selected tap impedance.

### 4.4 Mode latch (one-shot vs hold)
- **U2:** CD4013 D flip-flop. D = HIGH. CLK = comparator on→off ("reached") edge
  (RC-debounce if it chatters). CLR = power-on-reset cap **and** SW1 common
  (changing temp re-arms). Q′ → enable gate.
- **SW3 (Hold):**
  - HOLD → forces enable gate's 2nd input HIGH (latch bypassed → cycles forever).
  - ONE-SHOT → enable uses Q′ (heater latches OFF after first reach).
  - Wire SW3 to also pulse CLR so flipping to HOLD resumes cycling.

### 4.5 Heater drive
- Enable gate (heater-want) AND **SW2 rocker** (series) → **Q2** (small MOSFET) →
  relay coil **K1** (with flyback diode) *or* **SSR1** control input.
- K1/SSR1 switches mains to the element, downstream of **F1 (fuse)** and **TC1
  (independent thermal cutoff)**.

### 4.6 Clicker
- Comparator on→off edge → one-shot pulse → **Q1** (logic-level MOSFET) → **L1**
  mini push-pull solenoid (5 V, ~3.5 mm stroke, Adafruit 2776 class) + **D1**
  flyback diode. Plunger strikes a tactile keyswitch (switch gives the click +
  bump; pick the strike target/resonator for pitch).
- HOLD → ticks each maintain cycle; ONE-SHOT → single tick. Add jitter to the
  pulse for Geiger-style randomness instead of metronomic ticking.

### 4.7 Meter driver
- **U3:** op-amp + transistor current sink. LM35 OUT → op-amp (+); meter in the
  collector/drain; Re in emitter/source. `I_meter = V_LM35 / Re`, independent of
  coil resistance. **No offset stage** (0 °C already = 0 V).
- Re by movement: **1 mA → Re = 1.00 kΩ**; **100 µA → Re = 10.0 kΩ**.
- Dial is **linear 0–100 °C** — re-letter the М205 face 0→100.

---

## 5. Safety notes
- **SW1/SW2/SW3 are all low-voltage.** No panel switch carries element current;
  mains is switched only by K1/SSR1 (mains-last).
- **F1 fuse + TC1 thermal cutoff in series with the element are mandatory.** The
  comparator/latch is *control*, not *protection* — it can't save you from a
  welded relay or a failed/detached sensor.
- **Always-on rail = always-live internals.** True off is unplugging; the rocker
  only stops heating.
- Keep the meter/logic rail decoupled from heater switching transients, or the
  needle twitches and the latch can glitch each cycle.

---

## 6. Open unknowns (measure before committing values)
1. **М205 movement:** full-scale current and coil resistance — sets Re. Measure
   first; guess wrong and the needle pins or barely moves.
2. **Solenoid force/stroke** vs the chosen keyswitch (~0.5 N, ~2 mm): confirm
   force *at* the working stroke (not headline force) and intermittent duty at
   the fastest click rate (near boil).
3. **LM35 low-end:** reads down to ~+2 °C with a GND pull-down; true 0 °C needs a
   negative supply. Decide if that matters for the dial's bottom mark.
4. **Sensor sealing/placement:** the LM35 must be waterproofed; element-mounted
   (fast, overshoots) vs water-immersed (accurate, laggy) changes hysteresis feel.
5. **VREF_PRECISE accuracy** now sets setpoint accuracy directly (no ratiometric
   cancellation) — use a ≤0.5% reference and the master trim RT to null offset.
```
