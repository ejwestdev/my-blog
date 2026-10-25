---
title = "Coding with AI"
date = "October 24th 2026"
---

My AI coding workflow has gone through a ton of changes in the past year or so. On some level, I think that any workflow would work fine. The models are pretty good and if there was some incredible optimization, the labs would probably figure it out. I don't really use a lot of skills, or AGENTS.md or CLAUDE.md. The model ignores it around 50% of the time anyway, and in large codebases those files quickly drift from reality.

I've used pretty much every mainstream model and harness combo. Right now I use OpenCode and GPT, but I've tried Claude Code, Codex, Pi, T3 Code, Autolith, even Antigravity. Along with all the big lab models, and some Chinese ones. It's really just personal preference, pretty much everything works. I like OpenCode's UI so that's what I go with now. I know some people don't like using the TUI agents, but I pretty much do everything in the terminal, so it's a logical choice for me.

In general my AI policy is to use it on things where I could implement it by hand. For serious projects I will read every line of code, often with [Plannotator](https://plannotator.ai/), which is a great skill, you should check it out.

Every time I stop reading the code the model starts drifting further and further. I've seen "sloppified" projects first hand, and they turn into disasters quickly. In a greenfield project, AI can help you make massive leaps at the start. But if you over rely on it your codebase inevitably turns into an unworkable mess, and you risk falling into [the decoherence trap.](https://maevyn.codes/decoherence)

You also need to know what you're doing with your prompts. For example, if your website has a bug where it jitters while searching, a layperson might say "the site is glitching out when I search". This isn't a great prompt for the model. Sure maybe you could fix it with that prompt, but you're much more likely to get a good result if you say "I think we need to add debounce logic to the search bar, there is a jitter on search". This is just a small example, but it shows how AI can really boost you if you know what you're doing, or send you down the wrong path if you don't.

AI is a massive productivity booster, but it's a fine line between increased productivity and taking on more tech debt than you can handle.
