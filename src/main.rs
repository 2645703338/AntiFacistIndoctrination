use regex::Regex;
use crate::controllers::CheckResultRes::{NotStudied, Studied};
use crate::utils::StudyResult::{Unknown, Success, Duplicated};

mod api;
mod controllers;
mod types;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!(
    r#"
青年大学习自动学习程序
——Powered by Lanny.
        "#
  );
  let args = utils::get_args();
  if args.cookie.is_empty() {
    println!("请在命令行中输入cookie");
    return Ok(());
  }
  let mut res_ary = Vec::new();
  let len = args.cookie.len();
  for ck in args.cookie {
    let result = controllers::check_result_controller(ck.as_str()).await?;
    match result {
      Studied => {
        res_ary.push(Duplicated);
      }
      NotStudied(id) => {
        let res = controllers::antifa_controller(id, ck.as_str()).await?;
        let double_check = controllers::check_result_controller(ck.as_str()).await?;
        match double_check {
          Studied => {
            let regex = Regex::new(r"^.*/daxuexi/(.*)/.*$").unwrap();
            let hash = regex.captures(res.data.url.as_str());
            match hash {
              Some(cap) => res_ary.push(Success((cap[1]).to_string())),
              None => res_ary.push(Success(String::new()))
            }
          }
          NotStudied(_) => {
            res_ary.push(Unknown)
          }
        }
      }
    }
    if len > 1 && res_ary.len() < len {
      println!("等待 3 秒后继续运行下一个任务");
      tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
  }
  println!("运行结果：{:?}", res_ary);
  let mut images_str = String::new().to_owned();
  for res in &res_ary {
    match res {
      Success(id) =>
        images_str.push_str(
          format!("![screenshot](https://h5.cyol.com/special/daxuexi/{}/images/end.jpg)\n\n", id)
            .as_str()
        ),
      _ => continue
    }
  }
  controllers::send_message_controller(
    format!("运行结果：{:?}", res_ary).as_str(),
    Some(format!("{:?}\n\n{}", res_ary, images_str).as_str()),
  )
    .await?;
  Ok(())
}
