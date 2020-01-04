fn main() {
    let mut x = vec![
        r#"fn main() {"#,
        r#"    let mut x = vec!["#,
        r#"    ];"#,
        r#"    let mut tmp: Vec<String> = Vec::new();"#,
        r#"    for line in x.iter() {"#,
        r#"        let mut line_re = String::new();"#,
        r#"        line_re.push_str("        r#\"");"#,
        r#"        line_re.push_str(line);"#,
        r#"        line_re.push('"');"#,
        r#"        line_re.push('#');"#,
        r#"        line_re.push(',');"#,
        r#"        tmp.push(line_re);"#,
        r#"    }"#,
        r#"    let mut index = 0;"#,
        r#"    while index < tmp.len() {"#,
        r#"        x.insert(2+index, &tmp[index][..]);"#,
        r#"        index = index + 1;"#,
        r#"    }"#,
        r#"    for line in x.iter() {"#,
        r#"        println!("{}", line);"#,
        r#"    }"#,
        r#"}"#,
    ];
    let mut tmp: Vec<String> = Vec::new();
    for line in x.iter() {
        let mut line_re = String::new();
        line_re.push_str("        r#\"");
        line_re.push_str(line);
        line_re.push('"');
        line_re.push('#');
        line_re.push(',');
        tmp.push(line_re);
    }
    let mut index = 0;
    while index < tmp.len() {
        x.insert(2+index, &tmp[index][..]);
        index = index + 1;
    }
    for line in x.iter() {
        println!("{}", line);
    }
}
