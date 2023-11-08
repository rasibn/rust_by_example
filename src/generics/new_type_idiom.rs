struct Years(i64);
struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    /// truncates partial years
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

pub(crate) fn main() {
    let age = Years(5);
    let age_days = age.to_days();

    let years = Years(42);
    let years_as_primitive_1: i64 = years.0;
    let Years(years_as_primitive_2) = years;

    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // Compile time guarentee that the right type of value is supplied to a program
    // println!("Old enough {}", old_enough(&age_days)); -> Error!
}
