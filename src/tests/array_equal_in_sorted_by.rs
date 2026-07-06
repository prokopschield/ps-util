use crate::Array;

#[test]
fn ascending_equal_in_sorted_by_variants_match_linear_equivalents() {
    let arr = [-5, -3, -3, 0, 1, 1, 1, 4, 9];
    let targets = [-10, -5, -4, -3, -2, 0, 1, 2, 9, 10];

    for target in targets {
        let expected_find = arr.iter().copied().find(|x| *x == target);
        let expected_find_index = arr.iter().copied().position(|x| x == target);
        let expected_find_last = arr.iter().copied().rfind(|x| *x == target);
        let expected_find_last_index = arr.iter().copied().rposition(|x| x == target);
        let expected_filter: Vec<i32> = arr.iter().copied().filter(|x| *x == target).collect();
        let expected_some = arr.contains(&target);
        let expected_none = !expected_some;
        let expected_every = arr.iter().all(|x| *x == target);

        assert_eq!(
            arr.find_equal_in_sorted_by(|x| x.cmp(&target)).copied(),
            expected_find
        );
        assert_eq!(
            arr.find_index_equal_in_sorted_by(|x| x.cmp(&target)),
            expected_find_index
        );
        assert_eq!(
            arr.find_last_equal_in_sorted_by(|x| x.cmp(&target))
                .copied(),
            expected_find_last
        );
        assert_eq!(
            arr.find_last_index_equal_in_sorted_by(|x| x.cmp(&target)),
            expected_find_last_index
        );
        assert_eq!(
            arr.filter_equal_in_sorted_by(|x| x.cmp(&target)),
            expected_filter
        );
        assert_eq!(
            arr.slice_equal_in_sorted_by(|x| x.cmp(&target)),
            expected_filter
        );
        assert_eq!(
            arr.some_equal_in_sorted_by(|x| x.cmp(&target)),
            expected_some
        );
        assert_eq!(
            arr.none_equal_in_sorted_by(|x| x.cmp(&target)),
            expected_none
        );
        assert_eq!(
            arr.every_equal_in_sorted_by(|x| x.cmp(&target)),
            expected_every
        );
    }
}

#[test]
fn descending_equal_in_sorted_by_variants_match_linear_equivalents() {
    let arr = [9, 7, 7, 4, 1, -2, -2, -5];
    let targets = [12, 9, 8, 7, 5, 4, 1, 0, -2, -5, -9];

    for target in targets {
        let expected_find = arr.iter().copied().find(|x| *x == target);
        let expected_find_index = arr.iter().copied().position(|x| x == target);
        let expected_find_last = arr.iter().copied().rfind(|x| *x == target);
        let expected_find_last_index = arr.iter().copied().rposition(|x| x == target);
        let expected_filter: Vec<i32> = arr.iter().copied().filter(|x| *x == target).collect();
        let expected_some = arr.contains(&target);
        let expected_none = !expected_some;
        let expected_every = arr.iter().all(|x| *x == target);

        assert_eq!(
            arr.find_equal_in_sorted_by(|x| target.cmp(x)).copied(),
            expected_find
        );
        assert_eq!(
            arr.find_index_equal_in_sorted_by(|x| target.cmp(x)),
            expected_find_index
        );
        assert_eq!(
            arr.find_last_equal_in_sorted_by(|x| target.cmp(x)).copied(),
            expected_find_last
        );
        assert_eq!(
            arr.find_last_index_equal_in_sorted_by(|x| target.cmp(x)),
            expected_find_last_index
        );
        assert_eq!(
            arr.filter_equal_in_sorted_by(|x| target.cmp(x)),
            expected_filter
        );
        assert_eq!(
            arr.slice_equal_in_sorted_by(|x| target.cmp(x)),
            expected_filter
        );
        assert_eq!(
            arr.some_equal_in_sorted_by(|x| target.cmp(x)),
            expected_some
        );
        assert_eq!(
            arr.none_equal_in_sorted_by(|x| target.cmp(x)),
            expected_none
        );
        assert_eq!(
            arr.every_equal_in_sorted_by(|x| target.cmp(x)),
            expected_every
        );
    }
}

#[test]
fn empty_array_returns_consistent_results() {
    let arr: [i32; 0] = [];
    let target = 42;

    assert_eq!(arr.find_equal_in_sorted_by(|x| x.cmp(&target)), None);
    assert_eq!(arr.find_index_equal_in_sorted_by(|x| x.cmp(&target)), None);
    assert_eq!(arr.find_last_equal_in_sorted_by(|x| x.cmp(&target)), None);
    assert_eq!(
        arr.find_last_index_equal_in_sorted_by(|x| x.cmp(&target)),
        None
    );
    assert_eq!(
        arr.filter_equal_in_sorted_by(|x| x.cmp(&target)),
        Vec::<i32>::new()
    );
    assert_eq!(arr.slice_equal_in_sorted_by(|x| x.cmp(&target)), &[]);
    assert!(!arr.some_equal_in_sorted_by(|x| x.cmp(&target)));
    assert!(arr.none_equal_in_sorted_by(|x| x.cmp(&target)));
    assert!(arr.every_equal_in_sorted_by(|x| x.cmp(&target)));
}

#[test]
fn single_element_array_handles_match_and_miss() {
    let arr = [5];

    assert_eq!(arr.find_equal_in_sorted_by(|x| x.cmp(&5)), Some(&5));
    assert_eq!(arr.find_index_equal_in_sorted_by(|x| x.cmp(&5)), Some(0));
    assert_eq!(arr.find_last_equal_in_sorted_by(|x| x.cmp(&5)), Some(&5));
    assert_eq!(
        arr.find_last_index_equal_in_sorted_by(|x| x.cmp(&5)),
        Some(0)
    );
    assert_eq!(arr.filter_equal_in_sorted_by(|x| x.cmp(&5)), vec![5]);
    assert_eq!(arr.slice_equal_in_sorted_by(|x| x.cmp(&5)), &[5]);
    assert!(arr.some_equal_in_sorted_by(|x| x.cmp(&5)));
    assert!(!arr.none_equal_in_sorted_by(|x| x.cmp(&5)));
    assert!(arr.every_equal_in_sorted_by(|x| x.cmp(&5)));

    assert_eq!(arr.find_equal_in_sorted_by(|x| x.cmp(&4)), None);
    assert_eq!(arr.find_index_equal_in_sorted_by(|x| x.cmp(&4)), None);
    assert_eq!(arr.find_last_equal_in_sorted_by(|x| x.cmp(&4)), None);
    assert_eq!(arr.find_last_index_equal_in_sorted_by(|x| x.cmp(&4)), None);
    assert_eq!(
        arr.filter_equal_in_sorted_by(|x| x.cmp(&4)),
        Vec::<i32>::new()
    );
    assert_eq!(arr.slice_equal_in_sorted_by(|x| x.cmp(&4)), &[]);
    assert!(!arr.some_equal_in_sorted_by(|x| x.cmp(&4)));
    assert!(arr.none_equal_in_sorted_by(|x| x.cmp(&4)));
    assert!(!arr.every_equal_in_sorted_by(|x| x.cmp(&4)));
}

#[test]
fn all_equal_array_covers_full_range_behavior() {
    let arr = [7, 7, 7, 7];

    assert_eq!(arr.find_equal_in_sorted_by(|x| x.cmp(&7)), Some(&7));
    assert_eq!(arr.find_index_equal_in_sorted_by(|x| x.cmp(&7)), Some(0));
    assert_eq!(arr.find_last_equal_in_sorted_by(|x| x.cmp(&7)), Some(&7));
    assert_eq!(
        arr.find_last_index_equal_in_sorted_by(|x| x.cmp(&7)),
        Some(3)
    );
    assert_eq!(
        arr.filter_equal_in_sorted_by(|x| x.cmp(&7)),
        vec![7, 7, 7, 7]
    );
    assert_eq!(arr.slice_equal_in_sorted_by(|x| x.cmp(&7)), &[7, 7, 7, 7]);
    assert!(arr.some_equal_in_sorted_by(|x| x.cmp(&7)));
    assert!(!arr.none_equal_in_sorted_by(|x| x.cmp(&7)));
    assert!(arr.every_equal_in_sorted_by(|x| x.cmp(&7)));

    assert_eq!(arr.find_equal_in_sorted_by(|x| x.cmp(&8)), None);
    assert_eq!(arr.find_index_equal_in_sorted_by(|x| x.cmp(&8)), None);
    assert_eq!(arr.find_last_equal_in_sorted_by(|x| x.cmp(&8)), None);
    assert_eq!(arr.find_last_index_equal_in_sorted_by(|x| x.cmp(&8)), None);
    assert_eq!(
        arr.filter_equal_in_sorted_by(|x| x.cmp(&8)),
        Vec::<i32>::new()
    );
    assert_eq!(arr.slice_equal_in_sorted_by(|x| x.cmp(&8)), &[]);
    assert!(!arr.some_equal_in_sorted_by(|x| x.cmp(&8)));
    assert!(arr.none_equal_in_sorted_by(|x| x.cmp(&8)));
    assert!(!arr.every_equal_in_sorted_by(|x| x.cmp(&8)));
}

#[test]
fn custom_struct_comparator_by_key_works_across_variants() {
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Item {
        key: i32,
        label: &'static str,
    }

    let arr = [
        Item { key: 1, label: "a" },
        Item { key: 2, label: "b" },
        Item { key: 2, label: "c" },
        Item { key: 4, label: "d" },
    ];

    assert_eq!(
        arr.find_equal_in_sorted_by(|item| item.key.cmp(&2))
            .map(|item| item.label),
        Some("b")
    );
    assert_eq!(
        arr.find_index_equal_in_sorted_by(|item| item.key.cmp(&2)),
        Some(1)
    );
    assert_eq!(
        arr.find_last_equal_in_sorted_by(|item| item.key.cmp(&2))
            .map(|item| item.label),
        Some("c")
    );
    assert_eq!(
        arr.find_last_index_equal_in_sorted_by(|item| item.key.cmp(&2)),
        Some(2)
    );
    assert_eq!(
        arr.filter_equal_in_sorted_by(|item| item.key.cmp(&2)),
        vec![Item { key: 2, label: "b" }, Item { key: 2, label: "c" }]
    );
    assert_eq!(
        arr.slice_equal_in_sorted_by(|item| item.key.cmp(&2)),
        &arr[1..3]
    );
    assert!(arr.some_equal_in_sorted_by(|item| item.key.cmp(&4)));
    assert!(arr.none_equal_in_sorted_by(|item| item.key.cmp(&3)));
    assert!(!arr.every_equal_in_sorted_by(|item| item.key.cmp(&2)));
}
