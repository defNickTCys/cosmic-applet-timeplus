# 🌍 Translation Status

This document tracks the translation status of **cosmic-applet-timeplus** across all supported languages.

## Translation Coverage

### ✅ Fully Translated (8 languages)

Professional translations with native language support:

| Language | Code | Translator | Status |
|----------|------|------------|--------|
| 🇩🇪 German | `de` | AI-assisted | Complete |
| 🇪🇸 Spanish (Spain) | `es-ES` | AI-assisted | Complete |
| 🇫🇷 French | `fr` | AI-assisted | Complete |
| 🇮🇹 Italian | `it` | AI-assisted | Complete |
| 🇷🇺 Russian | `ru` | AI-assisted | Complete |
| 🇯🇵 Japanese | `ja` | AI-assisted | Complete |
| 🇨🇳 Chinese (Simplified) | `zh-CN` | AI-assisted | Complete |
| 🇰🇷 Korean | `ko` | AI-assisted | Complete |

### 📝 English Fallback (53 languages)

These languages use English text as placeholder. **Community contributions welcome!**

<details>
<summary>Click to expand full list</summary>

| Language | Code | Native Speakers |
|----------|------|-----------------|
| Afrikaans | `af` | ~7M |
| Arabic | `ar` | ~310M |
| Belarusian | `be` | ~3M |
| Bulgarian | `bg` | ~8M |
| Bengali | `bn` | ~230M |
| Catalan | `ca` | ~10M |
| Czech | `cs` | ~10M |
| Danish | `da` | ~6M |
| Greek | `el` | ~13M |
| English (UK) | `en-GB` | ~60M |
| Esperanto | `eo` | ~2M |
| Spanish (Latin America) | `es-419` | ~400M |
| Spanish (Mexico) | `es-MX` | ~130M |
| Estonian | `et` | ~1M |
| Persian | `fa` | ~70M |
| Finnish | `fi` | ~5M |
| Western Frisian | `fy` | ~500K |
| Irish | `ga` | ~1M |
| Scottish Gaelic | `gd` | ~60K |
| Gujarati | `gu` | ~55M |
| Hebrew | `he` | ~9M |
| Hindi | `hi` | ~340M |
| Croatian | `hr` | ~5M |
| Hungarian | `hu` | ~13M |
| Indonesian | `id` | ~43M |
| Interlingue | `ie` | ~1K |
| Icelandic | `is` | ~350K |
| Javanese | `jv` | ~82M |
| Georgian | `ka` | ~4M |
| Kabyle | `kab` | ~5M |
| Kurdish (Kurmanji) | `kmr` | ~15M |
| Kannada | `kn` | ~44M |
| Limburgish | `li` | ~1M |
| Lithuanian | `lt` | ~3M |
| Norwegian Bokmål | `nb` | ~5M |
| Dutch | `nl` | ~24M |
| Norwegian Nynorsk | `nn` | ~500K |
| Polish | `pl` | ~45M |
| Portuguese | `pt` | ~220M |
| Romanian | `ro` | ~24M |
| Slovak | `sk` | ~5M |
| Serbian | `sr` | ~9M |
| Serbian (Cyrillic) | `sr-Cyrl` | ~9M |
| Serbian (Latin) | `sr-Latn` | ~9M |
| Swedish | `sv` | ~10M |
| Tamil | `ta` | ~75M |
| Thai | `th` | ~60M |
| Turkish | `tr` | ~80M |
| Ukrainian | `uk` | ~40M |
| Vietnamese | `vi` | ~85M |
| Chinese (Traditional) | `zh-TW` | ~23M |

</details>

## 🤝 Contributing Translations

We welcome community contributions to improve translations! Here's how you can help:

### For Existing Translations

If you're a native speaker and notice improvements needed in the translated languages:

1. Open an issue describing the problem
2. Submit a PR with corrections to `i18n/{language}/cosmic_applet_timeplus.ftl`

### For New Translations

To translate TimePlus to a language currently using English fallback:

1. Copy `i18n/en/cosmic_applet_timeplus.ftl` as reference
2. Edit `i18n/{language}/cosmic_applet_timeplus.ftl`
3. Translate all strings while keeping the keys unchanged
4. Test your translation locally
5. Submit a PR with your translation

### Translation Guidelines

- **Keep it concise**: UI text should be short and clear
- **Maintain formatting**: Keep placeholders like `{variable}` unchanged
- **Test locally**: Verify your translation displays correctly
- **Cultural adaptation**: Adapt idioms and expressions when needed
- **Consistency**: Use consistent terminology throughout

## 📋 Strings to Translate

Each language file contains the following strings:

### Tab Labels
- `calendar` - Calendar tab
- `weather` - Weather tab  
- `timer` - Timer tab

### Settings
- `datetime-settings` - Settings menu item

### Weather Placeholder
- `weather-subtitle` - Weather widget subtitle
- `weather-placeholder-message` - Coming soon message
- `weather-placeholder-features` - Features heading
- `weather-feature-current` - Current conditions feature
- `weather-feature-temperature` - Temperature feature
- `weather-feature-forecast` - Forecast feature

### Timer Placeholder
- `timer-subtitle` - Timer widget subtitle
- `timer-placeholder-message` - Coming soon message
- `timer-placeholder-features` - Features heading
- `timer-feature-countdown` - Countdown timer feature
- `timer-feature-presets` - Presets feature
- `timer-feature-notifications` - Notifications feature
- `timer-feature-persistent` - Persistence feature

## 🎯 Translation Priority

Based on native speaker population, these languages would have the highest impact:

1. 🇮🇳 **Hindi** (`hi`) - 340M speakers
2. 🇪🇸 **Spanish (Latin America)** (`es-419`) - 400M speakers
3. 🇧🇩 **Bengali** (`bn`) - 230M speakers
4. 🇵🇹 **Portuguese** (`pt`) - 220M speakers
5. 🇮🇩 **Indonesian** (`id`) - 43M speakers

---

**Last Updated**: 2026-01-01  
**Total Languages**: 61  
**Translated**: 8 (13%)  
**Fallback**: 53 (87%)
