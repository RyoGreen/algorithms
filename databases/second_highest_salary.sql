select Max(salary) as SecondHighestSalary from employee where salary < (select Max(salary) from employee);
