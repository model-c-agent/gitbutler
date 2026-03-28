# HitForge

**"Ship hits, not demos."**

---

## The Pitch

HitForge is a Los Angeles-based startup that uses AI to predict whether a song will chart before it is finished. Founded in 2023 by ex-Spotify data scientist Maya Chen and producer-turned-engineer Devon "DQ" Quarles, HitForge claims a 73% accuracy rate on Billboard Hot 100 placement prediction using a proprietary model trained on 2.1 million tracks.

That number -- 73% -- is on every slide deck, every pitch meeting, every investor update. It is simultaneously the company's greatest asset and its biggest liability. Asset because no competitor has published a comparable figure. Liability because the model was validated on a test set that critics argue is not representative of new releases. Maya stands by the number. Devon thinks they should stop leading with it and start leading with the product. This disagreement is the engine that drives the company.

## Founding

Maya left Spotify in 2022 after her team's recommendation algorithm was deprioritized in favor of a simpler engagement-maximizing model. She believed the music industry was drowning in data but starving for prediction -- labels knew what people listened to yesterday but had no reliable way to predict what they would listen to tomorrow. She spent six months building a prototype predictor in a WeWork in Silver Lake, burning through her savings and three MacBook Pro keyboards.

Devon was a producer who had three songs chart in the 2010s but could not repeat the success. He understood, viscerally, the gap between making something good and making something that connects. When Maya's prototype correctly predicted that a song Devon was mixing would fail (it did), he quit producing and joined as CTO. His contribution is not code -- it is domain expertise. He knows what a hit feels like before the data confirms it.

They raised a $2.4M seed round from a music-tech fund and hired eight engineers. The product is a SaaS platform that ingests stems, analyzes them against the model, and produces a "hit probability score" with actionable feedback: tempo adjustments, arrangement suggestions, vocal mix recommendations. Labels pay $12,000/month for unlimited analysis. Three labels are currently paying. Seven more are in trial.

## Why This RFP

HitForge needs version control for their AI production pipeline. Their current workflow is a disaster: stems are stored in Google Drive, model configurations live in a shared Notion page, and production suggestions are communicated via Slack threads that no one can find two weeks later. Devon calls it "the demo tape era of engineering."

They see the `but-ai` plugin as an opportunity to build their production pipeline on proper infrastructure while contributing to a tool that other AI-first companies will use. Maya views it as a strategic investment: if GitButler becomes the standard for AI agent version control, HitForge wants to be the company that helped build it.

## Philosophy

1. **Speed over purity.** Ship it, measure it, fix it. A shipped product with known flaws beats an unshipped product with theoretical perfection.
2. **Data is not intuition.** The model informs decisions; it does not make them. A 73% prediction is 27% wrong, and that 27% is where artistry lives.
3. **Users first, architecture second.** Nobody cares how clean your code is if the product is slow.

## Team

| Name | Role | Background |
|------|------|------------|
| **Maya Chen** | CEO / ML Lead | Ex-Spotify, recommendation systems, 6 years |
| **Devon "DQ" Quarles** | CTO / Domain Expert | Producer, 3 Billboard entries, 15 years in music |
| **Aisha Bello** | Lead Engineer | Ex-SoundCloud, distributed systems |
| **Ravi Krishnamurthy** | ML Engineer | Ex-Google Brain, audio ML |

## Internal Tension

Maya wants to open-source the `but-ai` work completely -- she believes open infrastructure attracts talent and builds credibility. Devon wants to keep the integration proprietary, arguing that their production pipeline is a competitive advantage and open-sourcing it gives labels a reason to build in-house instead of paying HitForge. Aisha sides with Maya. Ravi sides with Devon. The current compromise: the plugin contribution is open-source, but the HitForge-specific production intelligence layer stays proprietary.

## Notable Achievement

In Q4 2025, HitForge's model correctly predicted 8 of the 11 tracks that debuted in the Billboard Hot 100 top 20 during a blind test conducted by Music Business Worldwide. The publication ran the story under the headline "Algorithm Knows Your Next Favorite Song." HitForge's monthly recurring revenue tripled in the following quarter. Devon framed the article. Maya framed the confusion matrix.

---

*"73% of the time, we are right every time."*
*-- Devon, at every board meeting, to Maya's visible annoyance*
