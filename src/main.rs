
use std::collections::VecDeque;
use std::{collections::HashMap, fs, time::Duration};
use chrono::{DateTime, Utc, NaiveDateTime};
use log::{info, warn};
use serde_json::{Map, Value};
// use tokio::{sync::broadcast::{self, Receiver}};
use test_alarm::adapters::binance::futures::http::actions::BinanceFuturesApi;
use test_alarm::base::ssh::SshClient;
use test_alarm::base::wxbot::WxbotHttpClient;
use test_alarm::actors::*;
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
        // let mut server_status: VecDeque<Value> = VecDeque::new();
        // let mut server_process: Map<String, Value> = Map::new();
        // print!("判断是true还是false {}", ssh_api.search_py_ps());
        // match ssh_api.search_py_ps() {
        //     true => {
        //         if !running {
        //             running = true;
        //             print!("改变running的值{}", running);
        //             // let sender = "程序开启";
        //             // let content = format!("process name: {}", ssh_api.get_root_name());
        //             // wx_robot.send_text(sender, &content).await;
        //         }
        //         server_process.insert(String::from("status"), Value::from("running"));
        //         server_process.insert(String::from("info"), Value::from(""));
        //     }
        //     false => {
        //         server_process.insert(String::from("status"), Value::from("stopped"));
        //         let mut info = ssh_api.download_log();
        //         if running {
        //             running = false;
        //             // let sender = "程序停止";
        //             let content;
        //             if info == "" {
        //                 content = format!("{}: 未找到错误，请查看日志", ssh_api.get_root_name());
        //             }else {
        //                 content = format!("{}: {}", ssh_api.get_root_name(), &info);
        //             }
        //             // wx_robot.send_text(sender, &content).await;
        //             info = content;
        //         }
        //         server_process.insert(String::from("info"), Value::from(info));
        //     }
        // }
        // map.insert(String::from("server"), Value::from(server_process));


        print!("running的值是否被改变{}", running);

        for f_config in binance {
            let mut history_open_orders: VecDeque<Value> = VecDeque::new();
            
            let binance_config = f_config.as_object().unwrap();
            let binance_futures_api=BinanceFuturesApi::new(
                binance_config
                    .get("base_url")
                    .unwrap()
                    .as_str()
                    .unwrap(),
                binance_config
                    .get("api_key")
                    .unwrap()
                    .as_str()
                    .unwrap(),
                binance_config
                    .get("secret_key")
                    .unwrap()
                    .as_str()
                    .unwrap(),
            );
            let name = binance_config.get("name").unwrap().as_str().unwrap();
            if let Some(data) = binance_futures_api.get_open_orders(None).await {
                let v: Value = serde_json::from_str(&data).unwrap();
                let vec = v.as_array().unwrap();
                
                println!("获取到的账户挂单信息:{:?}, 名字{}", vec, name);
                if vec.len() == 0 {
                    if i != 0 {
                        let sender = format!("{}账号", name);
                        let content = format!("一分钟内没有新挂单");
                        wx_robot.send_text(&sender, &content).await;
                    }
                    i += 1;
    
                } else {
                    for a in vec {
                        let obj = a.as_object().unwrap();
                        let mut open_order_object: Map<String, Value> = Map::new();
                        let millis = obj.get("time").unwrap().as_i64().unwrap();
                        let datetime: DateTime<Utc> = DateTime::from_utc(
                            NaiveDateTime::from_timestamp_millis(millis).unwrap(),
                            Utc,
                        );
                        // info!("datetime: {}", datetime);
                        let time = format!("{}", datetime.format("%Y-%m-%d %H:%M:%S"));
                        
                        let symbol = obj.get("symbol").unwrap().as_str().unwrap();
                        let r#type = obj.get("type").unwrap().as_str().unwrap();
                        let mut type_value = "";
                        if r#type == "LIMIT" {
                            type_value = "限价单"
                        } else if r#type == "MARKET" {
                            type_value = "市价单"
                        } else if r#type == "STOP" {
                            type_value = "止损限价单"
                        } else if r#type == "STOP_MARKET" {
                            type_value = "止盈市价单"
                        } else if r#type == "TAKE_PROFIT" {
                            type_value = "止盈限价单"
                        } else if r#type == "TAKE_PROFIT_MARKET" {
                            type_value = "止盈市价单"
                        } else if r#type == "TRAILING_STOP_MARKET" {
                            type_value = "跟踪止损单" 
                        }
                        let side = obj.get("side").unwrap().as_str().unwrap();
                        let price = obj.get("price").unwrap().as_str().unwrap();
                        let orig_qty = obj.get("origQty").unwrap().as_str().unwrap();
                        let executed_qty = obj.get("executedQty").unwrap().as_str().unwrap();
                        let reduce_only = obj.get("reduceOnly").unwrap().as_bool().unwrap();
                        open_order_object.insert(String::from("time"), Value::from(time.clone()));
                        open_order_object.insert(String::from("name"), Value::from(name));
                        open_order_object.insert(String::from("symbol"), Value::from(symbol));
                        open_order_object.insert(String::from("type"), Value::from(type_value));
                        open_order_object.insert(String::from("side"), Value::from(side));
                        open_order_object.insert(String::from("price"), Value::from(price));
                        open_order_object.insert(String::from("orig_qty"), Value::from(orig_qty));
                        open_order_object.insert(String::from("executed_qty"), Value::from(executed_qty));
                        open_order_object.insert(String::from("reduce_only"), Value::from(reduce_only));
                        history_open_orders.push_back(Value::from(open_order_object));




                        // println!("11111{}", vec[a]);
                    }
                }
                // net_worth = notional_total/ori_fund;
                // net_worth_histories.push_back(Value::from(new_account_object));
            }

            let res = trade_mapper::TradeMapper::insert_open_orders(Vec::from(history_open_orders.clone()), name);
            println!("插入挂单数据是否成功{},", res); 
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
        let server_config = config.get("Server").unwrap();
        let symbols = config.get("Symbols").unwrap().as_array().unwrap();
        let key = config.get("Alarm").unwrap().get("webhook").unwrap().as_str().unwrap();
        // info!("获取key");
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
        
        
        
        let ssh_api = SshClient::new(
            server_config.get("host").unwrap().as_str().unwrap(),
            server_config.get("port").unwrap().as_str().unwrap(),
            server_config.get("username").unwrap().as_str().unwrap(),
            server_config.get("password").unwrap().as_str().unwrap(),
            server_config.get("root_path").unwrap().as_str().unwrap(),
            server_config.get("root_name").unwrap().as_str().unwrap(),
        );
        

        
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
