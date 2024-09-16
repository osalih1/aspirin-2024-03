#[derive(PartialEq, Clone, Copy, Debug)]
enum ClassYear {
    Senior,
    Junior,
    Sophomore,
    FirstYear,
}

struct Student {
    name: &'static str,
    class_year: ClassYear,
    gpa: f32,
}

const OLIN_STUDENTS: [Student; 8] = [
    Student {
        name: "Alice",
        class_year: ClassYear::Senior,
        gpa: 3.9,
    },
    Student {
        name: "Foo",
        class_year: ClassYear::Sophomore,
        gpa: 2.3,
    },
    Student {
        name: "Bar",
        class_year: ClassYear::Junior,
        gpa: 3.9,
    },
    Student {
        name: "Ralph",
        class_year: ClassYear::Senior,
        gpa: 3.1,
    },
    Student {
        name: "Ayush",
        class_year: ClassYear::Senior,
        gpa: 0.0,
    },
    Student {
        name: "Anna",
        class_year: ClassYear::FirstYear,
        gpa: 4.0,
    },
    Student {
        name: "Hannah",
        class_year: ClassYear::FirstYear,
        gpa: 4.0,
    },
    Student {
        name: "Lorin",
        class_year: ClassYear::Junior,
        gpa: 3.6,
    },
];

fn get_average_gpa() -> f32 {
    let mut total_gpa = 0.0;
    let mut count = 0;

    for student in OLIN_STUDENTS.iter() {
        if student.class_year != ClassYear::FirstYear {
            total_gpa += student.gpa;
            count += 1;
        }
    }

    if count == 0 {
        0.0
    } else {
        total_gpa / count as f32
    }
}

fn get_num_excel_students_for_class(class_year: ClassYear) -> u32 {
    let average_gpa = get_average_gpa();
    let mut count = 0;

    // Count students in the given class_year with GPA above the average GPA
    for student in OLIN_STUDENTS.iter() {
        if student.class_year == class_year && student.gpa > average_gpa {
            count += 1;
        }
    }

    count
}

fn get_best_class() -> ClassYear {
    // Get the number of excelling students for each class year
    let senior_count = get_num_excel_students_for_class(ClassYear::Senior);
    let junior_count = get_num_excel_students_for_class(ClassYear::Junior);
    let sophomore_count = get_num_excel_students_for_class(ClassYear::Sophomore);
    let first_year_count = get_num_excel_students_for_class(ClassYear::FirstYear);

    // Determine the class with the highest number of excelling students
    let mut best_class = ClassYear::Senior;
    let mut max_count = senior_count;

    if junior_count > max_count {
        best_class = ClassYear::Junior;
        max_count = junior_count;
    }
    if sophomore_count > max_count {
        best_class = ClassYear::Sophomore;
        max_count = sophomore_count;
    }
    if first_year_count > max_count {
        best_class = ClassYear::FirstYear;
    }

    best_class
}

// Do not modify below here
#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use crate::university::{
        get_average_gpa, get_best_class, get_num_excel_students_for_class, ClassYear,
    };

    #[test]
    fn test_get_average_gpa() {
        assert!(approx_eq!(f32, get_average_gpa(), 2.8))
    }

    #[test]
    fn test_get_num_excel_students_for_class() {
        assert_eq!(get_num_excel_students_for_class(ClassYear::Sophomore), 0);
        assert_eq!(get_num_excel_students_for_class(ClassYear::Junior), 2);
        assert_eq!(get_num_excel_students_for_class(ClassYear::Senior), 2);
    }

    #[test]
    fn test_get_best_class() {
        assert_eq!(get_best_class(), ClassYear::Senior);
    }
}
