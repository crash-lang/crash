<a href="https://crash-lang.org">
<p align="center">
<img src="https://raw.githubusercontent.com/crash-lang/branding/main/logo/logo_red.svg" alt="logo" width="350"/>
</p>
</a>

<h1 align="center">The Crash Programming Language Runtime</h1>

<p align="center">
<a href="https://crash-lang.org"><img src="https://img.shields.io/badge/website-%23.svg?style=for-the-badge&logoColor=white&color=%23F94144&logo=data:image/svg%2bxml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIGhlaWdodD0iMWVtIiB2aWV3Qm94PSIwIDAgNjQwIDUxMiI+PCEtLSEgRm9udCBBd2Vzb21lIFBybyA2LjQuMiBieSBAZm9udGF3ZXNvbWUgLSBodHRwczovL2ZvbnRhd2Vzb21lLmNvbSBMaWNlbnNlIC0gaHR0cHM6Ly9mb250YXdlc29tZS5jb20vbGljZW5zZSAoQ29tbWVyY2lhbCBMaWNlbnNlKSBDb3B5cmlnaHQgMjAyMyBGb250aWNvbnMsIEluYy4gLS0+PGRlZnM+PHN0eWxlPi5mYS1zZWNvbmRhcnl7b3BhY2l0eTowLjQ7ZmlsbDojZmZmO30uZmEtcHJpbWFyeXtmaWxsOiNmZmY7fTwvc3R5bGU+PC9kZWZzPjxwYXRoIGNsYXNzPSJmYS1wcmltYXJ5IiBkPSJNMzA0IDY0YTE0NCAxNDQgMCAxIDAgMCAyODggMTQ0IDE0NCAwIDEgMCAwLTI4OHpNMTExIDM2N2MtOS40IDkuNC05LjQgMjQuNiAwIDMzLjlzMjQuNiA5LjQgMzMuOSAwbDE4LjUtMTguNWMzNC4zIDI3LjcgNzQuOSA0My44IDExNi41IDQ4LjNWNDY0SDE4NGMtMTMuMyAwLTI0IDEwLjctMjQgMjRzMTAuNyAyNCAyNCAyNEg0MjRjMTMuMyAwIDI0LTEwLjcgMjQtMjRzLTEwLjctMjQtMjQtMjRIMzI4VjQzMC43YzQ5LjEtNS4zIDk2LjgtMjYuNyAxMzQuNC02NC4zYzgxLjctODEuNyA4Ny4xLTIxMSAxNi4xLTI5OC45TDQ5NyA0OWM5LjQtOS40IDkuNC0yNC42IDAtMzMuOXMtMjQuNi05LjQtMzMuOSAwTDQyOC41IDQ5LjZjLTkuNCA5LjQtOS40IDI0LjYgMCAzMy45YzY4LjcgNjguNyA2OC43IDE4MC4yIDAgMjQ4LjlzLTE4MC4yIDY4LjctMjQ4LjkgMGMtOS40LTkuNC0yNC42LTkuNC0zMy45IDBMMTExIDM2N3oiLz48cGF0aCBjbGFzcz0iZmEtc2Vjb25kYXJ5IiBkPSIiLz48L3N2Zz4=" alt="label"/></a>
<a href="https://github.com/crash-lang/crasher"><img src="https://img.shields.io/badge/crasher-%23.svg?style=for-the-badge&logoColor=white&color=%23F3722C&logo=data:image/svg%2bxml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIGhlaWdodD0iMTYiIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjwhLS0hRm9udCBBd2Vzb21lIEZyZWUgNi41LjEgYnkgQGZvbnRhd2Vzb21lIC0gaHR0cHM6Ly9mb250YXdlc29tZS5jb20gTGljZW5zZSAtIGh0dHBzOi8vZm9udGF3ZXNvbWUuY29tL2xpY2Vuc2UvZnJlZSBDb3B5cmlnaHQgMjAyMyBGb250aWNvbnMsIEluYy4tLT48cGF0aCBmaWxsPSIjZmZmZmZmIiBkPSJNNDk1LjkgMTY2LjZjMy4yIDguNyAuNSAxOC40LTYuNCAyNC42bC00My4zIDM5LjRjMS4xIDguMyAxLjcgMTYuOCAxLjcgMjUuNHMtLjYgMTcuMS0xLjcgMjUuNGw0My4zIDM5LjRjNi45IDYuMiA5LjYgMTUuOSA2LjQgMjQuNmMtNC40IDExLjktOS43IDIzLjMtMTUuOCAzNC4zbC00LjcgOC4xYy02LjYgMTEtMTQgMjEuNC0yMi4xIDMxLjJjLTUuOSA3LjItMTUuNyA5LjYtMjQuNSA2LjhsLTU1LjctMTcuN2MtMTMuNCAxMC4zLTI4LjIgMTguOS00NCAyNS40bC0xMi41IDU3LjFjLTIgOS4xLTkgMTYuMy0xOC4yIDE3LjhjLTEzLjggMi4zLTI4IDMuNS00Mi41IDMuNXMtMjguNy0xLjItNDIuNS0zLjVjLTkuMi0xLjUtMTYuMi04LjctMTguMi0xNy44bC0xMi41LTU3LjFjLTE1LjgtNi41LTMwLjYtMTUuMS00NC0yNS40TDgzLjEgNDI1LjljLTguOCAyLjgtMTguNiAuMy0yNC41LTYuOGMtOC4xLTkuOC0xNS41LTIwLjItMjIuMS0zMS4ybC00LjctOC4xYy02LjEtMTEtMTEuNC0yMi40LTE1LjgtMzQuM2MtMy4yLTguNy0uNS0xOC40IDYuNC0yNC42bDQzLjMtMzkuNEM2NC42IDI3My4xIDY0IDI2NC42IDY0IDI1NnMuNi0xNy4xIDEuNy0yNS40TDIyLjQgMTkxLjJjLTYuOS02LjItOS42LTE1LjktNi40LTI0LjZjNC40LTExLjkgOS43LTIzLjMgMTUuOC0zNC4zbDQuNy04LjFjNi42LTExIDE0LTIxLjQgMjIuMS0zMS4yYzUuOS03LjIgMTUuNy05LjYgMjQuNS02LjhsNTUuNyAxNy43YzEzLjQtMTAuMyAyOC4yLTE4LjkgNDQtMjUuNGwxMi41LTU3LjFjMi05LjEgOS0xNi4zIDE4LjItMTcuOEMyMjcuMyAxLjIgMjQxLjUgMCAyNTYgMHMyOC43IDEuMiA0Mi41IDMuNWM5LjIgMS41IDE2LjIgOC43IDE4LjIgMTcuOGwxMi41IDU3LjFjMTUuOCA2LjUgMzAuNiAxNS4xIDQ0IDI1LjRsNTUuNy0xNy43YzguOC0yLjggMTguNi0uMyAyNC41IDYuOGM4LjEgOS44IDE1LjUgMjAuMiAyMi4xIDMxLjJsNC43IDguMWM2LjEgMTEgMTEuNCAyMi40IDE1LjggMzQuM3pNMjU2IDMzNmE4MCA4MCAwIDEgMCAwLTE2MCA4MCA4MCAwIDEgMCAwIDE2MHoiLz48L3N2Zz4=" alt="label"></a>
<a href="https://dc.crash-lang.org"><img src="https://img.shields.io/badge/discord-%23.svg?style=for-the-badge&logoColor=white&color=%23F3722C&logo=discord" alt="label"></a>
</p>

<b>This project is heavily work in progress! Don't expect anything to work.</b>

The **Crash Programming Language** is a statically typed language that embraces Object-Oriented Programming (OOP) principles
without imposing strict adherence to working with objects.
Positioned as a mid- to high-level language, Crash simplifies memory management by omitting a Garbage Collector (GC)
and instead relies on compile-time analysis of reference counts to determine the necessity of instances.

Known for its ease of learning, Crash borrows keywords from well-established languages like C,
Java, and Rust. Its syntax is familiar to those acquainted with programming fundamentals,
with a reduced set of keywords. Noteworthy is the absence of certain keywords like `void` or `private`,
as default behaviors handle these aspects.

Crash is an Ahead-of-Time (AOT) compiled language,
offering efficiency and performance. Notably, source-code files in Crash are identified by the `.crash` extension.
This language provides a balance between simplicity and power,
making it accessible to both beginners and experienced developers.

This is the main source code repository for [Crash](https://crash-lang.org).
It contains the [compiler](compiler_old) *(not [Crasher](https://github.com/crash-lang/crasher))* and [standard library](lib).


<h3>A Crash Language Project</h3>

This Project is licensed under the [Apache 2.0 License](LICENSE).
If you wish to contribute, please read [CONTRIBUTING.md](CONTRIBUTING.md).


<h1>Installation</h1>

Please visit [our Website](https://crash-lang.org/install) for install instructions.


<h1>Hello world</h1>

A Hello-world program in Crash:

```crash
main() {
    println("Hello world!");
}
```
Want more snippets? [Look here!](https://github.com/crash-lang/snippets)