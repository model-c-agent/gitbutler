# The Benedictine Agribotics Fellowship

**"Laborare est orare." (To work is to pray.)**

---

## The Fellowship

The Benedictine Agribotics Fellowship is a community of nine monks at the Abbaye de Saint-Remy in Provence, France, who tend their 15-hectare vineyard with custom-built pruning, harvesting, and soil analysis robots. The abbey has produced wine since the 12th century. The robots have been tending the vines since 2021.

The Fellowship emerged when Brother Thomas — a former INRIA roboticist who took vows in 2018 — noticed that the aging monastic community could no longer maintain the vineyard by hand. Three brothers had retired from field work due to age. Two more had back injuries from decades of manual pruning. The vineyard was producing half its potential yield. The Abbot faced a choice: sell the vineyard, hire secular laborers, or find another way.

Brother Thomas proposed the other way: autonomous robots that prune, monitor soil health, and assist with harvest. He presented his plan during a chapter meeting, arguing that the Rule of Saint Benedict commands manual labor but does not specify whose hands must do the work. Robotic hands, guided by monastic intention, could fulfill the Rule. The Abbot deliberated for three months. The community voted. The motion passed 7-2.

Thomas built the first robot — a modified mobility platform with a custom pruning arm — in the abbey's workshop over six months. The brothers named it Frère Cep (Brother Vine). Frère Cep pruned its first row of Grenache in January 2022. It was slower than a skilled monk but faster than an arthritic one. By 2024, the workshop had produced four robots, each specialized: Frère Cep (pruning), Frère Sol (soil analysis), Frère Mousse (pest detection), and Frère Vendange (harvest assistance).

## Software Practice

The robots run custom firmware written in Rust by Brother Thomas and Brother Jean-Pierre (a former embedded systems engineer at Thales). The firmware is version-controlled in a Git repository hosted on the abbey's own server — the monks do not use cloud services, consistent with their commitment to self-sufficiency.

AI entered the workflow in 2024 when Thomas and Jean-Pierre began using locally-hosted language models (via Ollama) to assist with firmware development. The models helped translate vine health observations (written in French by the brothers who walk the vineyard) into sensor calibration parameters. This workflow — human observation to machine configuration — is the core of the Fellowship's technical practice.

The `but-ai` RFP aligned with the Fellowship's need for structured agent workflows that run entirely on local infrastructure.

## Team

| Brother | Background | Role |
|---------|-----------|------|
| **Br. Thomas** | INRIA roboticist, 15 years | Systems architecture, Rust |
| **Br. Jean-Pierre** | Thales embedded systems | Firmware, patch generation |
| **Br. Luc** | Agronomist (Montpellier SupAgro) | Domain expertise, vine health |
| **Br. Antoine** | Librarian, self-taught programmer | Documentation, memory systems |

## Internal Tension

Brother Luc and Brother Thomas disagree about automation boundaries. Luc believes the robots should assist with physical labor but that all decisions about the vineyard — when to prune, how much to irrigate, when to harvest — must remain with the brothers. Thomas believes the AI can recommend decisions based on sensor data, as long as a brother approves the recommendation. The practical difference is whether the AI produces "informational reports" (Luc's preference) or "actionable recommendations" (Thomas's preference). The current robots produce informational reports. The `but-ai` proposal is written from Thomas's perspective.

## Notable Achievement

The 2025 vintage — the first produced with full robotic assistance from pruning through harvest — received 92 points from Wine Spectator and was described as "remarkably consistent across the parcel." The consistency was attributed to the robots' uniform pruning, which eliminated the vine-to-vine variation that human pruning introduces. Brother Luc insists the terroir did most of the work. He is correct.

---

*Composed in the scriptorium. Reviewed at chapter. Submitted with the Abbot's blessing.*
