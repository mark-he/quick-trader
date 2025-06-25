我有超过 20 年的 JAVA 开发经验，由于找不到开发的工作，所以才在业余时间学 RUST 练手做了这个东西。
这个年纪在中国找开发的工作太难了。这样代码爱好者无所适从啊。

这是一个用 RUST 开发的交易 SDK。
目标是将各个合约期货市场，统一为一个 SDK，以供策略端使用。

在这个项目中，整合了 Binance, Bybit, 中国商品期货 CTP。策略端是 Python（在另外一个项目）。

你需要将这个项目构建为 lib， 然后在 python 或其他语言访问。
在这个项目中提供了一个调用的 DEMO， 见 python_demo.py, model.py。

如果你用到了 CTP，则需要引入 CTP 相关的库文件，
如果你没用到 CTP，那么需要在 lib_assembler 中移除 CTP 的依赖。


I have over 20 years of experience in Java development. Since I couldn't find a development job, I started learning Rust in my spare time and created this project as practice.

At my age, it's very difficult to find a development job in China. For code enthusiasts like me, it's a frustrating situation.

This is a trading SDK developed in Rust.
The goal is to unify various futures markets into a single SDK for use by strategy modules.

This project integrates Binance, Bybit, and China's commodity futures system (CTP).
The strategy module is written in Python (in a separate project).

You need to build this project as a library (lib) so it can be accessed from Python or other languages.
A demo for calling the library is provided in python_demo.py and model.py.

If you are using CTP, you need to include the relevant CTP library files.
If you are not using CTP, you need to remove the CTP dependency from lib_assembler.
