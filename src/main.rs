
use std::f32::consts::E;
// use std::collections::VecDeque;
use std::{collections::HashMap, fs, time::Duration};
use log::{info, warn, error};
use serde_json::{Map, Value};
// use tokio::{sync::broadcast::{self, Receiver}};
use open_order_alarm::adapters::binance::futures::http::actions::BinanceFuturesApi;
use open_order_alarm::adapters::binance::papi::http::actions::BinancePapiApi;
use open_order_alarm::adapters::bybit::futures::http::actions::ByBitFuturesApi;
// use open_order_alarm::base::ssh::SshClient;
use open_order_alarm::base::wxbot::WxbotHttpClient;
use open_order_alarm::base::slackbot::SlackHttpClient;
use open_order_alarm::actors::*;
// use test_alarm::models::http_data::*;

#[warn(unused_mut, unused_variables, dead_code)]
async fn real_time(
    binance: &Vec<Value>,
    symbols: &Vec<Value>,
    wx_robot: WxbotHttpClient,
) {
    //rece: &mut Receiver<&str>){
    info!("get ready for real time loop");
    let running = true;
    // let mut day_pnl = 0.0;

    let mut i = 0;
    // let mut end = 6;

    // 每个品种的上一个trade_id
    let mut last_trade_ids: HashMap<String, u64> = HashMap::new();
    for symbol_v in symbols {
        let symbol = String::from(symbol_v.as_str().unwrap());
        let symbol = format!("{}USDT", symbol);
        last_trade_ids.insert(symbol, 0);
    }

    // 交易历史
    // let trade_histories: VecDeque<Value> = VecDeque::new();

    // 净值数据
    // let mut net_worth_histories: VecDeque<Value> = VecDeque::new();

    info!("begin real time loop");
    // 监控循环
    loop {
        info!("again");
        // json对象
        // let mut response: Map<String, Value> = Map::new();
        // let mut json_data: Map<String, Value> = Map::new();
        let mut map: Map<String, Value> = Map::new();
        map.insert(String::from("productId"), Value::from("TRADER_001"));
        // let now = Utc::now();
        // let date = format!("{}", now.format("%Y/%m/%d %H:%M:%S"));

        // 监控服务器状态
        info!("server process");
        
        let res = trade_mapper::TradeMapper::get_positions();
        let weixin = trade_mapper::TradeMapper::get_weixin().unwrap();
       
        if let Ok(a) = res{

        for f_config in a {

            println!("tra_venue{}, name{}", &f_config.tra_venue, &f_config.name);

            let tra_name = &f_config.name;
            let tra_alarm = &f_config.alarm;
            let new_tra_id = &f_config.tra_id;

            
            if &f_config.tra_venue == "Binance" && &f_config.r#type == "Futures"{
                println!("等于Bianace{}", &f_config.tra_venue);
                // let binance_config = f_config.as_object().unwrap();
                // let key = "6vLVAQ2j53HvBoEeFfFDPUqe";

                // let mut slackrobot = String::from("https://hooks.slack.com/services/T0233U2HAHF/B05L8PDQM09/");
                // slackrobot.push_str(key);
                // let slack_robot = SlackHttpClient::new(&slackrobot);
                // slack_robot.send_text("这是一个测试消息", "一分钟内没有挂单").await;

            let binance_futures_api=BinanceFuturesApi::new(
                "https://fapi.binance.com",
                &f_config.api_key,
                &f_config.secret_key,
            );
            let name = tra_name;
            let alarm = tra_alarm;
            if name != "pca01"{
                for f_weixin in &weixin {
                    let tra_id = &f_weixin.tra_id;
                    let wx_hook = &f_weixin.wx_hook;
                    let slack_hook = &f_weixin.slack_hook;
                    let mess_hok = &f_weixin.mess_hook;
                    if new_tra_id == tra_id {
                        if wx_hook.len() != 0 {
                            let mut wxbot = String::from("https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=");
        wxbot.push_str(&wx_hook);
        info!("wxbot  {}", wxbot);
        let wx_robot = WxbotHttpClient::new(&wxbot);

        if alarm == "true"{
            if let Some(data) = binance_futures_api.get_open_orders(None).await {
                let v: Value = serde_json::from_str(&data).unwrap();
                let vec = v.as_array().unwrap();
                
                println!(" 名字{}",  name);
                if vec.len() == 0 {
                    if i != 0 {
                        let sender = format!("Binance交易所的----{}普通账号", name);
                        let content = format!("一分钟内没有新挂单");
                        wx_robot.send_text(&sender, &content).await;
                    }
                    continue;
    
                } else {
                  println!("当前有挂单{}", vec.len());
                }
                // net_worth = notional_total/ori_fund;
                // net_worth_histories.push_back(Value::from(new_account_object));
            } else {
                error!("Can't get bian_futures_opens {} account.", name);
            }

        }

                        }

                        if slack_hook.len() != 0 {
                            let mut slackrobot = String::from("https://hooks.slack.com/services/");
                slackrobot.push_str(&slack_hook);
                let slack_robot = SlackHttpClient::new(&slackrobot);


                if alarm == "true"{
                    if let Some(data) = binance_futures_api.get_open_orders(None).await {
                        let v: Value = serde_json::from_str(&data).unwrap();
                        let vec = v.as_array().unwrap();
                        
                        println!(" 名字{}",  name);
                        if vec.len() == 0 {
                            if i != 0 {
                                let sender = format!("Binance交易所的----{}普通账号", name);
                                let content = format!("一分钟内没有新挂单");
                                slack_robot.send_text(&sender, &content).await;
                            }
                            continue;
            
                        } else {
                          println!("当前有挂单{}", vec.len());
                        }
                        // net_worth = notional_total/ori_fund;
                        // net_worth_histories.push_back(Value::from(new_account_object));
                    } else {
                        error!("Can't get bian_futures_opens {} account.", name);
                    }
                }

                        }
                        


                    }
                }

                
            }
        }

        if &f_config.tra_venue == "ByBit"{
            println!("等于Bybit{}", &f_config.tra_venue);
            // let bybit_config = f_config.as_object().unwrap();
        let bybit_futures_api=ByBitFuturesApi::new(
            "https://api.bybit.com",
            &f_config.api_key,
            &f_config.secret_key,
        );
        let name = tra_name;
        let alarm = tra_alarm;
        let category = "spot";
        let category_linear = "linear";
            if alarm == "true"{
                let mut open_orders = 0;
                if let Some(data) = bybit_futures_api.get_bybit_open_orders(category).await {
                    let v: Value = serde_json::from_str(&data).unwrap();
                    let result = v.as_object().unwrap().get("result").unwrap().as_object().unwrap();
                    let vec = result.get("list").unwrap().as_array().unwrap();
                    println!("名字{}",name);
                    open_orders += vec.len();
                    println!("现货usdt挂单数量{:?}", vec.len());
                    // net_worth = notional_total/ori_fund;
                    // net_worth_histories.push_back(Value::from(new_account_object));
                } else {
                    error!("Can't get bybit_futures_spot_opens {} account.", name);
                }

                if let Some(data) = bybit_futures_api.get_bybit_usdc_open_orders().await {
                    let v: Value = serde_json::from_str(&data).unwrap();
                    let result = v.as_object().unwrap().get("result").unwrap().as_object().unwrap();
                    let vec_usdc = result.get("list").unwrap().as_array().unwrap();
                    
                    open_orders += vec_usdc.len();
                    println!("现货挂单数量{:?}", vec_usdc.len());
                    // net_worth = notional_total/ori_fund;
                    // net_worth_histories.push_back(Value::from(new_account_object));
                } else {
                    error!("Can't get bybit_futures_spot_opens {} account.", name);
                }


                

                


                if open_orders == 0 {
                    for f_weixin in &weixin {
                        let tra_id = &f_weixin.tra_id;
                        let wx_hook = &f_weixin.wx_hook;
                    let slack_hook = &f_weixin.slack_hook;
                        if new_tra_id == tra_id {

                            if wx_hook.len() !=0 {
                                let mut wxbot = String::from("https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=");
            wxbot.push_str(&wx_hook);
            info!("wxbot  {}", wxbot);
            let wx_robot = WxbotHttpClient::new(&wxbot);

            if alarm == "true" {
                if i != 0 {
                    let sender = format!("ByBit交易所的----{}现货账号", name);
                    let content = format!("一分钟内没有新挂单");
                    wx_robot.send_text(&sender, &content).await;
                }
            }


            
                                
                            }

                            if slack_hook.len() != 0 {
                                let mut slackrobot = String::from("https://hooks.slack.com/services/");
                slackrobot.push_str(&slack_hook);
                let slack_robot = SlackHttpClient::new(&slackrobot);


                if alarm == "true" {
                    if i != 0 {
                        let sender = format!("ByBit交易所的----{}现货账号", name);
                        let content = format!("一分钟内没有新挂单");
                        slack_robot.send_text(&sender, &content).await;
                    }
                }

                                

                            }
                            
            
                        }
                    }
                    
                    continue;
    
                } else {
                  println!("当前有现货挂单{}", open_orders);
                }

                if let Some(data) = bybit_futures_api.get_bybit_open_orders(category_linear).await {
                    let v: Value = serde_json::from_str(&data).unwrap();
                    let result = v.as_object().unwrap().get("result").unwrap().as_object().unwrap();
                    let vec = result.get("list").unwrap().as_array().unwrap();
                    
                    println!("名字{}", name);
                    if vec.len() == 0 {
                        if i != 0 {
                            for f_weixin in &weixin {
                                let tra_id = &f_weixin.tra_id;
                                let wx_hook = &f_weixin.wx_hook;
                    let slack_hook = &f_weixin.slack_hook;
                                if new_tra_id == tra_id {
                                    if wx_hook.len() != 0 {
                                        let mut wxbot = String::from("https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=");
                    wxbot.push_str(&wx_hook);
                    info!("wxbot  {}", wxbot);
                    let wx_robot = WxbotHttpClient::new(&wxbot);

                    if alarm == "true" {
                        let sender = format!("ByBit交易所的----{}期货账号", name);
                            let content = format!("一分钟内没有新挂单");
                            wx_robot.send_text(&sender, &content).await;
                    }


                                    }

                                    if slack_hook.len() != 0 {
                                        let mut slackrobot = String::from("https://hooks.slack.com/services/");
                slackrobot.push_str(&slack_hook);
                let slack_robot = SlackHttpClient::new(&slackrobot);

                if alarm == "true" {
                    let sender = format!("ByBit交易所的----{}期货账号", name);
                        let content = format!("一分钟内没有新挂单");
                        slack_robot.send_text(&sender, &content).await;
                }

                                    }
                                     
                    
                                }
                            }
                            
                        }
                        continue;
        
                    } else {
                      println!("当前有挂单{}", vec.len());
                    }
                    // net_worth = notional_total/ori_fund;
                    // net_worth_histories.push_back(Value::from(new_account_object));
                } else {
                    error!("Can't get bybit_futures_linear_opens {} account.", name);
                }
        }
    }



         
    if &f_config.tra_venue == "Binance" && &f_config.r#type == "Papi"{
        println!("等于Bianace{}", &f_config.tra_venue);
        // let binance_config = f_config.as_object().unwrap();
    let binance_papi_api=BinancePapiApi::new(
        "https://papi.binance.com",
        &f_config.api_key,
        &f_config.secret_key,
    );
    let name = tra_name;
    let alarm = tra_alarm;
        for f_weixin in &weixin {
            let tra_id = &f_weixin.tra_id;
            let wx_hook = &f_weixin.wx_hook;
            let slack_hook = &f_weixin.slack_hook;
            if new_tra_id == tra_id {

                if wx_hook.len() != 0 {
                    let mut wxbot = String::from("https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=");
wxbot.push_str(wx_hook);
info!("wxbot  {}", wxbot);
let wx_robot = WxbotHttpClient::new(&wxbot);

if alarm == "true"{
    if let Some(data) = binance_papi_api.get_open_orders(None).await {
        let v: Value = serde_json::from_str(&data).unwrap();
        let vec = v.as_array().unwrap();
        
        println!(" 名字{}",  name);
        if vec.len() == 0 {
            if i != 0 {
                let sender = format!("Binance交易所的----{}统一账号", name);
                let content = format!("一分钟内没有新挂单");
                wx_robot.send_text(&sender, &content).await;
            }
            continue;

        } else {
          println!("当前有挂单{}", vec.len());
        }
        // net_worth = notional_total/ori_fund;
        // net_worth_histories.push_back(Value::from(new_account_object));
    } else {
        error!("Can't get bian_papi_opens {} account.", name);
    }
}

                }


                if slack_hook.len() != 0 {
                    let mut slackrobot = String::from("https://hooks.slack.com/services/");
                slackrobot.push_str(&slack_hook);
                let slack_robot = SlackHttpClient::new(&slackrobot);


                if alarm == "true"{
                    if let Some(data) = binance_papi_api.get_open_orders(None).await {
                        let v: Value = serde_json::from_str(&data).unwrap();
                        let vec = v.as_array().unwrap();
                        
                        println!(" 名字{}",  name);
                        if vec.len() == 0 {
                            if i != 0 {
                                let sender = format!("Binance交易所的----{}统一账号", name);
                                let content = format!("一分钟内没有新挂单");
                                slack_robot.send_text(&sender, &content).await;
                            }
                            continue;
                
                        } else {
                          println!("当前有挂单{}", vec.len());
                        }
                        // net_worth = notional_total/ori_fund;
                        // net_worth_histories.push_back(Value::from(new_account_object));
                    } else {
                        error!("Can't get bian_papi_opens {} account.", name);
                    }
                }

                }
                


            }
        }

        
}

            

             
        }

        i += 1;
    }
        


        

        // 等待下次执行
        info!("waiting for next real time task...({})", 6000 * 10);
        tokio::time::delay_for(Duration::from_millis(6000 * 10)).await;
    }
}

#[warn(unused_mut, unused_variables)]
#[tokio::main]
async fn main() {
    // 日志
    log4rs::init_file("./log4rs.yaml", Default::default()).unwrap();

    init();
    // let time = format!("{}", Local::now().format("%Y/%m/%d %H:%M:%S"));

    // 测试用api
    // let api_key="JwYo1CffkOLqmv2sC3Qhe2Qu5GgzbeLVw2BxWB5HgK6tnmc8yGfkzLuDImBgDkXm";
    // let api_secret="7FtQARZqM2PDgIZ5plr3nwEVYBXXbvmSuvmpf6Viz9e7Cq2B87grRTG3VZQiEC5C";

    // 连接数据库
    // let config_db: Value =
    //     serde_json::from_str(&fs::read_to_string("./configs/database.json").unwrap()).unwrap();

    // 读取配置
    let config: Value = serde_json::from_str(
        &fs::read_to_string("./configs/total.json").expect("Unable to read file"),
    )
    .expect("Unable to parse");

    // 任务间通信信道
    // let (send, mut rece) = broadcast::channel(32);

    // 创建任务
    let real_time_handle = tokio::spawn(async move {
        // let mut futures_config: Map<String, Value> = Map::new();
        // let mut servers_config = Map::new();
        let binance_config = config.get("Binance").unwrap();
        let binance_future_config = binance_config.get("futures").unwrap().as_array().unwrap();
        // let server_config = config.get("Server").unwrap();
        let symbols = config.get("Symbols").unwrap().as_array().unwrap();

        let key = config.get("Alarm").unwrap().get("webhook").unwrap().as_str().unwrap();
        // info!("获取key");
        let res = trade_mapper::TradeMapper::get_weixin().unwrap();


        let mut wxbot = String::from("https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=");
        wxbot.push_str(key);
        info!("wxbot  {}", wxbot);
        let wx_robot = WxbotHttpClient::new(&wxbot);
        info!("preparing...");

        // for s_config in server_config{
        //     let obj = s_config.as_object().unwrap(); 
        //     let host = obj.get("host").unwrap().as_str().unwrap();
        //     let port = obj.get("port").unwrap().as_str().unwrap();
        //     let username = obj.get("username").unwrap().as_str().unwrap();
        //     let password = obj.get("password").unwrap().as_str().unwrap();
        //     let root_path = obj.get("root_path").unwrap().as_str().unwrap();
        //     let root_name = obj.get("root_name").unwrap().as_str().unwrap();
        //     servers_config.insert(String::from("host"), Value::from(host));
        //     servers_config.insert(String::from("port"), Value::from(port));
        //     servers_config.insert(String::from("username"), Value::from(username));
        //     servers_config.insert(String::from("password"), Value::from(password));
        //     servers_config.insert(String::from("root_path"), Value::from(root_path));
        //     servers_config.insert(String::from("root_name"), Value::from(root_name));
        // }
        
        
        
        // let ssh_api = SshClient::new(
        //     server_config.get("host").unwrap().as_str().unwrap(),
        //     server_config.get("port").unwrap().as_str().unwrap(),
        //     server_config.get("username").unwrap().as_str().unwrap(),
        //     server_config.get("password").unwrap().as_str().unwrap(),
        //     server_config.get("root_path").unwrap().as_str().unwrap(),
        //     server_config.get("root_name").unwrap().as_str().unwrap(),
        // );
        

        
        // for f_config in binance_future_config{
        //     let obj = f_config.as_object().unwrap(); 
        //     let base_url = obj.get("base_url").unwrap().as_str().unwrap();
        //     let api_key = obj.get("api_key").unwrap().as_str().unwrap();
        //     let secret_key = obj.get("secret_key").unwrap().as_str().unwrap();
        //     futures_config.insert(String::from("base_url"), Value::from(base_url));
        //     futures_config.insert(String::from("api_key"), Value::from(api_key));
        //     futures_config.insert(String::from("secret_key"), Value::from(secret_key));
        // }

        info!("created ssh client");
        // let binance_futures_api=BinanceFuturesApi::new(
        //     binance_config
        //         .get("futures")
        //         .unwrap()
        //         .get("base_url")
        //         .unwrap()
        //         .as_str()
        //         .unwrap(),
        //     binance_config
        //         .get("futures")
        //         .unwrap()
        //         .get("api_key")
        //         .unwrap()
        //         .as_str()
        //         .unwrap(),
        //     binance_config
        //         .get("futures")
        //         .unwrap()
        //         .get("secret_key")
        //         .unwrap()
        //         .as_str()
        //         .unwrap(),
        // );

        
        info!("created http client");

            real_time(binance_future_config, symbols, wx_robot).await;
        
    });

    // 开始任务
    info!("alarm begin(binance account)");
    real_time_handle.await.unwrap();
    info!("alarm done");
}
