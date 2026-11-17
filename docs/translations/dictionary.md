# Translation dictionary: English → Polish

The terms Kid Mode uses, with the Polish word to use for each. Reuse these in
every new or changed Polish text, so the mod reads the same everywhere and
matches the game.

Rule of thumb: when vanilla Minecraft has an official Polish name for a thing
(a mob, an item, an effect, a game concept), use exactly that name. The
official names come from the game's own `assets/minecraft/lang/pl_pl.json`
(game version 26.3); the key is given so it can be checked again after an
update. Terms vanilla has no word for are the mod's own choices, marked
**mod**.

## How the mod is translated

- The texts live in `mixins/mord/runtime.rs` (`EN_US`, `PL_PL`) and, as
  vanilla-style language files, in `mixins/mord/lang/en_us.json` and
  `pl_pl.json`. `cargo test` fails when the two differ, or when a language is
  missing a key.
- Keys: `mord.name`, `mord.description`, `mord.setting.<setting file>`. The
  ON/OFF state uses vanilla's own `options.on` / `options.off`, so it reads
  "Wł." / "Wył." like the game's other buttons.
- The game's current language is checked first, so a resource pack with
  `assets/mord/lang/<code>.json` can change any text or add a language.
- Adding a language: add a table to `runtime.rs` and to `LANGUAGES`, and its
  `lang/<code>.json`.

## Style

- Sentence case, as vanilla Polish: "Nieśmiertelny gracz", not
  "Nieśmiertelny Gracz". Proper names keep their capitals: "Smok Endu".
- Short, plain words a child understands. A button is 310 pixels wide, so a
  label with its state ("…: Wył.") should stay under about 45 characters.
- Adjectives agree with the noun: nieśmiertelny gracz, nieśmiertelne
  zwierzęta, nieśmiertelni wrogowie, przyjazne stworzenia.

## The mod and its settings

| English | Polish | Source |
|---|---|---|
| Kid Mode (mod name) | Tryb dla dzieci | **mod**, after vanilla's "Tryb kreatywny", "Tryb przetrwania" |
| Invulnerable player | Nieśmiertelny gracz | **mod** |
| Friendly enemies | Przyjaźni wrogowie | **mod** |
| Friendly neutral mobs | Przyjazne neutralne stworzenia | **mod** |
| Friendly mobs fight other mobs | Przyjazne stworzenia walczą z innymi | **mod**, shortened to fit the button |
| Invulnerable animals | Nieśmiertelne zwierzęta | **mod** |
| Invulnerable enemies | Nieśmiertelni wrogowie | **mod** |
| Invulnerable Ender Dragon | Nieśmiertelny Smok Endu | **mod** + `entity.minecraft.ender_dragon` |
| Always sunny | Zawsze słonecznie | **mod** |
| Auto sleep | Automatyczny sen | **mod** |
| ON / OFF | Wł. / Wył. | `options.on`, `options.off` |

## General terms

| English | Polish | Source / note |
|---|---|---|
| mob(s) | stworzenie, stworzenia | `gamerule.category.mobs` "Stworzenia". Not "moby" |
| hostile mobs | agresywne stworzenia | `options.difficulty.peaceful.info` |
| neutral mobs | neutralne stworzenia | `options.difficulty.peaceful.info` |
| enemies (the mod's word for hostile mobs) | wrogowie | **mod** |
| animals | zwierzęta | **mod** |
| friendly | przyjazny, przyjazne | **mod** |
| invulnerable | nieśmiertelny | **mod**: the mob or player takes no damage (only `/kill` and the void still work) |
| player | gracz | `entity.minecraft.player` |
| boss(es) | boss(owie) | **mod** |
| undead | nieumarli | **mod** |
| damage | obrażenia | |
| knockback | odrzut | `enchantment.minecraft.knockback` |
| hunger | głód | `effect.minecraft.hunger` |
| settings | ustawienia | `mco.configure.world.settings.title` |
| options | opcje | `options.title` |
| mods | mody | Horizon's `horizon.mods.*` |
| Creative Mode | Tryb kreatywny | `gameMode.creative` |
| Survival Mode | Tryb przetrwania | `gameMode.survival` |
| Peaceful (difficulty) | Pokojowy | `options.difficulty.peaceful` |
| Overworld | Nadziemie | `flat_world_preset.minecraft.overworld` |
| the Nether | Nether | `advancements.nether.root.title` |
| the End | End | `biome.minecraft.the_end` |
| the Void | Pustka | `biome.minecraft.the_void` |
| day / night | dzień / noc | |
| sleeping through the night | przesypianie nocy | `sleep.skipping_night` |
| rest (time since last rest) | sen (czas od ostatniego snu) | `stat.minecraft.time_since_rest` |
| weather cycle | cykl pogodowy | `gamerule.doWeatherCycle` |
| daylight cycle | cykl dnia i nocy | `gamerule.doDaylightCycle` |
| clear weather / rain / thunderstorm | bezchmurna pogoda / deszcz / burza | `commands.weather.set.*`, `block.minecraft.bed.no_sleep` |
| fire / lava / water | ogień / lawa / woda | `block.minecraft.fire`, `.lava`, `.water` |
| Fire Resistance | Ognioodporność | `effect.minecraft.fire_resistance` |
| Potion of Fire Resistance | Mikstura ognioodporności | `item.minecraft.potion.effect.fire_resistance` |
| Flint and Steel | Krzesiwo | `item.minecraft.flint_and_steel` |
| End Crystal | Kryształ Endu | `item.minecraft.end_crystal` |
| Arrow | Strzała | `item.minecraft.arrow` |
| Wither Skull | Czaszka withera | `entity.minecraft.wither_skull` |
| Bed | Łóżko | `block.minecraft.white_bed` "Białe łóżko" |
| Chest | Skrzynia | `block.minecraft.chest` |
| Honey Bottle | Butelka miodu | `item.minecraft.honey_bottle` |
| Gold Ore / Gold Ingot | Złoże złota / Sztabka złota | `block.minecraft.gold_ore`, `item.minecraft.gold_ingot` |
| spawn phantoms | występowanie fantomów | `gamerule.doInsomnia` |

### Harmful effects (the ones friendly mobs can't give you)

| English | Polish |
|---|---|
| Blindness | Oślepienie |
| Darkness | Mrok |
| Hunger | Głód |
| Levitation | Lewitacja |
| Mining Fatigue | Wyczerpanie |
| Nausea | Mdłości |
| Poison | Zatrucie |
| Slowness | Spowolnienie |
| Weakness | Słabość |
| Wither (effect) | Obumieranie |

Keys: `effect.minecraft.<id>`.

## Mobs

All from `entity.minecraft.<id>`. The group is the mod's (see
docs/features.md).

### Enemies (hostile)

| English | Polish |
|---|---|
| Blaze | Płomyk |
| Bogged | Bagniak |
| Breeze | Wichrzyk |
| Cave Spider | Pająk jaskiniowy |
| Creaking | Trzeszcz |
| Creeper | Creeper |
| Drowned | Utopiec |
| Elder Guardian | Prastrażnik |
| Ender Dragon | Smok Endu |
| Endermite | Endermit |
| Evoker | Przywoływacz |
| Ghast | Ghast |
| Giant | Olbrzym |
| Guardian | Strażnik |
| Hoglin | Hoglin |
| Husk | Posuch |
| Illusioner | Iluzjoner |
| Magma Cube | Kostka magmy |
| Parched | Oschlak |
| Phantom | Fantom |
| Piglin Brute | Piglin okrutnik |
| Pillager | Rozbójnik |
| Ravager | Dewastator |
| Shulker | Shulker |
| Silverfish | Rybik cukrowy |
| Skeleton | Szkielet |
| Slime | Szlam |
| Spider | Pająk |
| Stray | Tułacz |
| Vex | Dręczyciel |
| Vindicator | Obrońca |
| Warden | Nadzorca |
| Witch | Wiedźma |
| Wither | Wither |
| Wither Skeleton | Witherowy szkielet |
| Zoglin | Zoglin |
| Zombie | Zombie |
| Zombie Nautilus | Łodzik-zombie |
| Zombie Villager | Osadnik-zombie |

### Neutral

| English | Polish |
|---|---|
| Bee | Pszczoła |
| Dolphin | Delfin |
| Enderman | Enderman |
| Goat | Koza |
| Iron Golem | Żelazny golem |
| Llama | Lama |
| Panda | Panda |
| Piglin | Piglin |
| Polar Bear | Niedźwiedź polarny |
| Trader Llama | Lama handlarza |
| Wolf | Wilk |
| Zombified Piglin | Zzombifikowany piglin |

### Animals

The neutral mobs that are animals too (bee, dolphin, goat, llama, panda,
polar bear, trader llama, wolf) are listed above only.

| English | Polish |
|---|---|
| Armadillo | Pancernik |
| Axolotl | Aksolotl |
| Bat | Nietoperz |
| Camel | Wielbłąd |
| Cat | Kot |
| Chicken | Kura |
| Cod | Dorsz |
| Cow | Krowa |
| Donkey | Osioł |
| Fox | Lis |
| Frog | Żaba |
| Glow Squid | Świecąca kałamarnica |
| Happy Ghast | Wesoły ghast |
| Horse | Koń |
| Mooshroom | Mooshroom |
| Mule | Muł |
| Nautilus | Łodzik |
| Ocelot | Ocelot |
| Parrot | Papuga |
| Pig | Świnia |
| Pufferfish | Rozdymka |
| Rabbit | Królik |
| Salmon | Łosoś |
| Sheep | Owca |
| Skeleton Horse | Koń-szkielet |
| Sniffer | Niuchacz |
| Squid | Kałamarnica |
| Strider | Magmołaz |
| Tadpole | Kijanka |
| Tropical Fish | Ryba tropikalna |
| Turtle | Żółw |
| Zombie Horse | Koń-zombie |

### Others the docs mention

| English | Polish |
|---|---|
| Villager | Osadnik |
