use serde::{Deserialize, Serialize, Deserializer};
use std::collections::{HashMap, HashSet};
use std::fs;
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
struct ServerConfig {
    upstream: Vec<HashSet<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    // #[serde(flatten)] // 使用 flatten 将动态键展平到 Vec<ServerConfig>
    // #[serde(deserialize_with = "deserialize_servers")]
    servers: Vec<ServerConfig>,
}

/// 自定义反序列化：将 HashMap<String, HashMap<String, Vec<String>>> 转为 Vec<ServerConfig>
// fn deserialize_servers<'de, D>(deserializer: D) -> Result<Vec<ServerConfig>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let raw_data: HashMap<String, HashMap<String, Vec<String>>> =
//         HashMap::deserialize(deserializer)?;

//     let servers = raw_data
//         .into_values()
//         .map(|map| ServerConfig {
//             upstream: map
//                 .into_iter() // 使用 into_iter() 获取键值对
//                 .map(|(key, neighbors)| {
//                     let mut set: HashSet<String> = neighbors.into_iter().collect(); // 将值转为 HashSet
//                     set.insert(key); // 将键名插入 HashSet
//                     set
//                 })
//                 .collect(),
//         })
//         .collect();

//     Ok(servers)
// }

// fn load_config_json() -> Config {
//     let config_content = fs::read_to_string("district.json").expect("Unable to read config file");
//     let config: Config = serde_json::from_str(&config_content).expect("Unable to parse config file");
//     config
// }

// fn load_config_json() -> Config {
//     let config_content = fs::read_to_string("district.json").expect("Unable to read config file");
//     let raw_data: Value = serde_json::from_str(&config_content).expect("Unable to parse config file");
//     println!("{:?}",raw_data);
//     let servers = if let Value::Object(map) = raw_data {
//         map.into_iter()
//             .map(|(_key, value)| {
//                 let mut upstream = Vec::new();
//                 if let Value::Object(inner_map) = value {
//                     for (key, neighbors) in inner_map {
//                         if let Value::Array(arr) = neighbors {
//                             let mut set = HashSet::new();
//                             set.insert(key);
//                             for v in arr {
//                                 if let Value::String(s) = v {
//                                     set.insert(s);
//                                 }
//                             }
//                             upstream.push(set);
//                         }
//                     }
//                 }
//                 ServerConfig { upstream }
//             })
//             .collect()
//     } else {
//         Vec::new()
//     };

//     Config { servers }
// }

// fn save_config_as_json(config: &Config) {
//     // 将 Config 序列化为 JSON 字符串
//     let json_string = serde_json::to_string_pretty(config).expect("Failed to serialize config");
//     // 将 JSON 字符串写入文件
//     fs::write("config.json", json_string).expect("Unable to write config file");
// }
fn parse_config(content: &str) -> Config {
    let mut servers = Vec::new();
    let mut current_group = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        // 检查是否是 "key": ["value1", "value2", ...] 格式
        if line.contains(':') && line.contains('[') && line.contains(']') {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().trim_matches(|c| c == '"' || c == ',');
                let value = parts[1].trim().trim_matches(|c| c == ',');
                if value.starts_with('[') && value.ends_with(']') {
                    // 解析值数组
                    let neighbors: Vec<String> = serde_json::from_str(value).unwrap_or_default();
                    let mut set = HashSet::new();
                    set.insert(key.to_string()); // 键名加入 HashSet
                    for neighbor in neighbors {
                        set.insert(neighbor); // 值加入 HashSet
                    }
                    current_group.push(set);
                }
            }
        } else if !current_group.is_empty() && (line == "}" || line == "},") {
            // 遇到组结束标记，且当前组不为空，则保存当前组
            servers.push(ServerConfig {
                upstream: current_group,
            });
            current_group = Vec::new(); // 开始新组
        }
    }

    // 处理最后一个组（如果文件末尾没有明确结束符）
    if !current_group.is_empty() {
        servers.push(ServerConfig {
            upstream: current_group,
        });
    }

    Config { servers }
}


// 合并有交集的 HashSet，不考虑顺序
fn merge_sets(server: &ServerConfig) -> Vec<HashSet<String>> {
    let mut sets = server.upstream.clone();
    let mut merged = Vec::new();
    let mut visited = vec![false; sets.len()];

    for i in 0..sets.len() {
        if visited[i] {
            continue; // 已合并的跳过
        }

        let mut current_set = sets[i].clone();
        visited[i] = true;

        // 检查所有其他集合，合并有交集的
        loop {
            let mut merged_any = false;
            for j in 0..sets.len() {
                if !visited[j] {
                    let intersection: HashSet<String> = current_set
                        .intersection(&sets[j])
                        .cloned()
                        .collect();
                    if !intersection.is_empty() {
                        // 有交集，合并
                        current_set.extend(sets[j].iter().cloned());
                        visited[j] = true;
                        merged_any = true;
                    }
                }
            }
            if !merged_any {
                break; // 没有新的合并，退出循环
            }
        }

        merged.push(current_set);
    }

    merged
}

pub fn count_provinces() -> String {
    let config_content = fs::read_to_string("district.json").expect("Unable to read config file");
    let config = parse_config(&config_content);

    let mut result = "".to_string();
    for mut sub in config.servers.into_iter() {
        // println!("{:?}",sub);
        sub.upstream = merge_sets(&sub);
        // println!("{:?}",sub.upstream);
        // println!("{:?}",sub.upstream.len());
        result.push_str(&format!("{},", sub.upstream.len()));

    }
    // println!("{:?}",result);
    result.trim_end_matches(',').to_string()
}
