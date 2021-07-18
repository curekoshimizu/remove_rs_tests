use std::io::BufRead;

fn main() {
    remove_tests(std::io::stdin().lock());
}

struct TestModeContext {
    count: usize,
    is_test_mode: bool,
}

impl TestModeContext {
    fn new() -> Self {
        TestModeContext {
            count: 0,
            is_test_mode: false,
        }
    }
}

fn remove_tests<R: BufRead>(reader: R) {
    let mut context = TestModeContext::new();

    for line in reader.lines() {
        let line = line.unwrap();

        match line.trim() {
            "#[cfg(test)]" => {}
            "mod tests {" => {
                context.is_test_mode = true;
            }
            _ => {
                if !context.is_test_mode {
                    println!("{}", line);
                }
            }
        }

        if context.is_test_mode {
            if line.contains("{") {
                context.count += 1;
            } else if line.contains("}") {
                context.count -= 1;
                if context.count == 0 {
                    context.is_test_mode = false;
                }
            }
        }
    }
}
