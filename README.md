<p align="center">&nbsp;<img align="center" src="https://github.com/user-attachments/assets/599d4962-2006-4ad1-8154-0bb7d5991cad" alt="logo" /></p>

---
<h1 align="center">A collection of ascii screensavers</h1>

## 🚀 Installation
### Dependencies
- A [Nerd Font](nerdfonts.com) (Optional)

### Linux/Macos specific
```sh
wget -O - "https://raw.githubusercontent.com/coderspark/asciisavers/refs/heads/master/install.sh" | sh
```

### Universal download
```sh
cargo install asciisavers
```

## 🧠 Features
<details>
<summary><h3>DVD</h3></summary>

Run it with the following command:
```sh
asciisavers dvd
```
#### Options
- -C --disablecount - Disable the corner counter
- -d --delay \<DELAY\>  - Set the delay in milliseconds
- -h --help - Print the help menu

</details>
<details>
<summary><h3>Flying Toasters</h3></summary>
  
Run it with the following command:
```sh
asciisavers toasters
```
</details>

<details>
<summary><h3>Bouncing Ball</h3></summary>
  
Run it with the following command:
```sh
asciisavers ball
```
#### Options
- -d --delay \<DELAY\> - Set the delay between frames in milliseconds
- -f --fancy - Enable fancy mode on the balls requires a [Nerd Font](https://www.nerdfonts.com/)
- -r --reset \<CHARS\> - Reset after *n* amount of characters [Default: 1000]
- -h --help - Print the help menu

</details>

<details>
<summary><h3>Pipes.rs</h3></summary>
  
Run it with the following command:
```sh
asciisavers pipes
```
#### Options
- -d --delay \<DELAY\> - Set the delay between frames in milliseconds
- -t --type \<TYPE\> - Set the type of the pipes 0-9 (can be used multiple times) [Default: [0]]
- -c --colour \<COLOUR\> - Set the colours of the pipes 0-7 (can be used multiple times) [Default: [0, 1, 2, 3 ,4 ,5 ,6 ,7]]
- -R --randomize - Randomize the starting position of the pipes
- -s --stats - Disable the stats of the pipes in the corner
- -h --help - Print the help menu

</details>

## 💭 Inspired By
- [Pipes.sh](https://github.com/pipeseroni/pipes.sh)
- [Cmatrix](https://github.com/abishekvashok/cmatrix)
- [ASCIIquarium](https://github.com/cmatsuoka/asciiquarium)

## 🤝 Contributing
We are okay with contributors of any skill level! If you have an idea for a screensaver or want to make a documention, go ahead!
