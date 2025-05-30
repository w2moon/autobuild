# Autobuild

一个自动构建和发布工具，用于监控 Git 仓库变化并自动执行构建和发布命令。

## 功能特点

- 监控指定 Git 分支的更新
- 自动执行构建和发布命令
- 支持钉钉 webhook 通知
- 可配置的检查间隔时间
- 详细的构建日志和耗时统计

## 安装

```bash
cargo install autobuild
```

## 使用方法

1. 创建配置文件 `autobuild.json`（可选）：

```bash
# 使用 --init 命令创建默认配置文件
autobuild --init
```

或者手动创建配置文件：

```json
{
  "repository": ".",
  "build": "npm run build",
  "publish": "npm run publish",
  "branch": "main",
  "interval": 10,
  "webhook": {
    "url": "https://oapi.dingtalk.com/robot/send?access_token=YOUR_TOKEN",
    "prefix": "Autobuild"
  }
}
```

2. 运行程序：

```bash
# 使用默认配置
autobuild

# 指定配置文件
autobuild -c path/to/autobuild.json
```

## 配置说明

- `repository`: Git 仓库路径
- `build`: 构建命令
- `publish`: 发布命令
- `branch`: 监控的分支
- `interval`: 检查更新的间隔时间（秒）
- `webhook`: 钉钉机器人配置
  - `url`: 钉钉机器人 webhook 地址
  - `prefix`: 消息前缀

## 许可证

MIT
