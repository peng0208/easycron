use std::time::Duration;

use anyhow::Result;
use easycron::*;
use tokio::time::sleep;

#[tokio::main]
// 定义一个异步的主函数
async fn main() -> Result<()> {
    // 构建一个Cron调度器实例，开启调试模式
    let mut crond = CronBuilder::default().debug(true).build().unwrap();
    // 启动Cron调度器
    crond.run().await;

    // 创建第1个任务，每秒触发一次，执行一个简单的打印操作
    let j1 = Job::new("test", "* * * * * *", || println!("test"))?;
    crond.add(j1).await;

    // 创建第2个任务
    sleep(Duration::from_secs(3)).await;
    let j2 = Job::new("test2", "* * * * * *", || println!("test2"))?;
    crond.add(j2.clone()).await;

    // 移除第2个任务
    sleep(Duration::from_secs(3)).await;
    crond.remove(j2.id).await;

    // 按下第1次Ctrl+C停止调度器
    tokio::signal::ctrl_c().await?;
    crond.stop();

    // 按下第2次Ctrl+C信号退出进程
    tokio::signal::ctrl_c().await?;
    Ok(())
}
