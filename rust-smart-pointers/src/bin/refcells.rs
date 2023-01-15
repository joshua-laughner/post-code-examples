use std::collections::HashMap;
use std::cell::RefCell;

fn main() {
    let mut data = HashMap::new();
    data.insert("baseline", vec![1.0, 2.0, 3.0]);
    data.insert("test_1", vec![0.9, 2.1, 3.0]);
    data.insert("test_2", vec![1.1, 2.2, 2.9]);

    // This won't compile because we can't borrow the test 1 vector
    // mutably at the same time as we have an immutable reference to
    // the baseline data. And if we used `get_mut` to retrieve the baseline
    // data, we'd have two mutable borrows of `data`, which isn't
    // allowed either.
    
    /* if let Some(baseline) = data.get("baseline") {
        if let Some(test_1) = data.get_mut("test_1") {
            for (i, v) in baseline.iter().enumerate() {
                test_1[i] /= v;
            }
        }
    } */

    // One solution is to remove the vectors we need and put them back 
    // in the hash map when done.

    if let Some(mut test_1) = data.remove("test_1") {
        if let Some(baseline) = data.get("baseline") {
            for (i, v) in baseline.iter().enumerate() {
                test_1[i] /= v;
            }
        }
        data.insert("test_1", test_1);
    }

    // Otherwise, this is a case where RefCell can help.
    let mut cell_data = HashMap::new();
    cell_data.insert("baseline", RefCell::new(vec![10.0, 20.0, 30.0]));
    cell_data.insert("test_1", RefCell::new(vec![9.5,  21.2, 32.1]));
    cell_data.insert("test_2", RefCell::new(vec![11.9, 19.5, 29.6]));

    if let Some(baseline_cell) = cell_data.get("baseline") {
        // The downside is this part *could* panic if we got our borrows wrong.
        let baseline = baseline_cell.borrow();
        if let Some(test_1_cell) = cell_data.get("test_1") {
            let mut test_1 = test_1_cell.borrow_mut();
            for (i, v) in baseline.iter().enumerate() {
                test_1[i] /= v;
            }
        }

        // We could be safer if we did a try_borrow instead, though this requires
        // either an additional if-let block or some chaining.
        let test_2_opt = cell_data.get("test_2")
            .and_then(|c| c.try_borrow_mut().ok());
        if let Some(mut test_2) = test_2_opt {
            for (i, v) in baseline.iter().enumerate() {
                test_2[i] /= v;
            }
        }
    }

    dbg!(data);
    dbg!(cell_data);
}
