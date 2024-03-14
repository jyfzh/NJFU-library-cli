# NJFU-library-cli ![LICENSE](https://img.shields.io/badge/LICENSE-MIT-yellow)

NJFU图书馆研讨间预约,取消的命令行工具

![forthebadge](https://img.shields.io/badge/test_ok-2024/03/15-blue)
![wakatime](https://wakatime.com/badge/user/cfee0eb2-658b-4917-a1ed-9801e76b961f/project/896c2bad-d07b-4cfd-bf71-35a4cb5d13dc.svg)

![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)
![forthebadge](https://forthebadge.com/images/badges/built-with-love.svg)

## Install

```bash
git clone https://github.com/jyf-111/NJFU-library-cli.git
cd NJFU-library-cli
cargo build --release
cd target/release
./njfulib -h
```

## how to use

```bash
njfulib login -u <username> -p <password>
njfulib query -n <your name>
njfulib statue
njfulib reserve -s <space>... -d 2 --start <start time> --end <end time> -u <user>...
njfulib cancel -i <id>
```

## Example

```bash
njfulib loin -u 200855110 -p njfu******!
njfulib reserve -s 8A515 --start 8:00 --end 12:00 -d 1
```

[具体参数解释](https://github.com/jyf-111/NJFU-library-cli/wiki/参数解释)
