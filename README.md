# jjy-pi

電波時計を合わせるために標準電波（JJY）と同じ電波を出すためのソフトウェアです。

## jjy-core

特定のハードウェアに依存しない部分を切り出したライブラリクレートです。次の送信時刻の計算や、時刻に応じて送信する符号を決定するロジックが書かれています。

```sh
cargo test -p jjy-core
```

でテストが実行できます。

## jjy-pi

RasPiで動作させるためのバイナリクレートです。RasPiのクロックにあわせてGPIO4のLow/Highを切り替えます。  
Highになっている間だけ40kHzの電波を出せば標準電波と同じになります。

[cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild)があるとクロスコンパイルが容易です。

```sh
cargo zigbuild --target aarch64-unknown-linux-gnu --release
```
