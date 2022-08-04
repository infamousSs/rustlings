// quiz3.rs
// This quiz tests:
// - Generics
// - Traits
// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to "A+"
// to show that your changes allow alphabetical grades.

// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
    pub use_grade_as_string: bool,
}

trait Grade {
    fn as_number(&self) -> String;
    fn as_string(&self) -> String;
}

impl Grade for f32 {
    fn as_number(&self) -> String {
        self.to_string()
    }

    fn as_string(&self) -> String {
        match self {
            -1.0..=1.0 => "F-".into(),
            1.0..=1.5 => "F".into(),
            1.5..=2.0 => "D".into(),
            2.0..=2.5 => "C".into(),
            2.5..=3.0 => "C+".into(),
            3.0..=3.5 => "B".into(),
            4.0..=4.5 => "B+".into(),
            4.5..=5.0 => "A".into(),
            5.0..=5.5 => "A+".into(),
            x if x > &5.5 => "A+".into(),
            x => "Invalid".into(),
        }
    }
}

impl ReportCard {
    pub fn print(&self) -> String {
        if self.use_grade_as_string {
            format!(
                "{} ({}) - achieved a grade of {}",
                &self.student_name,
                &self.student_age,
                &self.grade.as_string()
            )
        } else {
            format!(
                "{} ({}) - achieved a grade of {}",
                &self.student_name,
                &self.student_age,
                &self.grade.as_number()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
            use_grade_as_string: false,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: 5.5,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
            use_grade_as_string: true,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
