<!--suppress HtmlDeprecatedAttribute -->
<div align="center">
    <br>
    <img src="https://raw.githubusercontent.com/Asthowen/AFetch/main/.github/resources/banner.svg" align="center" alt="AFetch banner">
    <br>
    <br>
    <div>
        <a href="https://www.rust-lang.org/">
            <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Made with Rust">
        </a>
        <a href="https://github.com/Asthowen/AFetch/blob/main/LICENSE">
            <img src="https://img.shields.io/github/license/Asthowen/AFetch?style=for-the-badge" alt="License">
        </a>
        <a href="https://github.com/Asthowen/AFetch/stargazers">
            <img src="https://img.shields.io/github/stars/Asthowen/AFetch?style=for-the-badge" alt="Stars">
        </a>
    </div>
    <h3>
        <strong>A CLI system information tool written in Rust.</strong>
    </h3>
</div>

## Installation
### Install with Cargo
You can use cargo to install the latest version of Afetch:
```cargo
cargo install --locked afetch
```

### Build manually
Start by cloning the repo:
```bash
git clone https://github.com/Asthowen/AFetch.git
```
**For the next step you need to have Rust and cargo installed on your PC, for that follow the [official documentation](https://www.rust-lang.org/tools/install).**

Now switch to project folder and compile a release:
```bash
cd AFetch && cargo build --release
```

Your executable will be in the `target/release/` folder, it is named `afetch`.

## Configuration
### Locations of the configuration file
**Linux** -> `$XDG_CONFIG_HOME/afetch` or `$HOME/.config/afetch`
<br>
**Windows** -> `%APPDATA%\Roaming\afetch`
<br>
**MacOS** -> `$HOME/Library/Application Support/afetch`

### Configuration options
#### - Language
**Key name**: language
<br>
**Description**: The language used by AFetch.
<br>
**Available**: auto / fr / en
<br>
**Default**: auto

#### - Disable entries
**Key name**: disabled_entries
<br>
**Description**: List of entries to be deactivated.
<br>
**Available**: os / host / kernel / uptime / packages / resolution / desktop / desktop-version / shell / terminal / memory / cpu / cpu-usage / network / disk / disks / public-ip / battery / color-blocks
<br>
**Default**: network, battery, cpu-usage & public-ip
<br>
**Example**:
```yaml
disabled_entries:
  - battery
  - public-ip
  - network
```

#### - Logo
**Key name**: logo
<br>
**Description**: Allows you to customize the logo.
<br>
**Example**:
```yaml
logo:
  status: enable # disable / enable
  char_type: braille # braille / picture
  picture_path: none # `the file path: eg: ~/pictures/some.png` / none
```

#### - Text Color
**Key name**: text_color
<br>
**Description**: Allows you to customize the color of printed information.
<br>
**Example**:
```yaml
text_color:
  - 255 # r
  - 255 # g
  - 255 # b
```

#### - Text Header Color
**Key name**: text_color
<br>
**Description**: Allows you to customize the color of printed information header.
<br>
**Example**:
```yaml
text_color_header:
  - 133 # r
  - 218 # g
  - 249 # b
```

## Contributors
[<img width="45" src="https://avatars.githubusercontent.com/u/59535754?v=4" alt="Asthowen">](https://github.com/Asthowen)
[<img width="45" src="https://avatars.githubusercontent.com/u/63391793?v=4" alt="SquitchYT">](https://github.com/SquitchYT)

## License
**[AFetch](https://github.com/Asthowen/AFetch) | [GNU General Public License v3.0](https://github.com/Asthowen/AFetch/blob/main/LICENSE)**