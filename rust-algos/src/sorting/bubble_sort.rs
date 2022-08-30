use clap::{App, ArgMatches};

pub const NAME: &str = "rust-algos:sorting/bubble-sort";
const ABOUT: &str = "https://algos.rs/sorting/bubble-sort.html";

pub fn sub_command<'help>() -> App<'help> {
    let sub_command = App::new(NAME)
        .about(ABOUT);
    return sub_command;
}

pub fn sub_handler(_matches :&ArgMatches){

    let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
    println!("原始值：{:?}", vec);
    bubble_sort(&mut vec);
    println!("排序后：{:?}", vec);

}

pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let size = arr.len();
    for i in 0..(size - 1) {
        // 标记当前循环是否发生元素交换
        let mut swapped = false;

        // 最后i个元素已经排好了顺序
        for j in 1..(size - i) {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
                swapped = true;
            }
        }

        // 如果当前循环没有发生元素交换，说明数组已经有序
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vec() {
        let mut empty_vec: Vec<String> = vec![];
        bubble_sort(&mut empty_vec);
        assert_eq!(empty_vec, Vec::<String>::new());
    }

    #[test]
    fn test_number_vec() {
        let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
    }

    #[test]
    fn test_string_vec() {
        let mut vec = vec![
            String::from("Bob"),
            String::from("David"),
            String::from("Carol"),
            String::from("Alice"),
        ];
        bubble_sort(&mut vec);
        assert_eq!(
            vec,
            vec![
                String::from("Alice"),
                String::from("Bob"),
                String::from("Carol"),
                String::from("David"),
            ]
        );
    }
}