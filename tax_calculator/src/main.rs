use std::io;

fn main() {
    println!("Welcome to Tax Calculator!!\n\n> No tax is calculated for first LKR 100,000 of salary income\n> Each subsequent increase of LKR 50,000 in salary income, tax rate is incremented at 6% blocks to a maximum of 36%\n> For additional income, the first LKR 100,000 is taxed at 10%, and it is increased by 10% for each subsequent LKR 50,000 to a maximum 40%\n\n");
    
    loop {
        println!("Enter the salary income: ");
        
        let mut salary = String::new();
        io::stdin()
            .read_line(&mut salary)
            .expect("Error occurred while reading the input!");
        let salary: f32 = match salary.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a correct value!\n");
                continue;
            }
        };

        if salary <= 0.0 {
            println!("Exiting...");
            break;
        }

        println!("Enter any additional income: ");
        
        let mut additional = String::new();
        io::stdin()
            .read_line(&mut additional)
            .expect("Error occurred while reading the input!");
        let additional: f32 = match additional.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a correct value!\n");
                continue;
            }
        };

        let salary_tax = calculate_annual_salary_tax(salary);
        let additional_tax = calculate_annual_additional_tax(additional);
        println!("Annual Salary Income: LKR {salary}\nAnnual Additional Income: LKR {additional}\nAnnual Total Income: LKR {}\n", salary + additional);
        println!("Annual Salary Income Tax Amount: LKR {salary_tax}\nAnnual Additional Income Tax Amount: LKR {additional_tax}\nAnnual Total Income Tax Amount: LKR {}\n", salary_tax + additional_tax);
    }
}

fn calculate_annual_additional_tax(mut additional: f32) -> f32 {
    let mut rate = 10.0;
    let mut tax_amount = 0.0;
    
    if additional < 100000.0 {
        return additional * rate / 100.0;
    
    } else {
        tax_amount += 100000.0 * rate / 100.0;
        additional -= 100000.0;
    }

    rate += 10.0;
   
    while additional > 0.0 {
        additional -= 50000.0;

        if additional > 0.0 {
            tax_amount += 50000.0 * rate / 100.0;
        } else {
            tax_amount += (additional + 50000.0) * rate / 100.0;
        }

        if rate < 40.0 {
            rate += 10.0;
        }
    }

    return tax_amount;
}

fn calculate_annual_salary_tax(mut salary: f32) -> f32 {
    if salary < 100000.0 {
        return 0.0;
    }

    salary -= 100000.0;
    let mut rate = 6.0;
    let mut tax_amount = 0.0;

    while salary > 0.0 {
        salary -= 50000.0;

        if salary > 0.0 {
            tax_amount += 50000.0 * rate / 100.0;
        } else {
            tax_amount += (salary + 50000.0) * rate / 100.0;
        }

        if rate < 36.0 {
            rate += 6.0;
        }
    }

    return tax_amount;
}
