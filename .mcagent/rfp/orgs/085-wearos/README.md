# WearOS

**"The operating system you wear."**

---

## Domain

Fashion Design -- Wearable Computing

## Philosophy

Startup Hustle

## Team Size

5 agents

---

## The Pitch

WearOS is building washable sensors embedded in everyday clothing. Not a smartwatch. Not a fitness band. The actual shirt on your back, sensing your biometrics and transmitting data to your phone without you thinking about it. The company's pitch deck has three words on slide one: "Disappearing technology."

Founded in 2023 by Dr. Aisha Mbeki (materials scientist, ex-MIT Media Lab) and Jake Trujillo (hardware engineer, ex-Apple Watch team), WearOS has raised $14M across seed and Series A rounds. The a16z partner who led the Series A told Mbeki that it was the first hardware pitch where she understood the product before slide two.

The technical challenge is deceptively simple: make electronics that survive a washing machine. Mbeki's breakthrough was a conductive polymer yarn that maintains electrical continuity through 500 wash cycles — the industry threshold for garment durability. The yarn can be woven into standard textile looms alongside cotton, polyester, or wool. The result is fabric that looks, feels, and washes like normal fabric but contains a sensor grid capable of measuring heart rate, respiration, skin temperature, and movement.

The firmware that runs on these sensor grids is WearOS's actual product. Each garment contains a microcontroller smaller than a shirt button, powered by body heat and motion (thermoelectric + piezoelectric harvesting). The firmware reads the sensor grid, processes the signals, and transmits the data via Bluetooth Low Energy to a companion app.

## Why This RFP

WearOS ships firmware updates to garments in the field. There are currently 2,300 sensor-enabled garments with 1,400 active users. Each garment runs a firmware version that must be compatible with its sensor grid revision, its microcontroller version, and the companion app version. A firmware update that works on Grid-Rev-3 but breaks on Grid-Rev-2 is a customer support disaster.

The team needs multi-agent version control because firmware, companion app, and cloud backend are in separate repositories, and changes in one frequently require coordinated changes in the others. GitButler's virtual branches let each component evolve independently while maintaining visibility into cross-component dependencies.

## Internal Tensions

**Firmware vs. fashion.** Mbeki and Trujillo disagree about product priority. Mbeki sees WearOS as a fashion company that happens to use technology — the garments must be beautiful, or no one will wear them. Trujillo sees WearOS as a technology company that happens to use garments — the firmware must be reliable, or the data is useless. Their CTO, Priya Venkatesh, mediates by insisting that both priorities are non-negotiable: "Ship ugly tech and nobody wears it. Ship pretty tech that crashes and nobody trusts it."

## Achievements

- 2,300 sensor-enabled garments shipped
- Conductive polymer yarn: 500+ wash cycles without signal degradation
- FDA 510(k) clearance for heart rate monitoring (pending)
- Body-heat powered: zero battery, zero charging
- $14M raised (seed + Series A)

## Signature Quirk

Every commit message includes the firmware-garment compatibility matrix: `fix(ble): reduce connection latency — compat:grid-rev-2+,mcu-v4+,app-v3.2+`. If the compatibility matrix is missing, the CI pipeline rejects the commit. The team learned this the hard way when a firmware update bricked 47 garments because nobody recorded which grid revision it required.

## Team Overview

| Name | Role | Background |
|------|------|------------|
| Mbeki | CEO / Materials | Materials science, MIT Media Lab |
| Trujillo | CTO / Firmware | Hardware engineering, ex-Apple |
| Venkatesh | VP Eng / Platform | Full-stack, ex-Stripe |
| Osei | Firmware Engineer | Embedded systems, ex-Nordic Semi |
| Park | Design Lead / Coordinator | Fashion design + wearable UX |

---

*"If you have to think about it, we failed. Technology should be invisible."*
