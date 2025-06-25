QUICK-TRADER
======================= 
这是一个 RUST 编写的 期货交易 SDK。
它对接了 Binance 合约期货，Bybit 合约期货，以及中国期货 CTP。 我有超过 20 年的 JAVA 开发经验，这个 RUSK SDK 是学习 RUST 的练手之作。

This is a futures trading SDK written in Rust. 
It integrates with Binance Futures, Bybit Futures, and China's CTP futures trading system. I have over 20 years of experience in Java development, and this Rust SDK is a hands-on project I created to learn Rust.

# HOW TO USE
构建这个项目为 SO 或 DLL，用 Python、Rust 或其他语言调用。
请参照 python_demo.py

如果你用到了 CTP，请注意将相关的 CTP 库复制到同一个目录；
如果你没用到 CTP，请删除 spi 中对 cpi 的依赖，并移除相关代码。

Build this project as a shared library (SO or DLL) so it can be called from Python, Rust, or other languages.
Please refer to the file python_demo.py

If you are using CTP, please make sure to copy the relevant CTP libraries into the same directory.
If you are not using CTP, please remove the dependency on CTP in the spi module and delete the related code.


===== 以下内容请忽略 =====
===== Please ignore the following content =====

这个年纪已经找不到开发的工作了，在中国太难...

At this age, it's no longer possible to find a development job. It's too difficult in China..
