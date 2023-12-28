# Details

Date : 2023-12-28 16:22:16

Directory c:\\Users\\User\\Documents\\rust_learning\\connect_four

Total : 54 files,  916 codes, 24 comments, 221 blanks, all 1161 lines

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)

## Files
| filename | language | code | comment | blank | total |
| :--- | :--- | ---: | ---: | ---: | ---: |
| [Cargo.lock](/Cargo.lock) | TOML | 130 | 2 | 20 | 152 |
| [Cargo.toml](/Cargo.toml) | TOML | 7 | 1 | 2 | 10 |
| [src/board.rs](/src/board.rs) | Rust | 392 | 14 | 100 | 506 |
| [src/board/game_state.rs](/src/board/game_state.rs) | Rust | 7 | 0 | 2 | 9 |
| [src/board/player_color.rs](/src/board/player_color.rs) | Rust | 10 | 0 | 3 | 13 |
| [src/lib.rs](/src/lib.rs) | Rust | 1 | 0 | 1 | 2 |
| [src/main.rs](/src/main.rs) | Rust | 7 | 0 | 7 | 14 |
| [src/pve_game.rs](/src/pve_game.rs) | Rust | 83 | 0 | 24 | 107 |
| [src/pve_game/engine.rs](/src/pve_game/engine.rs) | Rust | 151 | 7 | 39 | 197 |
| [src/pve_game/engine/zobrist_hash.rs](/src/pve_game/engine/zobrist_hash.rs) | Rust | 39 | 0 | 7 | 46 |
| [src/pvp_game.rs](/src/pvp_game.rs) | Rust | 46 | 0 | 16 | 62 |
| [target/.rustc_info.json](/target/.rustc_info.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/cfg-if-818b22f8a5582b22/lib-cfg-if.json](/target/debug/.fingerprint/cfg-if-818b22f8a5582b22/lib-cfg-if.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/cfg-if-eab9226ac7bafd83/lib-cfg-if.json](/target/debug/.fingerprint/cfg-if-eab9226ac7bafd83/lib-cfg-if.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/connect_four-023400703abe073f/test-lib-connect_four.json](/target/debug/.fingerprint/connect_four-023400703abe073f/test-lib-connect_four.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/connect_four-0c1f72fbaea3e0a9/lib-connect_four.json](/target/debug/.fingerprint/connect_four-0c1f72fbaea3e0a9/lib-connect_four.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/connect_four-3f8701bed1af8069/lib-connect_four.json](/target/debug/.fingerprint/connect_four-3f8701bed1af8069/lib-connect_four.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/connect_four-56bcd603b7d2b985/test-bin-connect_four.json](/target/debug/.fingerprint/connect_four-56bcd603b7d2b985/test-bin-connect_four.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/connect_four-682b05c884c8e2b4/test-lib-connect_four.json](/target/debug/.fingerprint/connect_four-682b05c884c8e2b4/test-lib-connect_four.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/connect_four-6ed9c9c606aa7cb5/lib-connect_four.json](/target/debug/.fingerprint/connect_four-6ed9c9c606aa7cb5/lib-connect_four.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/connect_four-6ffa11d6f72a4516/test-lib-connect_four.json](/target/debug/.fingerprint/connect_four-6ffa11d6f72a4516/test-lib-connect_four.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/connect_four-745b252998505b24/test-lib-connect_four.json](/target/debug/.fingerprint/connect_four-745b252998505b24/test-lib-connect_four.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/connect_four-7b1f91997c8ae634/lib-connect_four.json](/target/debug/.fingerprint/connect_four-7b1f91997c8ae634/lib-connect_four.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/connect_four-945c03f4a5267485/bin-connect_four.json](/target/debug/.fingerprint/connect_four-945c03f4a5267485/bin-connect_four.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/connect_four-95627766af99abde/test-bin-connect_four.json](/target/debug/.fingerprint/connect_four-95627766af99abde/test-bin-connect_four.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/connect_four-a1bdd7d10220364d/test-bin-connect_four.json](/target/debug/.fingerprint/connect_four-a1bdd7d10220364d/test-bin-connect_four.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/connect_four-ad05fcccd90344f6/bin-connect_four.json](/target/debug/.fingerprint/connect_four-ad05fcccd90344f6/bin-connect_four.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/connect_four-aeeba1bbb642702f/bin-connect_four.json](/target/debug/.fingerprint/connect_four-aeeba1bbb642702f/bin-connect_four.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/connect_four-ca537631922be304/test-lib-connect_four.json](/target/debug/.fingerprint/connect_four-ca537631922be304/test-lib-connect_four.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/connect_four-d94392cd40bd7254/bin-connect_four.json](/target/debug/.fingerprint/connect_four-d94392cd40bd7254/bin-connect_four.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/connect_four-e2ac9e27d7d9a575/lib-connect_four.json](/target/debug/.fingerprint/connect_four-e2ac9e27d7d9a575/lib-connect_four.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/connect_four-e9fea0deeba0d373/test-bin-connect_four.json](/target/debug/.fingerprint/connect_four-e9fea0deeba0d373/test-bin-connect_four.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/connect_four-ef4c778ecb2d77bd/lib-connect_four.json](/target/debug/.fingerprint/connect_four-ef4c778ecb2d77bd/lib-connect_four.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/connect_four-f14d78dde0743b71/test-bin-connect_four.json](/target/debug/.fingerprint/connect_four-f14d78dde0743b71/test-bin-connect_four.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/getrandom-0804bcdf0fdecaa6/lib-getrandom.json](/target/debug/.fingerprint/getrandom-0804bcdf0fdecaa6/lib-getrandom.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/getrandom-4361fdd1fc00167a/lib-getrandom.json](/target/debug/.fingerprint/getrandom-4361fdd1fc00167a/lib-getrandom.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/nu-ansi-term-49163a70f890f42c/lib-nu-ansi-term.json](/target/debug/.fingerprint/nu-ansi-term-49163a70f890f42c/lib-nu-ansi-term.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/nu-ansi-term-b416bdf06d056d21/lib-nu-ansi-term.json](/target/debug/.fingerprint/nu-ansi-term-b416bdf06d056d21/lib-nu-ansi-term.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/ppv-lite86-1ff752d925b83a28/lib-ppv-lite86.json](/target/debug/.fingerprint/ppv-lite86-1ff752d925b83a28/lib-ppv-lite86.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/ppv-lite86-451d982bcb721595/lib-ppv-lite86.json](/target/debug/.fingerprint/ppv-lite86-451d982bcb721595/lib-ppv-lite86.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/rand-b08b3771112215ed/lib-rand.json](/target/debug/.fingerprint/rand-b08b3771112215ed/lib-rand.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/rand-f827d9bc70808490/lib-rand.json](/target/debug/.fingerprint/rand-f827d9bc70808490/lib-rand.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/rand_chacha-168d6db565f14912/lib-rand_chacha.json](/target/debug/.fingerprint/rand_chacha-168d6db565f14912/lib-rand_chacha.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/rand_chacha-3b34f01a38d77d33/lib-rand_chacha.json](/target/debug/.fingerprint/rand_chacha-3b34f01a38d77d33/lib-rand_chacha.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/rand_core-c63b4fd1a25aca0a/lib-rand_core.json](/target/debug/.fingerprint/rand_core-c63b4fd1a25aca0a/lib-rand_core.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/rand_core-dce6e40008f9090e/lib-rand_core.json](/target/debug/.fingerprint/rand_core-dce6e40008f9090e/lib-rand_core.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/windows-sys-97c39527e0de43f3/lib-windows-sys.json](/target/debug/.fingerprint/windows-sys-97c39527e0de43f3/lib-windows-sys.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/windows-sys-9bd495d53b8103d4/lib-windows-sys.json](/target/debug/.fingerprint/windows-sys-9bd495d53b8103d4/lib-windows-sys.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/windows-targets-124643dafcf27ba4/lib-windows-targets.json](/target/debug/.fingerprint/windows-targets-124643dafcf27ba4/lib-windows-targets.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/windows-targets-3eb01b83033930aa/lib-windows-targets.json](/target/debug/.fingerprint/windows-targets-3eb01b83033930aa/lib-windows-targets.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/windows_x86_64_msvc-56d328d50203674b/run-build-script-build-script-build.json](/target/debug/.fingerprint/windows_x86_64_msvc-56d328d50203674b/run-build-script-build-script-build.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/windows_x86_64_msvc-9cb6c844d0c61af1/build-script-build-script-build.json](/target/debug/.fingerprint/windows_x86_64_msvc-9cb6c844d0c61af1/build-script-build-script-build.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/windows_x86_64_msvc-dc19799fa9cad29c/lib-windows_x86_64_msvc.json](/target/debug/.fingerprint/windows_x86_64_msvc-dc19799fa9cad29c/lib-windows_x86_64_msvc.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/windows_x86_64_msvc-e331de0b14128e42/lib-windows_x86_64_msvc.json](/target/debug/.fingerprint/windows_x86_64_msvc-e331de0b14128e42/lib-windows_x86_64_msvc.json) | JSON | 1 | 0 | 0 | 1 |

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)