use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    vec,
};

enum Namespace {
    User,
    Mount,
    Cgroup,
    IPC,
    Time,
    UTS,
    PID,
    Network,
}

#[derive(Debug, PartialEq)]
struct Mapping {
    container_id: u32,
    host_id: u32,
    range: u32,
}

fn get_uid_mappings() -> Vec<Mapping> {
    let lines = read_lines("/etc/subuid");
    parse_mappings(&get_user_id(), lines)
}

fn get_gid_mappings() -> Vec<Mapping> {
    let lines = read_lines("/etc/subgid");
    parse_mappings(&get_group_id(), lines)
}

fn get_user_id() -> u32 {
    users::get_current_uid()
}

fn get_group_id() -> u32 {
    users::get_current_gid()
}

fn parse_mapping(host_id: &u32, map_str: &str) -> Option<Mapping> {
    let parts = map_str.split(":");
    let len = parts.clone().count();
    if len != 3 {
        return None;
    }
    let parts: Vec<&str> = parts.collect();
    let host_id_part = parts.get(0).unwrap();
    if host_id_part.parse::<u32>().unwrap() == *host_id || *host_id_part == "ALL" {
        return Some(Mapping {
            container_id: parts.get(1).unwrap().parse().unwrap(),
            host_id: *host_id,
            range: parts.get(2).unwrap().parse().unwrap(),
        });
    }
    return None;
}

fn parse_mappings<T: BufRead>(host_id: &u32, lines: Lines<T>) -> Vec<Mapping> {
    // /etc/sub{uid,gid} is of the following format
    // USERNAME_OR_GROUP:START_UID_IN_USERNAMESPACE:RANGE
    let mut mappings = vec![];
    for line in lines {
        if line.is_err() {
            continue;
        }
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        let mapping = match parse_mapping(host_id, line.as_str()) {
            Some(mapping) => mapping,
            None => continue,
        };
        mappings.push(mapping);
    }
    mappings
}

fn read_lines(path: &str) -> Lines<BufReader<File>> {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::{get_user_id, parse_mapping, Mapping};

    #[test]
    fn test_parse_mapping_correct_pattern() {
        let uid = get_user_id();
        let subuid_content = format!("{}:100000:1000", &uid);
        let actual = parse_mapping(&uid, &subuid_content).unwrap();
        let expected = Mapping {
            host_id: uid,
            range: 1000,
            container_id: 100000,
        };
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_parse_mapping_wrong_pattern() {
        let uid = get_user_id();
        let subuid_content = "yeeah";
        let actual = parse_mapping(&uid, &subuid_content);
        let expected = None;
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_parse_mapping_host_id_doesnt_match() {
        let uid = 9999;
        let subuid_content = format!("{}:100000:1000", &uid);
        let actual = parse_mapping(&1000, &subuid_content);
        let expected = None;
        assert_eq!(expected, actual)
    }
}
