# Projet Wave Player 2025-2026 

Répertoire GitHub: [https://github.com/fusetim/rusty-soc](https://github.com/fusetim/rusty-soc)

## Introduction

À l'origine, le projet est supposé être réalisé en C/C++ avec des outils déjà livrés (SD, FAT, OLED, ...) et Silice pour la partie hardware. 
Toutefois, pour rajouter un peu de fun et de défi, j'ai décidé de faire la partie firmware en Rust, ce qui a nécessité quelques efforts supplémentaires. En dehors de quelques lignes d'assembleur, tout le firmware est écrit en Rust, et est plus ou moins conforme à ce qu'on attendrait 
des bibliothèques PAC et HAL réalisées.

## Fonctionnalités

Par rapport aux fonctionnalités attendues, les suivantes ont été implémentées :

##### Fonctions minimales

- [x] Menu avec classement par album (e.g. un par repertoire)
- [x] Lecture / Pause d'une musique
- [x] Arrêt de la lecture, Retour dans le menu
- [x] Affichage d'une image correspondant à l'album / la musique
- [x] Effet de lumiere sur les LEDs, en hardware (bonus: synchronisé avec la musique)  
      *Bug connu [#3](https://github.com/fusetim/rusty-soc/issues/3): L'energy-meter ne fonctionne correctement que lorsque le volume est au maximum.*

##### Fonctions bonus

- [x] Avance/Retour rapide  
      *Bug connu [#1](https://github.com/fusetim/rusty-soc/issues/1): Le seeking peut ne pas fonctionner correctement près du début ou de la fin du fichier*
- [x] Couleurs sur écran (RGB565)
- [x] Animation pendant la musique (LED: Energy Meter)
- [x] Volume sur LEDs (lors du changement avec boutons)

##### Améliorations techniques supplémentaires

- [x] Firmware en Rust (tout ce qui était en C/C++ a été réimplémenté ou remplacé par une alternative Rust)
- [x] Support de la mémoire "statique/globale" (sections ro_data/data et bss)
- [x] 2x Hardware SPI Master (un pour la SDCard, un pour l'OLED) basé sur [un périphérique SPI maison (en Verilog)](../hardware/lib/fusetim/spi/SpiMasterPeripheral.v)
- [x] Génération d'horloge basée sur PLL :
  - *25MHz pour le core,*
  - *80MHz pour le SPI0 (40MHz pour la SDCard),*
  - *20MHz pour le SPI1 (10MHz pour l'OLED)*
- [x] Audio PCM 48kHz (8bit, Mono) au lieu de 8kHz
- [x] Graphismes (Pas de framebuffer ni soft ni hard) utilisant la crate `embedded-graphics` et un driver SSD1351 maison (SPI-based)

## Compiler et lancer le projet

### Prérequis

Afin de compiler et lancer le projet, vous aurez besoin de :

- Silice
- Yosys
- Nextpnr (nextpnr-ecp5)
- Une toolchain Rust Nightly avec `cargo` et `rustup` (voir [rustup.rs](https://rustup.rs/) pour l'installation)
- La target Rust RISC-V RV32i: `rustup target add riscv32i-unknown-none-elf`
- riscv64-unknown-elf-gcc toolchain for linking steps
- make

### Compiler le firmware

La première étape est de compiler le firmware Rust. Vous pouvez le faire en exécutant dans le répertoire racine du projet :

```bash
cargo run -p silicon --release --target riscv32i-unknown-none-elf
```

Si tout se passe bien, cela compilera le code Rust et produira les fichiers suivants dans le répertoire `target/riscv32i-unknown-none-elf/release/` :
- `silicon`: le binaire ELF du firmware (pratique pour utiliser `size` ou `objdump` dessus)
- `silicon.mem`: le firmware converti en format hexadécimal, prêt à être flashé sur la mémoire du SoC  
  *À noter que ce fichier est prévu pour être chargé grâce à la commande `$readmemh` du [module `dmem`](../hardware/src/dmem.v).*[^1]

### Compiler le SoC et flasher la carte

Ensuite, vous pouvez compiler le SoC avec Silice et flasher la carte en suivant les instructions suivantes :

```bash
cd hardware
make clean
make soc BOARD=ulx3s
```

De manière additionnelle, il est possible de flasher le bitstream précédemment généré avec `openFPGALoader` :

```bash
openFPGALoader -b ulx3s BUILD/build.bit
```

## Utiliser le lecteur audio

Une fois le SoC flashé, vous pourrez utilser une carte SD (FAT32) comprenant les fichiers PCM (48kHz, 8bit, Mono) et 
les images (format RGB565) pour chaque titre, organisés dans des répertoires correspondant à soit des albums, soit des artistes.

Un exemple de structure de fichiers sur la carte SD pourrait être :

```
SDCard/
├── __SYS__/        /* Répertoire réservé pour les fichiers système (bootimage, assets) */
├── Album1/
|   ├── Track1/
|   ├── Track2/
|   |   └── art.raw     /* Image 128x128 RGB565 pour l'album */
|   |   └── music.raw   /* Fichier PCM (48kHz, 8bit / u8, Mono) pour la musique */
├── Album2/
...
```

> [!NOTE]  
> Il n'y a pas de tri automatique des titres / albums, il faut donc de préférence les écrire dans l'ordre dans lequel on souhaite les voir apparaître 
> dans le menu. 
> Par exemple, si on veut que "Track2" soit affiché avant "Track1", il faudra d'abord créer le répertoire "Track2" puis "Track1" dans l'album.

### Produire les fichiers PCM et les images

Pour produire les fichiers PCM, vous pouvez utiliser `ffmpeg` pour convertir des fichiers audio existants (MP3, WAV, etc.) en PCM 48kHz 8bit Mono. Par exemple :

```bash
ffmpeg -i input.mp3 -acodec pcm_u8 -ar 48000 -ac 1 -f u8 music.raw
```

Pour les images, `image2cpp` permet assez simplement de produire le fichier adapté.

* * *

***Notes:***

[^1]: Le module mémoire a été modifié pour utiliser mon propre module qui utilise la commande `$readmemh` pour initialiser la mémoire à partir du fichier `silicon.mem`. Contrairement à l'approche Silice avec la BRAM, cette méthode est significativement plus rapide à synthétiser (10 min -> 2min).