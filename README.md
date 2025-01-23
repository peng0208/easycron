# EasyCron

## 简介

`easycron` 是一个基于Rust Tokio 的轻量级任务调度库，支持通过 Crontab（秒级） 表达式定义定时任务，支持动态添加、删除、启动、停止定时任务。

## 安装

```
cargo add easycron
```

## 示例

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let mut crond = CronBuilder::default().debug(true).build().unwrap();

    crond.run().await;

    let j1 = Job::new("test", "* * * * * *", || println!("test"))?;
    crond.add(j1).await;
    
    sleep(Duration::from_secs(3)).await;
    crond.remove(j2.id).await;

    tokio::signal::ctrl_c().await?;
    Ok(())
}
```

