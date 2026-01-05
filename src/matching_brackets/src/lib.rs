fn get_matching_bracket(bracket: char) -> char {
    match bracket {
        '{' => '}',
        '}' => '{',
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        _ => ' ',
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut res = true;
    let filtered_brackets = string
        .trim()
        .chars()
        .filter(|c| match c {
            '{' => true,
            '}' => true,
            '(' => true,
            ')' => true,
            '[' => true,
            ']' => true,
            _ => false,
        })
        .collect::<String>();
    for (n, c) in filtered_brackets.chars().enumerate() {
        let matching_bracket = get_matching_bracket(c);
        println!("{}, {}, {}", c, matching_bracket, n);
        if !string.contains(matching_bracket) {
            res = false;
            break;
        }
    }
    println!("{}", res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paired_square_brackets() {
        assert!(brackets_are_balanced("[]"));
    }

    #[test]
    fn empty_string() {
        assert!(brackets_are_balanced(""));
    }

    #[test]
    fn unpaired_brackets() {
        assert!(!brackets_are_balanced("[["));
    }

    #[test]
    fn wrong_ordered_brackets() {
        assert!(!brackets_are_balanced("}{"));
    }

    #[test]
    fn wrong_closing_bracket() {
        assert!(!brackets_are_balanced("{]"));
    }

    #[test]
    fn paired_with_whitespace() {
        assert!(brackets_are_balanced("{ }"));
    }

    #[test]
    fn partially_paired_brackets() {
        assert!(!brackets_are_balanced("{[])"));
    }

    #[test]
    fn simple_nested_brackets() {
        assert!(brackets_are_balanced("{[]}"));
    }

    #[test]
    fn several_paired_brackets() {
        assert!(brackets_are_balanced("{}[]"));
    }

    #[test]
    fn paired_and_nested_brackets() {
        assert!(brackets_are_balanced("([{}({}[])])"));
    }

    #[test]
    fn unopened_closing_brackets() {
        assert!(!brackets_are_balanced("{[)][]}"));
    }

    #[test]
    fn unpaired_and_nested_brackets() {
        assert!(!brackets_are_balanced("([{])"));
    }

    #[test]
    fn paired_and_wrong_nested_brackets() {
        assert!(!brackets_are_balanced("[({]})"));
    }

    #[test]
    #[ignore]
    fn paired_and_wrong_nested_brackets_but_innermost_are_correct() {
        assert!(!brackets_are_balanced("[({}])"));
    }

    #[test]
    #[ignore]
    fn paired_and_incomplete_brackets() {
        assert!(!brackets_are_balanced("{}["));
    }

    #[test]
    #[ignore]
    fn too_many_closing_brackets() {
        assert!(!brackets_are_balanced("[]]"));
    }

    #[test]
    #[ignore]
    fn early_unexpected_brackets() {
        assert!(!brackets_are_balanced(")()"));
    }

    #[test]
    #[ignore]
    fn early_mismatched_brackets() {
        assert!(!brackets_are_balanced("{)()"));
    }

    #[test]
    #[ignore]
    fn math_expression() {
        assert!(brackets_are_balanced("(((185 + 223.85) * 15) - 543)/2"));
    }

    #[test]
    #[ignore]
    fn complex_latex_expression() {
        assert!(brackets_are_balanced(
            "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \\end{array}\\right)"
        ));
    }
}
